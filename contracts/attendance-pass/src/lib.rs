#![no_std]

mod storage;
mod types;

use soroban_sdk::{contract, contractimpl, contracttype, Address, Env};

pub use types::{ExpiryBand, HolderCoverageSummary, PassRecord, PassStatus};

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Admin,
    Pass(u64),
    TotalHolders,
    ActiveHolders,
    ExpiredPasses,
    TotalIssued,
}

#[contract]
pub struct AttendancePass;

#[contractimpl]
impl AttendancePass {
    pub fn init(env: Env, admin: Address) {
        if env.storage().instance().has(&DataKey::Admin) {
            panic!("Already initialized");
        }
        admin.require_auth();
        env.storage().instance().set(&DataKey::Admin, &admin);
    }

    pub fn issue_pass(
        env: Env,
        admin: Address,
        pass_id: u64,
        holder: Address,
        expires_at: u64,
    ) {
        admin.require_auth();
        assert!(expires_at > env.ledger().timestamp(), "Expiry must be in future");

        let record = PassRecord {
            pass_id,
            holder: holder.clone(),
            issued_at: env.ledger().timestamp(),
            expires_at,
            active: true,
        };

        storage::set_pass(&env, &record);
        storage::increment_total_holders(&env);
        storage::increment_active_holders(&env);
        storage::increment_total_issued(&env);
    }

    pub fn expire_pass(env: Env, admin: Address, pass_id: u64) {
        admin.require_auth();

        let mut record = storage::get_pass(&env, pass_id).expect("Pass not found");
        assert!(record.active, "Already expired");

        record.active = false;
        storage::set_pass(&env, &record);
        storage::decrement_active_holders(&env);
        storage::increment_expired_passes(&env);
    }

    pub fn holder_coverage_summary(env: Env) -> HolderCoverageSummary {
        let configured = env.storage().instance().has(&DataKey::Admin);

        HolderCoverageSummary {
            configured,
            total_holders: storage::get_total_holders(&env),
            active_holders: storage::get_active_holders(&env),
            expired_passes: storage::get_expired_passes(&env),
            total_issued: storage::get_total_issued(&env),
        }
    }

    pub fn expiry_band(env: Env, pass_id: u64) -> ExpiryBand {
        let now = env.ledger().timestamp();
        let configured = env.storage().instance().has(&DataKey::Admin);

        let Some(record) = storage::get_pass(&env, pass_id) else {
            return ExpiryBand {
                pass_id,
                configured,
                exists: false,
                status: if configured {
                    PassStatus::Active
                } else {
                    PassStatus::NotConfigured
                },
                issued_at: 0,
                expires_at: 0,
                now,
            };
        };

        let status = if !record.active {
            PassStatus::Expired
        } else if now >= record.expires_at {
            PassStatus::Expired
        } else {
            PassStatus::Active
        };

        ExpiryBand {
            pass_id,
            configured,
            exists: true,
            status,
            issued_at: record.issued_at,
            expires_at: record.expires_at,
            now,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::testutils::{Address as _, Ledger};

    #[test]
    fn test_init() {
        let env = Env::default();
        let admin = Address::random(&env);
        AttendancePass::init(env.clone(), admin);
    }

    #[test]
    fn test_issue_and_expire_pass() {
        let env = Env::default();
        env.ledger().set_timestamp(1000);

        let admin = Address::random(&env);
        let holder = Address::random(&env);

        AttendancePass::init(env.clone(), admin.clone());
        AttendancePass::issue_pass(env.clone(), admin.clone(), 1, holder.clone(), 2000);

        let summary = AttendancePass::holder_coverage_summary(env.clone());
        assert_eq!(summary.total_holders, 1);
        assert_eq!(summary.active_holders, 1);

        AttendancePass::expire_pass(env.clone(), admin, 1);

        let summary = AttendancePass::holder_coverage_summary(env);
        assert_eq!(summary.expired_passes, 1);
        assert_eq!(summary.active_holders, 0);
    }

    #[test]
    fn test_expiry_band_missing() {
        let env = Env::default();
        let admin = Address::random(&env);
        AttendancePass::init(env.clone(), admin);

        let band = AttendancePass::expiry_band(env, 999);
        assert_eq!(band.exists, false);
        assert_eq!(band.configured, true);
    }
}
