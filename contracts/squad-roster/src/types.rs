use soroban_sdk::{contracttype, Address, Symbol};

/// A single squad member slot.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RosterSlot {
    /// The role this slot is designed for (e.g. "captain", "support").
    pub role: Symbol,
    /// The player occupying this slot, if any.
    pub player: Option<Address>,
    /// Whether the slot is currently locked (cannot be changed until unlocked).
    pub locked: bool,
}

/// Aggregated lineup readiness summary returned by `lineup_readiness_summary`.
///
/// Zero-state: all counts 0, `ready` false when no slots are registered.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LineupReadinessSummary {
    /// Total number of slots in the roster.
    pub total_slots: u32,
    /// Slots that have a player assigned.
    pub filled_slots: u32,
    /// Slots with no player assigned.
    pub vacant_slots: u32,
    /// Slots that are locked (frozen).
    pub locked_slots: u32,
    /// True only when every slot is filled and none are locked.
    pub ready: bool,
}

/// Vacancy information for a single role, returned by `vacancy_for_role`.
///
/// Zero-state: `exists` false when the role has no registered slot.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RoleVacancy {
    /// Whether a slot for this role exists in the roster.
    pub exists: bool,
    /// Whether the slot is vacant (no player assigned).
    pub vacant: bool,
    /// The current occupant, if any.
    pub player: Option<Address>,
}
