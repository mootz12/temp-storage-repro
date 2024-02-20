use soroban_sdk::{Address, Env};

const ONE_DAY_LEDGERS: u32 = 17280; // assumes 5 seconds per ledger on average
const LEDGER_THRESHOLD_SHARED: u32 = 7 * ONE_DAY_LEDGERS;
const LEDGER_BUMP_SHARED: u32 = 8 * ONE_DAY_LEDGERS;

/// Bump the instance lifetime by the defined amount
pub fn extend_instance(e: &Env) {
    e.storage()
        .instance()
        .extend_ttl(LEDGER_THRESHOLD_SHARED, LEDGER_BUMP_SHARED);
}

pub fn get_temp_storage(e: &Env, address: &Address) -> i128 {
    e.storage().temporary().get(address).unwrap_or(-1)
}

pub fn set_temp_storage(e: &Env, address: &Address, amount: i128, extend_to: u32) {
    e.storage()
        .temporary()
        .set::<Address, i128>(address, &amount);
    e.storage().temporary().extend_ttl(address, extend_to, extend_to)
}