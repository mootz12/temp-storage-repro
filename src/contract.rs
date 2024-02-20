use soroban_sdk::{contract, contractimpl, Address, Env};

use crate::storage;

#[contract]
pub struct TempStorageTest;

#[contractimpl]
impl TempStorageTest {
    pub fn set_temp(e: Env, address: Address, amount: i128, ledgers: u32) {
        storage::set_temp_storage(&e, &address, amount, ledgers)
    }

    pub fn get_temp(e: Env, address: Address) -> i128 {
        storage::get_temp_storage(&e, &address)
    }
}
