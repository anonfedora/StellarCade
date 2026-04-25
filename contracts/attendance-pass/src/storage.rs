use crate::types::PassRecord;
use soroban_sdk::Env;

use crate::lib::DataKey;

pub fn set_pass(env: &Env, record: &PassRecord) {
    env.storage()
        .persistent()
        .set(&DataKey::Pass(record.pass_id), record);
}

pub fn get_pass(env: &Env, pass_id: u64) -> Option<PassRecord> {
    env.storage()
        .persistent()
        .get(&DataKey::Pass(pass_id))
        .unwrap_or(None)
}

pub fn increment_total_holders(env: &Env) {
    let current: u32 = env
        .storage()
        .instance()
        .get(&DataKey::TotalHolders)
        .unwrap_or(0u32);
    env.storage()
        .instance()
        .set(&DataKey::TotalHolders, &(current + 1));
}

pub fn get_total_holders(env: &Env) -> u32 {
    env.storage()
        .instance()
        .get(&DataKey::TotalHolders)
        .unwrap_or(0u32)
}

pub fn increment_active_holders(env: &Env) {
    let current: u32 = env
        .storage()
        .instance()
        .get(&DataKey::ActiveHolders)
        .unwrap_or(0u32);
    env.storage()
        .instance()
        .set(&DataKey::ActiveHolders, &(current + 1));
}

pub fn decrement_active_holders(env: &Env) {
    let current: u32 = env
        .storage()
        .instance()
        .get(&DataKey::ActiveHolders)
        .unwrap_or(0u32);
    if current > 0 {
        env.storage()
            .instance()
            .set(&DataKey::ActiveHolders, &(current - 1));
    }
}

pub fn get_active_holders(env: &Env) -> u32 {
    env.storage()
        .instance()
        .get(&DataKey::ActiveHolders)
        .unwrap_or(0u32)
}

pub fn increment_expired_passes(env: &Env) {
    let current: u32 = env
        .storage()
        .instance()
        .get(&DataKey::ExpiredPasses)
        .unwrap_or(0u32);
    env.storage()
        .instance()
        .set(&DataKey::ExpiredPasses, &(current + 1));
}

pub fn get_expired_passes(env: &Env) -> u32 {
    env.storage()
        .instance()
        .get(&DataKey::ExpiredPasses)
        .unwrap_or(0u32)
}

pub fn increment_total_issued(env: &Env) {
    let current: u64 = env
        .storage()
        .instance()
        .get(&DataKey::TotalIssued)
        .unwrap_or(0u64);
    env.storage()
        .instance()
        .set(&DataKey::TotalIssued, &(current + 1));
}

pub fn get_total_issued(env: &Env) -> u64 {
    env.storage()
        .instance()
        .get(&DataKey::TotalIssued)
        .unwrap_or(0u64)
}
