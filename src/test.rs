use std::println;

#[cfg(test)]

use soroban_sdk::{Address, Env, testutils::{Ledger, LedgerInfo, Address as _}};
use crate::{TempStorageTest, TempStorageTestClient};

#[test]
fn test_temp_storage() {
    let e = Env::default();
    e.mock_all_auths();

    e.ledger().set(LedgerInfo {
        timestamp: 123456,
        protocol_version: 20,
        sequence_number: 100,
        network_id: Default::default(),
        base_reserve: 10,
        min_temp_entry_ttl: 100,
        min_persistent_entry_ttl: 10000,
        max_entry_ttl: 9999999,
    });

    let contract = e.register_contract(None, TempStorageTest {});
    let contract_client = TempStorageTestClient::new(&e, &contract);

    let user = Address::generate(&e);

    assert_eq!(contract_client.get_temp(&user), -1);

    contract_client.set_temp(&user, &12345, &120);

    assert_eq!(contract_client.get_temp(&user), 12345);

    e.ledger().set(LedgerInfo {
        timestamp: 223456,
        protocol_version: 20,
        sequence_number: 100 + 500,
        network_id: Default::default(),
        base_reserve: 10,
        min_temp_entry_ttl: 100,
        min_persistent_entry_ttl: 10000,
        max_entry_ttl: 9999999,
    });

    // entry should be expired

    // should be -1
    println!("temp: {}", contract_client.get_temp(&user));

    // should work
    contract_client.set_temp(&user, &222, &120);
}