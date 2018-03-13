// Copyright 2018 Catallaxy

use bitcoin::network::constants::Network;
use bitcoin::util::base58::ToBase58;
use bitcoin::util::address::Address;

use secp256k1::key::PublicKey;

pub fn get_address(public_key: PublicKey) -> Vec<String> {
    let mut addresses = Vec::new();
    let p2pkh = Address::from_key(Network::Bitcoin, &public_key, true);
    addresses.push(p2pkh.to_base58check());

    addresses
}
