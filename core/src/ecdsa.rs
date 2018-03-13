// Copyright 2018 Catallaxy
use rustc_serialize::hex::ToHex;
use secp256k1::{self, Message, Secp256k1, Signature};
use secp256k1::key::{PublicKey, SecretKey};
use std::{error, fmt};

use currencies;
use ecdsa;
use types::{AuditedAsset, AuditedAssets, Key};
use util;

pub fn sign_with_keys(chain: String, keys: Vec<Key>, message: Message) -> AuditedAssets {
    println!("Starting {} Signing Process", chain);
    let secp = Secp256k1::new();

    let mut assets: Vec<AuditedAsset> = Vec::new();

    for key in keys {
        let secret_key = util::get_secret_key(key.key, &secp);
        if secret_key.is_some() {
            assets.push(get_audited_asset(
                chain.clone(),
                secret_key.unwrap(),
                message,
                &secp,
            ));
        } else {
            // If unable to get key from JSOn just write a failed json for this part
            let failed_asset = AuditedAsset {
                addresses: vec![String::from("FAILED")],
                public_key: String::from("FAILED"),
                signature: String::from("FAILED"),
                signature_algorithm: String::from("FAILED"),
            };
            assets.push(failed_asset);
        }
    }
    println!("Done {} Signing Process", chain);

    AuditedAssets {
        chain: chain,
        assets: assets,
    }
}

fn get_audited_asset(
    chain: String,
    secret_key: SecretKey,
    message: Message,
    secp: &Secp256k1,
) -> AuditedAsset {
    let public_key = PublicKey::from_secret_key(&secp, &secret_key)
        .map_err(ecdsa::Error::Secp)
        .unwrap();
    /*println!(
        "Compressed public key: {:?}",
        public_key.serialize().to_hex()
    );*/

    let signature = ecdsa::sign(message, secret_key, &secp).unwrap();
    let compact_signature = signature.serialize_compact(&secp);

    let mut addresses = Vec::new();
    match chain.to_lowercase().as_ref() {
        "bitcoin" => {
            addresses = currencies::bitcoin::get_address(public_key);
        },
        "bitcoincash" => {
            addresses = currencies::bitcoincash::get_address(public_key);
        },
        "ethereum" => {
            addresses = currencies::ethereum::get_address(public_key);
        }
        _ => {
            println!("No address derivation scheme for {}", chain);
            addresses = vec![String::from("")];
        }
    }

    AuditedAsset {
        addresses : addresses,
        public_key: public_key.serialize().to_hex(),
        signature: compact_signature.to_hex(),
        signature_algorithm: String::from("SHA256-ECDSA"),
    }
}

pub fn sign(message: Message, secret_key: SecretKey, secp: &Secp256k1) -> Result<Signature, Error> {
    //println!("Signing message: {:?}", message);
    let sig = secp.sign(&message, &secret_key)?;
    Ok(sig)
}

pub fn verify(
    message: Message,
    signature: Signature,
    public_key: PublicKey,
    secp: &Secp256k1,
) -> Result<(), Error> {
    //println!("Verifying signature with message: {:?}", message);
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
            _ => "some kind of error",
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            _ => write!(f, "some kind of error"),
        }
    }
}
