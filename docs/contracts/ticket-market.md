# ticket-market

## Public Methods

### `init`
```rust
pub fn init(env: Env, admin: Address, token: Address) -> Result<(), Error>
```

#### Parameters

| Name | Type |
|------|------|
| `env` | `Env` |
| `admin` | `Address` |
| `token` | `Address` |

#### Return Type

`Result<(), Error>`

### `list_ticket`
Post a new ticket listing. `expires_at_ledger` must be strictly greater than the current ledger.

```rust
pub fn list_ticket(env: Env, seller: Address, game_id: Symbol, price: i128, expires_at_ledger: u32) -> Result<u64, Error>
```

#### Parameters

| Name | Type |
|------|------|
| `env` | `Env` |
| `seller` | `Address` |
| `game_id` | `Symbol` |
| `price` | `i128` |
| `expires_at_ledger` | `u32` |

#### Return Type

`Result<u64, Error>`

### `cancel_listing`
Cancel a listing (seller only).

```rust
pub fn cancel_listing(env: Env, seller: Address, listing_id: u64) -> Result<(), Error>
```

#### Parameters

| Name | Type |
|------|------|
| `env` | `Env` |
| `seller` | `Address` |
| `listing_id` | `u64` |

#### Return Type

`Result<(), Error>`

### `fill_listing`
Mark a listing as sold (admin-only — called after successful payment).

```rust
pub fn fill_listing(env: Env, listing_id: u64) -> Result<(), Error>
```

#### Parameters

| Name | Type |
|------|------|
| `env` | `Env` |
| `listing_id` | `u64` |

#### Return Type

`Result<(), Error>`

### `orderbook_summary`
Returns a live summary of the orderbook, including best/worst ask and total volume across all non-expired active listings.

```rust
pub fn orderbook_summary(env: Env) -> OrderbookSummary
```

#### Parameters

| Name | Type |
|------|------|
| `env` | `Env` |

#### Return Type

`OrderbookSummary`

### `listing_expiry`
Returns expiry details for a single listing. Returns a not-found struct when the listing_id is unknown.

```rust
pub fn listing_expiry(env: Env, listing_id: u64) -> ListingExpiry
```

#### Parameters

| Name | Type |
|------|------|
| `env` | `Env` |
| `listing_id` | `u64` |

#### Return Type

`ListingExpiry`

