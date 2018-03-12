extern crate bitcoin;
extern crate crypto;
extern crate rustc_serialize;
extern crate secp256k1;

use bitcoin::blockdata::script;
use bitcoin::network::constants::Network;
use bitcoin::util::base58::FromBase58;
use bitcoin::util::address::Address;
use bitcoin::util::hash::Hash160;

use rustc_serialize::hex::{FromHex, ToHex};
use secp256k1::Secp256k1;
use secp256k1::key::{PublicKey, SecretKey};

fn main() {

    let private_key_compressed_wif = "Ky4qA36DaWBfURv3FEziUd9Z5MELiaQLtKMvEH7kJouwpNnNnGaA";
    display_compressed_wif_private_key(String::from(private_key_compressed_wif));
}

fn display_compressed_wif_private_key(private_key_compressed_wif: String) {
    let private_key_compressed_base58: Vec<u8> =
        FromBase58::from_base58check(&private_key_compressed_wif).unwrap();
    println!("Hello, world!");
    let private_key_compressed_base58_hex = private_key_compressed_base58.to_hex();
    // Removing network parameters and checkcsum
    let private_key_hex: String = String::from(
        &private_key_compressed_base58_hex[2..private_key_compressed_base58_hex.len() - 2],
    );
    println!("Private key: {:?}", private_key_hex);

    let private_key_slice: Vec<u8> = private_key_hex.from_hex().unwrap();

    let secp = Secp256k1::new();
    let secret_key = SecretKey::from_slice(&secp, &private_key_slice).unwrap();

    let public_key = PublicKey::from_secret_key(&secp, &secret_key)
        .map_err(Error::Secp)
        .unwrap();
    println!("Public key: {:?}", public_key);

    let address = Address::from_key(Network::Bitcoin, &public_key, true);
    println!("Address: {:?}", address);

    let hash160 = Hash160::from_data(&public_key.serialize());
    let script = script::Builder::new()
        .push_int(0)
        .push_slice(&hash160[..])
        .into_script();

    let segwit_address = Address::from_script(Network::Bitcoin, &script);
    println!("P2SH-P2WSH: {:?}", segwit_address);
}

/// A contract-hash error
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Error {
    /// Contract hashed to an out-of-range value (this is basically impossible
    /// and much more likely suggests memory corruption or hardware failure)
    BadTweak(secp256k1::Error),
    /// Other secp256k1 related error
    Secp(secp256k1::Error),
    /// Encountered an uncompressed key in a script we were deserializing. The
    /// reserialization will compress it which might be surprising so we call
    /// this an error.
    UncompressedKey,
    /// Expected a public key when deserializing a script, but we got something else.
    ExpectedKey,
    /// Expected some sort of CHECKSIG operator when deserializing a script, but
    /// we got something else.
    ExpectedChecksig,
}
