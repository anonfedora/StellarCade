# reserve-auction

## Public Methods

### `init`
Initialise the contract and set the admin. Must be called exactly once.

```rust
pub fn init(env: Env, admin: Address) -> Result<(), Error>
```

### `set_paused`
Pause or unpause auction creation, bidding, and settlement globally.

```rust
pub fn set_paused(env: Env, paused: bool) -> Result<(), Error>
```

### `create_auction`
Create a reserve-price auction with an explicit live window.

```rust
pub fn create_auction(
    env: Env,
    seller: Address,
    asset_label: String,
    reserve_price: i128,
    start_ledger: u32,
    end_ledger: u32,
) -> Result<u64, Error>
```

### `place_bid`
Place a bid on a live auction. Bids must be strictly greater than the current highest bid.

```rust
pub fn place_bid(env: Env, bidder: Address, auction_id: u64, amount: i128) -> Result<(), Error>
```

### `settle_auction`
Settle an ended auction. If the reserve was not met, the returned winner is `None` and seller proceeds are `0`.

```rust
pub fn settle_auction(
    env: Env,
    seller: Address,
    auction_id: u64,
) -> Result<SettlementOutcome, Error>
```

### `auction_snapshot`
Return a stable snapshot for one auction id.

Fallback behavior:
- unknown auction ids return `exists = false`
- seller and asset metadata return `None`
- numeric fields return `0`
- `phase` returns `Missing`

```rust
pub fn auction_snapshot(env: Env, auction_id: u64) -> ReserveAuctionSnapshot
```

### `seller_summary`
Return aggregate reserve-auction state for a seller.

Fallback behavior:
- sellers with no created auctions return `exists = false`
- counts return `0`
- `highest_open_bid` returns `0`

```rust
pub fn seller_summary(env: Env, seller: Address) -> SellerAuctionSummary
```
