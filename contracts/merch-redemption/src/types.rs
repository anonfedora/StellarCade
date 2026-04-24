use soroban_sdk::contracttype;

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum StockPressureLevel {
    None = 0,
    Low = 1,
    Medium = 2,
    High = 3,
}

/// Snapshot of claim window details for merchandise.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ClaimWindowSnapshot {
    pub item_id: Symbol,
    /// True when the item_id exists.
    pub exists: bool,
    /// Start time of the claim window.
    pub start_time: u64,
    /// End time of the claim window.
    pub end_time: u64,
    /// Total items available for claiming.
    pub total_available: u32,
    /// Number of items already claimed.
    pub claimed_count: u32,
}

/// Stock pressure information for merchandise.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StockPressure {
    pub item_id: Symbol,
    /// True when the item_id exists.
    pub exists: bool,
    /// Remaining stock available.
    pub remaining_stock: u32,
    /// Current stock pressure level.
    pub pressure_level: StockPressureLevel,
}