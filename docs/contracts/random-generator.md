# random-generator

A pending randomness request registered by an authorized game contract.

## Public Methods

### `init`
Initialize the contract. May only be called once.  `oracle` is the sole address permitted to call `fulfill_random`. It is expected to be a backend service that pre-commits server seeds off-chain before each game round begins.

```rust
pub fn init(env: Env, admin: Address, oracle: Address) -> Result<(), Error>
```

#### Parameters

| Name | Type |
|------|------|
| `env` | `Env` |
| `admin` | `Address` |
| `oracle` | `Address` |

#### Return Type

`Result<(), Error>`

### `authorize`
Add a game contract to the caller whitelist. Admin only.

```rust
pub fn authorize(env: Env, admin: Address, caller: Address) -> Result<(), Error>
```

#### Parameters

| Name | Type |
|------|------|
| `env` | `Env` |
| `admin` | `Address` |
| `caller` | `Address` |

#### Return Type

`Result<(), Error>`

### `revoke`
Remove a game contract from the caller whitelist. Admin only.

```rust
pub fn revoke(env: Env, admin: Address, caller: Address) -> Result<(), Error>
```

#### Parameters

| Name | Type |
|------|------|
| `env` | `Env` |
| `admin` | `Address` |
| `caller` | `Address` |

#### Return Type

`Result<(), Error>`

### `request_random`
Submit a randomness request. Only whitelisted callers may call this.  `max` must be >= 2. The fulfilled result will be in `[0, max - 1]`. `request_id` must be globally unique — rejected if a pending or fulfilled entry for the same ID already exists.

```rust
pub fn request_random(env: Env, caller: Address, request_id: u64, max: u64) -> Result<(), Error>
```

#### Parameters

| Name | Type |
|------|------|
| `env` | `Env` |
| `caller` | `Address` |
| `request_id` | `u64` |
| `max` | `u64` |

#### Return Type

`Result<(), Error>`

### `fulfill_random`
Fulfill a pending randomness request. Oracle only.  The result is derived as: `sha256(server_seed || request_id_be_bytes)[0..8] % max`  Both `server_seed` and `result` are persisted for on-chain verification. Fairness holds when the oracle published `sha256(server_seed)` before the corresponding `request_random` call was submitted.

```rust
pub fn fulfill_random(env: Env, oracle: Address, request_id: u64, server_seed: BytesN<32>) -> Result<(), Error>
```

#### Parameters

| Name | Type |
|------|------|
| `env` | `Env` |
| `oracle` | `Address` |
| `request_id` | `u64` |
| `server_seed` | `BytesN<32>` |

#### Return Type

`Result<(), Error>`

### `set_entropy_metadata`
Set entropy source version metadata. Admin only.  Metadata is informational and does not affect randomness output.

```rust
pub fn set_entropy_metadata(env: Env, admin: Address, metadata: EntropySourceMetadata) -> Result<(), Error>
```

#### Parameters

| Name | Type |
|------|------|
| `env` | `Env` |
| `admin` | `Address` |
| `metadata` | `EntropySourceMetadata` |

#### Return Type

`Result<(), Error>`

### `get_entropy_metadata`
Read the current entropy source version metadata.

```rust
pub fn get_entropy_metadata(env: Env) -> Result<EntropySourceMetadata, Error>
```

#### Parameters

| Name | Type |
|------|------|
| `env` | `Env` |

#### Return Type

`Result<EntropySourceMetadata, Error>`

### `get_result`
Return the fulfilled result for a `request_id`.  Returns `RequestNotFound` if the request is still pending or never existed.

```rust
pub fn get_result(env: Env, request_id: u64) -> Result<FulfilledEntry, Error>
```

#### Parameters

| Name | Type |
|------|------|
| `env` | `Env` |
| `request_id` | `u64` |

#### Return Type

`Result<FulfilledEntry, Error>`

### `get_request_status`
Return the lifecycle status for a request id.

```rust
pub fn get_request_status(env: Env, request_id: u64) -> Result<RequestStatus, Error>
```

#### Parameters

| Name | Type |
|------|------|
| `env` | `Env` |
| `request_id` | `u64` |

#### Return Type

`Result<RequestStatus, Error>`

### `get_config`
Return a stable snapshot of the generator's operational configuration. This is a read-only, side-effect-free accessor designed for operational tooling. The config shape is extensible for future generator policy fields.

```rust
pub fn get_config(env: Env) -> Result<GeneratorConfig, Error>
```

#### Parameters

| Name | Type |
|------|------|
| `env` | `Env` |

#### Return Type

`Result<GeneratorConfig, Error>`

#### GeneratorConfig Structure

| Field | Type | Description |
|-------|------|-------------|
| `admin` | `Address` | Address of the contract administrator |
| `oracle` | `Address` | Address of the oracle authorized to fulfill randomness requests |
| `persistent_ttl` | `u32` | Persistent storage TTL in ledgers (518,400 ≈ 30 days) |
| `min_max_bound` | `u64` | Minimum allowed value for the `max` parameter (always 2) |
| `entropy_version` | `Option<String>` | Current entropy metadata version string, if set |

### `get_requester_summary`
Return a per-requester sequencing summary showing their request history. This is a deterministic, side-effect-free read operation. For missing requesters (never authorized or no requests), returns a summary with all counts at zero.

```rust
pub fn get_requester_summary(env: Env, caller: Address) -> RequesterSummary
```

#### Parameters

| Name | Type |
|------|------|
| `env` | `Env` |
| `caller` | `Address` |

#### Return Type

`RequesterSummary`

#### RequesterSummary Structure

| Field | Type | Description |
|-------|------|-------------|
| `caller` | `Address` | The caller address this summary is for |
| `total_requests` | `u64` | Total number of requests made by this caller (pending + fulfilled) |
| `pending_count` | `u64` | Number of requests currently pending fulfillment |
| `fulfilled_count` | `u64` | Number of requests that have been fulfilled |

#### Missing-Requester Behavior

For callers that have never made requests or are not in the whitelist, the function returns a `RequesterSummary` with all count fields set to zero. This allows operational tooling to inspect any address without requiring error handling or prior knowledge of which callers have interacted with the contract.


