use soroban_sdk::Env;

use crate::{types::PoolTotals, DataKey};

pub fn get_pool_totals(env: &Env) -> Option<PoolTotals> {
    env.storage().instance().get(&DataKey::PoolTotals)
}

pub fn set_pool_totals(env: &Env, totals: &PoolTotals) {
    env.storage().instance().set(&DataKey::PoolTotals, totals);
}
