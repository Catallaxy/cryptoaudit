// Copyright 2018 Catallaxy

use bitcoin::blockdata::script;
use bitcoin::network::constants::Network;
use bitcoin::util::base58::FromBase58;
use bitcoin::util::address::Address;
use bitcoin::util::hash::Hash160;

use rustc_serialize::hex::{FromHex, ToHex};
use secp256k1::{self, Secp256k1, Signature, Message};
use secp256k1::key::{PublicKey, SecretKey};
use std::{error, fmt};


use crypto::digest::Digest;
use crypto::sha2::Sha256;

pub fn example() {
    let private_key_compressed_wif = "Ky4qA36DaWBfURv3FEziUd9Z5MELiaQLtKMvEH7kJouwpNnNnGaA";
    display_compressed_wif_private_key(String::from(private_key_compressed_wif));
}

fn display_compressed_wif_private_key(private_key_compressed_wif: String) {
    let private_key_compressed_base58: Vec<u8> =
        FromBase58::from_base58check(&private_key_compressed_wif).unwrap();

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
    println!("Compressed public key: {:?}", public_key.serialize().to_hex());

    let address = Address::from_key(Network::Bitcoin, &public_key, true);
    println!("Address: {:?}", address);

    let hash160 = Hash160::from_data(&public_key.serialize());
    let script = script::Builder::new()
        .push_int(0)
        .push_slice(&hash160[..])
        .into_script();

    let segwit_address = Address::from_script(Network::Bitcoin, &script);
    println!("P2SH-P2WSH: {:?}", segwit_address);

    let message = "catallaxy";
    let mut sha = Sha256::new();
    sha.input_str(message);
    println!("Signing message: {}", message);
    println!("Effectively signing hash of the message which is: {:?}", sha.result_str());
    let mut hashed_message = [0; 32];
    sha.result(&mut hashed_message);

    let message_to_sign = Message::from_slice(&hashed_message).unwrap();

    let secp = Secp256k1::new();
    let signature = sign(message_to_sign, secret_key, &secp).unwrap();

    let compact_signature = signature.serialize_compact(&secp);

    println!("Compact signature: {:?}",compact_signature.to_hex());
    //println!("{:?}", signature.serialize_der(&secp).to_hex());

    match verify(message_to_sign, signature, public_key, &secp) {
        Ok(_) => println!("Signature is valid"),
        Err(_) => println!("Signature is invalid"),
    };

    let message2 = "invalid";
    let mut sha2 = Sha256::new();
    sha2.input_str(message2);
    let mut hashed_message2 = [0; 32];
    sha2.result(&mut hashed_message2);
    let message_to_sign2 = Message::from_slice(&hashed_message2).unwrap();

    match verify(message_to_sign2, signature, public_key, &secp) {
        Ok(_) => println!("Signature is valid"),
        Err(_) => println!("Signature is invalid"),
    };


}

fn sign(message: Message, secret_key: SecretKey, secp: &Secp256k1) -> Result<Signature, Error> {
    println!("Signing message: {:?}", message);
    let sig = secp.sign(&message, &secret_key)?;
    Ok(sig)
}

fn verify(message: Message, signature: Signature, public_key: PublicKey, secp: &Secp256k1) -> Result<(), Error> {
    println!("Verifying signature with message: {:?}", message);
    let res = secp.verify(&message, &signature, &public_key)?;
    Ok(res)
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum Error {
	Secp(secp256k1::Error),
}

impl From<secp256k1::Error> for Error {
	fn from(e: secp256k1::Error) -> Error {
		Error::Secp(e)
	}
}

impl error::Error for Error {
	fn description(&self) -> &str {
		match *self {
			_ => "some kind of keychain error",
		}
	}
}

impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match *self {
			_ => write!(f, "some kind of keychain error"),
		}
	}
}
