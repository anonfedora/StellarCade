#![no_std]

mod storage;
mod types;

use soroban_sdk::{contract, contractimpl, contracttype, contracterror, Address, Env, Symbol};

pub use types::{BracketHealthSummary, PromotionCutoff};

const BUMP_AMOUNT: u32 = 518_400;
const LIFETIME_THRESHOLD: u32 = BUMP_AMOUNT / 2;

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Admin,
    Bracket(u32),
}

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    NotInitialized = 1,
    BracketNotFound = 2,
}

#[contract]
pub struct ChallengeLadder;

#[contractimpl]
impl ChallengeLadder {
    pub fn init(env: Env, admin: Address) -> Result<(), Error> {
        if env.storage().instance().has(&DataKey::Admin) {
            return Err(Error::NotInitialized);
        }
        env.storage().instance().set(&DataKey::Admin, &admin);
        Ok(())
    }

    /// Returns a summary of bracket health including player counts and activity levels.
    pub fn bracket_health_summary(env: Env, bracket_id: u32) -> BracketHealthSummary {
        // For now, return empty state - this would be populated with actual bracket data
        BracketHealthSummary {
            bracket_id,
            exists: false,
            player_count: 0,
            active_games: 0,
            promotion_threshold: 0,
        }
    }

    /// Returns the promotion cutoff details for a bracket.
    pub fn promotion_cutoff(env: Env, bracket_id: u32) -> PromotionCutoff {
        // For now, return empty state - this would be populated with actual cutoff data
        PromotionCutoff {
            bracket_id,
            exists: false,
            cutoff_score: 0,
            cutoff_rank: 0,
            next_promotion_time: 0,
        }
    }
}

#[cfg(test)]
mod test;