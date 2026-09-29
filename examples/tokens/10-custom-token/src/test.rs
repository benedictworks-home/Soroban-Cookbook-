#![cfg(test)]
#![allow(deprecated)]

extern crate std;

use soroban_sdk::{
    symbol_short,
    testutils::{Address as _, Ledger},
    Address, Env, IntoVal, Symbol, Vec,
};

use crate::CustomTokenContractClient;

fn setup() -> (Env, Address, Address, Address, CustomTokenContractClient<'static>) {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let user1 = Address::generate(&env);
    let user2 = Address::generate(&env);

    let signers: Vec<Address> = Vec::from_array(&env, [admin.clone(), user1.clone()]);

    let contract_id = env.register_contract(None, crate::CustomTokenContract);
    let client = CustomTokenContractClient::new(&env, &contract_id);

    client.initialize(
        &admin,
        &String::from_str(&env, "CustomToken"),
        &symbol_short!("CT"),
        &7u32,
        &1000i128,
        &2u32,
        &signers,
    );

    (env, admin, user1, user2, client)
}

#[test]
fn test_initialize() {
    let (env, admin, _user1, _user2, client) = setup();

    assert_eq!(client.name(), String::from_str(&env, "CustomToken"));
    assert_eq!(client.symbol(), symbol_short!("CT"));
    assert_eq!(client.decimals(), 7);
    assert_eq!(client.total_supply(), 1000);
    assert_eq!(client.balance(&admin), 1000);
    assert!(!client.is_paused());
}

#[test]
fn test_initialize_twice_fails() {
    let (_env, admin, _user1, _user2, client) = setup();

    let result = client.try_initialize(
        &admin,
        &String::from_str(&std::vec!["Duplicate"].into()),
        &symbol_short!("DUP"),
        &7u32,
        &500i128,
        &2u32,
        &Vec::from_array(&_env, [admin.clone(), Address::generate(&_env)]),
    );
    assert_eq!(result.err().unwrap().to_string(), "AlreadyInitialized");
}

#[test]
fn test_transfer() {
    let (_env, admin, user1, _user2, client) = setup();

    client.transfer(&admin, &user1, &300);

    assert_eq!(client.balance(&admin), 700);
    assert_eq!(client.balance(&user1), 300);
}

#[test]
fn test_transfer_insufficient_balance() {
    let (_env, admin, user1, _user2, client) = setup();

    let result = client.try_transfer(&admin, &user1, &2000);
    assert_eq!(result.err().unwrap().to_string(), "InsufficientBalance");
}

#[test]
fn test_transfer_invalid_amount() {
    let (_env, admin, user1, _user2, client) = setup();

    let result = client.try_transfer(&admin, &user1, &0);
    assert_eq!(result.err().unwrap().to_string(), "InvalidAmount");

    let result = client.try_transfer(&admin, &user1, &(-100));
    assert_eq!(result.err().unwrap().to_string(), "InvalidAmount");
}

#[test]
fn test_approve_and_transfer_from() {
    let (_env, admin, user1, user2, client) = setup();

    client.approve(&admin, &user1, &500);

    assert_eq!(client.allowance(&admin, &user1), 500);

    client.transfer_from(&user1, &admin, &user2, &200);

    assert_eq!(client.balance(&admin), 800);
    assert_eq!(client.balance(&user2), 200);
    assert_eq!(client.allowance(&admin, &user1), 300);
}

#[test]
fn test_transfer_from_exceeds_allowance() {
    let (_env, admin, user1, user2, client) = setup();

    client.approve(&admin, &user1, &100);
    let result = client.try_transfer_from(&user1, &admin, &user2, &200);
    assert_eq!(result.err().unwrap().to_string(), "AllowanceExceeded");
}

#[test]
fn test_mint() {
    let (_env, admin, user1, _user2, client) = setup();

    client.mint(&admin, &user1, &500);

    assert_eq!(client.total_supply(), 1500);
    assert_eq!(client.balance(&user1), 500);
}

#[test]
fn test_mint_unauthorized() {
    let (_env, _admin, user1, _user2, client) = setup();

    let result = client.try_mint(&user1, &user1, &500);
    assert_eq!(result.err().unwrap().to_string(), "Unauthorized");
}

#[test]
fn test_burn() {
    let (_env, admin, _user1, _user2, client) = setup();

    client.burn(&admin, &300);

    assert_eq!(client.total_supply(), 700);
    assert_eq!(client.balance(&admin), 700);
}

#[test]
fn test_burn_insufficient_balance() {
    let (_env, admin, _user1, user2, client) = setup();

    let result = client.try_burn(&user2, &100);
    assert_eq!(result.err().unwrap().to_string(), "InsufficientBalance");
}

#[test]
fn test_pause_lifecycle() {
    let (_env, admin, user1, _user2, client) = setup();

    assert!(!client.is_paused());
    client.set_pause(&admin, &true);
    assert!(client.is_paused());

    let result = client.try_transfer(&admin, &user1, &100);
    assert_eq!(result.err().unwrap().to_string(), "Paused");

    client.set_pause(&admin, &false);
    assert!(!client.is_paused());
    client.transfer(&admin, &user1, &100);
    assert_eq!(client.balance(&user1), 100);
}

#[test]
fn test_multi_sig_transfer() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let signer1 = Address::generate(&env);
    let signer2 = Address::generate(&env);
    let recipient = Address::generate(&env);

    let signers: Vec<Address> = Vec::from_array(&env, [admin.clone(), signer1.clone(), signer2.clone()]);

    let contract_id = env.register_contract(None, crate::CustomTokenContract);
    let client = CustomTokenContractClient::new(&env, &contract_id);

    client.initialize(
        &admin,
        &String::from_str(&env, "MultiSigToken"),
        &symbol_short!("MST"),
        &7u32,
        &1000i128,
        &3u32,
        &signers,
    );

    let approving_signers: Vec<Address> =
        Vec::from_array(&env, [admin.clone(), signer1.clone(), signer2.clone()]);

    client.multi_sig_transfer(&approving_signers, &recipient, &500);

    assert_eq!(client.balance(&recipient), 500);
    assert_eq!(client.balance(&contract_id), 0);
}

#[test]
fn test_multi_sig_insufficient_signers() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let signer1 = Address::generate(&env);
    let signer2 = Address::generate(&env);
    let recipient = Address::generate(&env);

    let signers: Vec<Address> = Vec::from_array(&env, [admin.clone(), signer1.clone(), signer2.clone()]);

    let contract_id = env.register_contract(None, crate::CustomTokenContract);
    let client = CustomTokenContractClient::new(&env, &contract_id);

    client.initialize(
        &admin,
        &String::from_str(&env, "MST"),
        &symbol_short!("MST"),
        &7u32,
        &1000i128,
        &3u32,
        &signers,
    );

    let partial_signers: Vec<Address> = Vec::from_array(&env, [admin.clone(), signer1.clone()]);
    let result = client.try_multi_sig_transfer(&partial_signers, &recipient, &500);
    assert_eq!(result.err().unwrap().to_string(), "InsufficientApprovals");
}

#[test]
fn test_multi_sig_unauthorized_signer() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let signer1 = Address::generate(&env);
    let stranger = Address::generate(&env);
    let recipient = Address::generate(&env);

    let signers: Vec<Address> = Vec::from_array(&env, [admin.clone(), signer1.clone()]);

    let contract_id = env.register_contract(None, crate::CustomTokenContract);
    let client = CustomTokenContractClient::new(&env, &contract_id);

    client.initialize(
        &admin,
        &String::from_str(&env, "MST"),
        &symbol_short!("MST"),
        &7u32,
        &1000i128,
        &2u32,
        &signers,
    );

    let bad_signers: Vec<Address> = Vec::from_array(&env, [admin.clone(), stranger.clone()]);
    let result = client.try_multi_sig_transfer(&bad_signers, &recipient, &500);
    assert_eq!(result.err().unwrap().to_string(), "NotSigner");
}

#[test]
fn test_update_signers() {
    let (_env, admin, user1, user2, client) = setup();

    let new_signers: Vec<Address> = Vec::from_array(&_env, [admin.clone(), user2.clone()]);
    client.update_signers(&admin, &2u32, &new_signers);

    let (threshold, signers) = client.get_signers();
    assert_eq!(threshold, 2);
    assert_eq!(signers.len(), 2);
    assert_eq!(signers.get(0).unwrap(), admin);
    assert_eq!(signers.get(1).unwrap(), user2);
}

#[test]
fn test_initialize_with_invalid_threshold() {
    let env = Env::default();
    let admin = Address::generate(&env);

    let contract_id = env.register_contract(None, crate::CustomTokenContract);
    let client = CustomTokenContractClient::new(&env, &contract_id);

    let result = client.try_initialize(
        &admin,
        &String::from_str(&env, "Bad"),
        &symbol_short!("BAD"),
        &7u32,
        &1000i128,
        &0u32,
        &Vec::from_array(&env, [admin.clone()]),
    );
    assert_eq!(result.err().unwrap().to_string(), "InvalidThreshold");
}

#[test]
fn test_events_emitted() {
    let (env, admin, user1, _user2, client) = setup();

    client.transfer(&admin, &user1, &100);

    let events = env.events().all();
    let transfer_event = events
        .iter()
        .find(|e| {
            let topics = e.0.clone();
            let topic2: Symbol = topics.get(1).unwrap().unwrap();
            topic2 == symbol_short!("transfer")
        })
        .unwrap();

    assert_eq!(
        transfer_event.0.get(2).unwrap().unwrap(),
        admin.clone().into_val(&env)
    );
    assert_eq!(
        transfer_event.0.get(3).unwrap().unwrap(),
        user1.clone().into_val(&env)
    );
}
