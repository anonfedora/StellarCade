#![no_std]

mod storage;
mod types;

use soroban_sdk::{contract, contractimpl, contracttype, Env};

pub use types::{BatchProgressSummary, BatchRecord, BatchStatus, RetryableFailure};

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Admin,
    Batch(u64),
    TotalBatches,
    CompletedBatches,
    PendingBatches,
    TotalDistributed,
    FailedBatches,
}

#[contract]
pub struct FanoutDistributor;

#[contractimpl]
impl FanoutDistributor {
    pub fn init(env: Env, admin: soroban_sdk::Address) {
        if env.storage().instance().has(&DataKey::Admin) {
            panic!("Already initialized");
        }
        admin.require_auth();
        env.storage().instance().set(&DataKey::Admin, &admin);
    }

    pub fn create_batch(
        env: Env,
        admin: soroban_sdk::Address,
        batch_id: u64,
        total_amount: i128,
        recipient_count: u32,
    ) {
        admin.require_auth();
        assert!(total_amount > 0, "Total amount must be positive");
        assert!(recipient_count > 0, "Recipient count must be positive");

        let record = BatchRecord {
            batch_id,
            total_amount,
            distributed_amount: 0,
            recipient_count,
            completed: false,
            failed: false,
        };

        storage::set_batch(&env, &record);
        storage::increment_total_batches(&env);
        storage::increment_pending_batches(&env);
    }

    pub fn distribute(
        env: Env,
        admin: soroban_sdk::Address,
        batch_id: u64,
        amount: i128,
    ) -> i128 {
        admin.require_auth();

        let mut record = storage::get_batch(&env, batch_id).expect("Batch not found");
        assert!(amount > 0, "Amount must be positive");
        assert!(
            record.distributed_amount + amount <= record.total_amount,
            "Exceeds batch amount"
        );

        record.distributed_amount = record.distributed_amount + amount;
        storage::set_batch(&env, &record);
        storage::add_total_distributed(&env, amount);

        amount
    }

    pub fn complete_batch(env: Env, admin: soroban_sdk::Address, batch_id: u64) {
        admin.require_auth();

        let mut record = storage::get_batch(&env, batch_id).expect("Batch not found");
        assert!(!record.completed, "Already completed");

        record.completed = true;
        storage::set_batch(&env, &record);
        storage::decrement_pending_batches(&env);
        storage::increment_completed_batches(&env);
    }

    pub fn mark_failed(env: Env, admin: soroban_sdk::Address, batch_id: u64) {
        admin.require_auth();

        let mut record = storage::get_batch(&env, batch_id).expect("Batch not found");
        assert!(!record.failed, "Already marked failed");

        record.failed = true;
        storage::set_batch(&env, &record);
        storage::increment_failed_batches(&env);
    }

    pub fn batch_progress_summary(env: Env) -> BatchProgressSummary {
        let configured = env.storage().instance().has(&DataKey::Admin);

        BatchProgressSummary {
            configured,
            total_batches: storage::get_total_batches(&env),
            completed_batches: storage::get_completed_batches(&env),
            pending_batches: storage::get_pending_batches(&env),
            total_distributed: storage::get_total_distributed(&env),
            failed_batches: storage::get_failed_batches(&env),
        }
    }

    pub fn retryable_failure(env: Env, batch_id: u64) -> RetryableFailure {
        let now = env.ledger().timestamp();
        let configured = env.storage().instance().has(&DataKey::Admin);

        let Some(record) = storage::get_batch(&env, batch_id) else {
            return RetryableFailure {
                batch_id,
                configured,
                exists: false,
                status: if configured {
                    BatchStatus::Pending
                } else {
                    BatchStatus::NotConfigured
                },
                failed: false,
                total_amount: 0,
                distributed_amount: 0,
                now,
            };
        };

        let status = if record.completed {
            BatchStatus::Completed
        } else if record.failed {
            BatchStatus::Pending
        } else {
            BatchStatus::InProgress
        };

        RetryableFailure {
            batch_id,
            configured,
            exists: true,
            status,
            failed: record.failed,
            total_amount: record.total_amount,
            distributed_amount: record.distributed_amount,
            now,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::testutils::Address as _;

    #[test]
    fn test_init() {
        let env = Env::default();
        let admin = soroban_sdk::Address::random(&env);
        FanoutDistributor::init(env.clone(), admin);
    }

    #[test]
    fn test_batch_lifecycle() {
        let env = Env::default();
        let admin = soroban_sdk::Address::random(&env);

        FanoutDistributor::init(env.clone(), admin.clone());
        FanoutDistributor::create_batch(env.clone(), admin.clone(), 1, 1000, 5);

        let summary = FanoutDistributor::batch_progress_summary(env.clone());
        assert_eq!(summary.total_batches, 1);
        assert_eq!(summary.pending_batches, 1);

        FanoutDistributor::distribute(env.clone(), admin.clone(), 1, 500);
        let summary = FanoutDistributor::batch_progress_summary(env.clone());
        assert_eq!(summary.total_distributed, 500);

        FanoutDistributor::complete_batch(env.clone(), admin, 1);
        let summary = FanoutDistributor::batch_progress_summary(env);
        assert_eq!(summary.completed_batches, 1);
        assert_eq!(summary.pending_batches, 0);
    }

    #[test]
    fn test_retryable_failure_missing() {
        let env = Env::default();
        let admin = soroban_sdk::Address::random(&env);
        FanoutDistributor::init(env.clone(), admin);

        let failure = FanoutDistributor::retryable_failure(env, 999);
        assert_eq!(failure.exists, false);
        assert_eq!(failure.configured, true);
    }
}
