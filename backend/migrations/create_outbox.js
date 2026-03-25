/**
 * @param { import("knex").Knex } knex
 * @returns { Promise<void> }
 */
exports.up = function(knex) {
  return knex.schema.createTable('outbox', function(table) {
    table.uuid('id').primary().defaultTo(knex.raw('gen_random_uuid()'));
    table.string('transaction_xdr').notNullable();
    table.text('error_message').nullable();
    table.string('error_code').nullable();
    table.jsonb('result_codes').nullable();
    table.integer('attempts').notNullable().defaultTo(0);
    table.timestamp('next_retry_at').nullable();
    table.timestamp('created_at').notNullable().defaultTo(knex.fn.now());
    table.timestamp('updated_at').notNullable().defaultTo(knex.fn.now());
    table.string('status').notNullable().defaultTo('pending'); // pending, processing, completed, failed
    
    // Indexes for efficient querying
    table.index(['status', 'next_retry_at']);
    table.index(['created_at']);
    table.index(['attempts']);
    
    // Add unique constraint to prevent duplicate processing
    table.string('processing_lock').nullable().unique();
  });
};

/**
 * @param { import("knex").Knex } knex
 * @returns { Promise<void> }
 */
exports.down = function(knex) {
  return knex.schema.dropTableIfExists('outbox');
};
