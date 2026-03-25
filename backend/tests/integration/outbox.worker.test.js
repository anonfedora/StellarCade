const request = require('supertest');
const app = require('../../src/server');
const Outbox = require('../../src/models/Outbox.model');
const stellarService = require('../../src/services/stellar.service');
const outboxWorker = require('../../src/services/outbox.worker');
const knex = require('../../src/config/database');

describe('Outbox Worker Integration', () => {
  beforeEach(async () => {
    // Clean up outbox table
    await knex('outbox').del();
  });

  afterAll(async () => {
    await knex.destroy();
  });

  describe('Success after retry', () => {
    it('should successfully process transaction after initial failure', async () => {
      // Mock stellar service to fail first time, succeed second time
      let callCount = 0;
      const originalSubmit = stellarService.submitTransaction;
      stellarService.submitTransaction = jest.fn().mockImplementation(() => {
        callCount++;
        if (callCount === 1) {
          return Promise.resolve({
            status: 'error',
            errorCode: stellarService.STELLAR_ERRORS.TIMEOUT,
            errorMessage: 'Request timed out',
            resultCodes: null,
          });
        }
        return Promise.resolve({
          status: 'success',
          hash: 'test-hash',
          ledger: 12345,
          successful: true,
          envelopeXDR: 'test-envelope',
          resultXDR: 'test-result',
        });
      });

      // Add transaction to outbox
      const result = await stellarService.submitTransactionAsync('test-xdr');
      expect(result.status).toBe('queued');

      // Process pending entries
      const processResult = await outboxWorker.processPendingEntries();
      expect(processResult.processed).toBe(1);
      expect(processResult.successful).toBe(1);

      // Verify transaction was processed successfully
      const outboxEntry = await Outbox.findById(result.outboxId);
      expect(outboxEntry.status).toBe('completed');
      expect(outboxEntry.attempts).toBe(2);

      // Restore original function
      stellarService.submitTransaction = originalSubmit;
    });
  });

  describe('Permanent failure', () => {
    it('should mark transaction as permanently failed after max attempts', async () => {
      // Mock stellar service to always fail with non-retryable error
      const originalSubmit = stellarService.submitTransaction;
      stellarService.submitTransaction = jest.fn().mockResolvedValue({
        status: 'error',
        errorCode: stellarService.STELLAR_ERRORS.TX_FAILED,
        errorMessage: 'Transaction failed',
        resultCodes: { transaction: 'tx_failed' },
      });

      // Add transaction to outbox
      const result = await stellarService.submitTransactionAsync('test-xdr');
      expect(result.status).toBe('queued');

      // Simulate 5 failed attempts
      for (let i = 0; i < 5; i++) {
        await outboxWorker.processPendingEntries();
      }

      // Verify transaction is marked as permanently failed
      const outboxEntry = await Outbox.findById(result.outboxId);
      expect(outboxEntry.status).toBe('failed');
      expect(outboxEntry.attempts).toBe(5);

      // Restore original function
      stellarService.submitTransaction = originalSubmit;
    });
  });

  describe('Duplicate processing protection', () => {
    it('should prevent duplicate processing of same transaction', async () => {
      // Mock successful stellar service
      const originalSubmit = stellarService.submitTransaction;
      stellarService.submitTransaction = jest.fn().mockResolvedValue({
        status: 'success',
        hash: 'test-hash',
        ledger: 12345,
        successful: true,
        envelopeXDR: 'test-envelope',
        resultXDR: 'test-result',
      });

      // Add transaction to outbox
      const result = await stellarService.submitTransactionAsync('test-xdr');
      expect(result.status).toBe('queued');

      // Process same entry twice concurrently
      const [process1, process2] = await Promise.all([
        outboxWorker.processPendingEntries(),
        outboxWorker.processPendingEntries(),
      ]);

      // Only one should succeed
      expect(process1.successful + process2.successful).toBe(1);

      // Verify transaction was processed successfully only once
      const outboxEntry = await Outbox.findById(result.outboxId);
      expect(outboxEntry.status).toBe('completed');

      // Restore original function
      stellarService.submitTransaction = originalSubmit;
    });
  });

  describe('Exponential backoff', () => {
    it('should use exponential backoff for retries', async () => {
      const delays = [];
      const originalSetTimeout = global.setTimeout;
      global.setTimeout = jest.fn().mockImplementation((callback, delay) => {
        delays.push(delay);
        return originalSetTimeout(callback, 0); // Execute immediately for test
      });

      // Test backoff calculation
      expect(outboxWorker._calculateBackoff(0)).toBeLessThan(2000);
      expect(outboxWorker._calculateBackoff(1)).toBeLessThan(4000);
      expect(outboxWorker._calculateBackoff(2)).toBeLessThan(8000);

      // Restore original setTimeout
      global.setTimeout = originalSetTimeout;
    });
  });
});
