#![cfg(test)]
use soroban_sdk::{testutils::Address as _, Address, Env, Symbol};

use super::*;

#[test]
fn test_claim_window_snapshot_empty() {
    let env = Env::default();
    let contract_id = env.register_contract(None, MerchRedemption);
    let client = MerchRedemptionClient::new(&env, &contract_id);
    let item_id = Symbol::new(&env, "item1");

    let snapshot = client.claim_window_snapshot(&item_id);
    assert_eq!(snapshot.item_id, item_id);
    assert!(!snapshot.exists);
    assert_eq!(snapshot.start_time, 0);
    assert_eq!(snapshot.end_time, 0);
    assert_eq!(snapshot.total_available, 0);
    assert_eq!(snapshot.claimed_count, 0);
}

#[test]
fn test_stock_pressure_empty() {
    let env = Env::default();
    let contract_id = env.register_contract(None, MerchRedemption);
    let client = MerchRedemptionClient::new(&env, &contract_id);
    let item_id = Symbol::new(&env, "item1");

    let pressure = client.stock_pressure(&item_id);
    assert_eq!(pressure.item_id, item_id);
    assert!(!pressure.exists);
    assert_eq!(pressure.remaining_stock, 0);
    assert_eq!(pressure.pressure_level as u32, StockPressureLevel::None as u32);
}