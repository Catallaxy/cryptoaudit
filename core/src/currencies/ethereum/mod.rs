// Copyright 2018 Catallaxy
extern crate tiny_keccak;

use self::tiny_keccak::Keccak;
use rustc_serialize::hex::{ToHex, FromHex};
use secp256k1::key::PublicKey;

pub fn get_address(public_key: PublicKey) -> Vec<String> {
    let mut addresses = Vec::new();

    let uncrompressed_public_key_hex_with_leading = public_key.serialize_uncompressed().to_hex();
    // Remove leading bytes
    let uncrompressed_public_key_hex: String =
        String::from(&uncrompressed_public_key_hex_with_leading[2..uncrompressed_public_key_hex_with_leading.len()]);

    let uncrompressed_public_key = uncrompressed_public_key_hex.from_hex().unwrap();

    // SHA 3
    let mut keccak = Keccak::new_keccak256();
    let mut hash = [0u8; 32];
    keccak.update(&uncrompressed_public_key);
    keccak.finalize(&mut hash);

    let address = String::from("0x") + &hash[12..].to_hex();
    addresses.push(address);

	addresses
}
