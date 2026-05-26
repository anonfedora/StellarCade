# creator-escrow

## Public Methods

### `init`
Initialise the contract and set the admin. Must be called exactly once.

```rust
pub fn init(env: Env, admin: Address) -> Result<(), Error>
```

### `set_paused`
Pause or unpause funding and release workflows contract-wide.

```rust
pub fn set_paused(env: Env, paused: bool) -> Result<(), Error>
```

### `configure_creator`
Register or update creator escrow configuration. Unknown creators become configured after this call.

```rust
pub fn configure_creator(
    env: Env,
    creator: Address,
    payout_token: Address,
    beneficiary: Address,
    release_delay_ledgers: u32,
) -> Result<(), Error>
```

### `set_creator_paused`
Pause or unpause a single creator configuration without affecting the rest of the contract.

```rust
pub fn set_creator_paused(env: Env, creator: Address, paused: bool) -> Result<(), Error>
```

### `fund_escrow`
Create a new escrow entry for a configured creator. The entry becomes releasable after `release_delay_ledgers`.

```rust
pub fn fund_escrow(env: Env, creator: Address, amount: i128) -> Result<u64, Error>
```

### `release_available`
Release every unreleased entry whose `releasable_at_ledger` has passed for the specified creator.

```rust
pub fn release_available(env: Env, creator: Address) -> Result<i128, Error>
```

### `creator_summary`
Return the current creator escrow state in a consumer-friendly shape.

Fallback behavior:
- Unknown creators return `exists = false`
- `payout_token` and `beneficiary` return `None`
- all counters return `0`
- `next_entry_id` returns the stored next id, which is `0` for unconfigured creators

```rust
pub fn creator_summary(env: Env, creator: Address) -> CreatorEscrowSummary
```

### `escrow_entry`
Return one escrow entry by creator and entry id.

Fallback behavior:
- Unknown creators or unknown entry ids return `exists = false`
- amount and ledger fields return `0`
- `releasable_now` returns `false`

```rust
pub fn escrow_entry(env: Env, creator: Address, entry_id: u64) -> CreatorEscrowEntryView
```
