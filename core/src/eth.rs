// Copyright 2018 Catallaxy

use rustc_serialize::hex::{FromHex, ToHex};
use secp256k1::{Secp256k1, Message};
use secp256k1::key::{PublicKey, SecretKey};

use ecdsa;
use types::{AuditedAssets, AuditedAsset, Key};

pub fn ethereum_sign(keys: Vec<Key>, message: Message) ->  AuditedAssets {
    println!("Starting Ethereum Signing Process");
    let secp = Secp256k1::new();

    let mut assets: Vec<AuditedAsset> = Vec::new();

    for key in keys {
        let secret_key = get_secret_key(key.key, &secp);
        assets.push(get_audited_asset(secret_key, message, &secp));
    }

    AuditedAssets {
        chain: String::from("Ethereum"),
        assets: assets,
    }

}

fn get_audited_asset(secret_key: SecretKey, message: Message, secp: &Secp256k1) -> AuditedAsset {
    let public_key = PublicKey::from_secret_key(&secp, &secret_key)
        .map_err(ecdsa::Error::Secp)
        .unwrap();
    println!("Compressed public key: {:?}", public_key.serialize().to_hex());

    let signature = ecdsa::sign(message, secret_key, &secp).unwrap();
    let compact_signature = signature.serialize_compact(&secp);

    AuditedAsset {
        public_key: public_key.serialize().to_hex(),
        signature: compact_signature.to_hex(),
        signature_algorithm: String::from("SHA256-ECDSA")
    }

}


fn get_secret_key(private_key: String, secp: &Secp256k1) -> SecretKey {
    let private_key_vec: Vec<u8> = private_key.from_hex().unwrap();

    println!("Private key: {:?}", private_key_vec.to_hex());

    let secret_key = SecretKey::from_slice(&secp, &private_key_vec).unwrap();

    secret_key
}
