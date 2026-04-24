#![no_std]

mod storage;
mod types;

use soroban_sdk::{contract, contractimpl, contracttype, contracterror, Address, Env, Symbol};

pub use types::{ClaimWindowSnapshot, StockPressure};

const BUMP_AMOUNT: u32 = 518_400;
const LIFETIME_THRESHOLD: u32 = BUMP_AMOUNT / 2;

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Admin,
    ClaimWindow(Symbol),
}

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    NotInitialized = 1,
    ClaimWindowNotFound = 2,
}

#[contract]
pub struct MerchRedemption;

#[contractimpl]
impl MerchRedemption {
    pub fn init(env: Env, admin: Address) -> Result<(), Error> {
        if env.storage().instance().has(&DataKey::Admin) {
            return Err(Error::NotInitialized);
        }
        env.storage().instance().set(&DataKey::Admin, &admin);
        Ok(())
    }

    /// Returns a snapshot of the claim window for merchandise.
    pub fn claim_window_snapshot(env: Env, item_id: Symbol) -> ClaimWindowSnapshot {
        // For now, return empty state - this would be populated with actual claim window data
        ClaimWindowSnapshot {
            item_id,
            exists: false,
            start_time: 0,
            end_time: 0,
            total_available: 0,
            claimed_count: 0,
        }
    }

    /// Returns stock pressure information for merchandise.
    pub fn stock_pressure(env: Env, item_id: Symbol) -> StockPressure {
        // For now, return empty state - this would be populated with actual stock pressure data
        StockPressure {
            item_id,
            exists: false,
            remaining_stock: 0,
            pressure_level: StockPressureLevel::None,
        }
    }
}