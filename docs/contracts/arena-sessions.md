# arena-sessions

## Public Methods

### `init`
Initialise the contract and set the admin. Must be called exactly once.

```rust
pub fn init(env: Env, admin: Address) -> Result<(), Error>
```

### `set_paused`
Pause or unpause new session starts and player-controlled completion flows.

```rust
pub fn set_paused(env: Env, paused: bool) -> Result<(), Error>
```

### `start_session`
Start a new arena session for a player. If a previously active session has already expired, it is swept before the new session is created.

```rust
pub fn start_session(
    env: Env,
    player: Address,
    arena_id: u32,
    stake_amount: i128,
    duration_ledgers: u32,
) -> Result<u64, Error>
```

### `complete_session`
Complete an active session before it expires.

```rust
pub fn complete_session(env: Env, player: Address, session_id: u64) -> Result<(), Error>
```

### `expire_session`
Persist an expired active session into terminal expired state so player summary counts stay in sync with storage.

```rust
pub fn expire_session(env: Env, session_id: u64) -> Result<(), Error>
```

### `session_status`
Return the latest derived view for a session id.

Fallback behavior:
- unknown session ids return `exists = false`
- `player` returns `None`
- numeric fields return `0`
- `state` returns `Missing`

```rust
pub fn session_status(env: Env, session_id: u64) -> ArenaSessionView
```

### `player_summary`
Return aggregate arena-session state for a player.

Fallback behavior:
- players with no sessions return `exists = false`
- all counters return `0`
- `active_session_id` returns `None`
- `next_session_id` returns the global next id, which is `0` before any sessions exist

```rust
pub fn player_summary(env: Env, player: Address) -> PlayerArenaSessionSummary
```
