// Copyright 2018 Catallaxy

use serde_json::{self, Error};
use std::fs::File;
use std::path::Path;

use crypto::digest::Digest;
use crypto::sha2::Sha256;
use rustc_serialize::hex::FromHex;
use secp256k1::{PublicKey, Secp256k1, Signature, Message};

use currencies;
use types::{AuditedAsset, RootAuditedAssets, VerifyConfig};

pub fn verify(verify_config: VerifyConfig) -> bool {
    let secp = Secp256k1::new();
    println!("Started verification for file {:?}", verify_config.file_path);
    let mut root_audited_assets: RootAuditedAssets = RootAuditedAssets::default();
    match read_json(verify_config.file_path) {
        Ok(assets_res) => root_audited_assets = assets_res,
        Err(e) => println!("Cannot read file, Error: {:?}", e),
    }

    // Check that the digest is corresponding to the message in the file
    let message_digest = get_message_digest(root_audited_assets.message.clone());
    if message_digest != root_audited_assets.message_digest {
        println!("Mismatching message digest");
        false
    } else {
        let message = get_message(root_audited_assets.message.clone());
        for audited_asset in root_audited_assets.audited_assets {
            let chain = audited_asset.chain;
            for audited_asset in audited_asset.assets {
                // Check public key and addresses
                if match_public_key_addresses(chain.clone(), audited_asset.clone(), &secp) {
                    if !match_public_key_signature(message, audited_asset.clone(), &secp) {
                        println!("Mismatched signature for asset {:?}", audited_asset);
                        println!("Exiting");
                    }
                } else {
                    println!("Mismatched addresses for chain {:?} and asset {:?}", chain, audited_asset);
                    println!("Not going to verify signature for this asset");
                    return false
                }
                // Check public key and signature


            }
        }
        true
    }
}

fn match_public_key_addresses(chain: String, audited_asset: AuditedAsset, secp: &Secp256k1) -> bool {
    let public_key_hex = audited_asset.clone().public_key;

    let public_key = PublicKey::from_slice(secp, &public_key_hex.from_hex().unwrap()).unwrap();

    let mut addresses = Vec::new();
    match chain.to_lowercase().as_ref() {
        "bitcoin" => {
            addresses = currencies::bitcoin::get_address(public_key);
        },
        "bitcoincash" => {
            addresses = currencies::bitcoincash::get_address(public_key);
        },
        "counterparty" => {
            addresses = currencies::bitcoin::get_address(public_key);
        }
        "ethereum" => {
            addresses = currencies::ethereum::get_address(public_key);
        }
        _ => {
            println!("No address derivation scheme for {}", chain);
        }
    };

    if addresses.len() == audited_asset.addresses.len() {
        for address in addresses {
            if (&audited_asset.addresses).into_iter().any(|v| v == &address) {
                continue;
            } else {
                println!("Mismatched addresses for chain {:?} and asset {:?}", chain, audited_asset);
                return false
            }
        }
        true
    } else {
        println!("Mismatched addresses for chain {:?} and asset {:?}", chain, audited_asset);
        false
    }
}

fn match_public_key_signature(message: Message, audited_asset: AuditedAsset, secp: &Secp256k1) -> bool {
    let public_key_hex = audited_asset.clone().public_key;

    let public_key = PublicKey::from_slice(secp, &public_key_hex.from_hex().unwrap()).unwrap();

    let signature = Signature::from_compact(&secp, &audited_asset.signature.from_hex().unwrap()).unwrap();

    match verify_ecdsa(message, signature, public_key, &secp) {
        Ok(_) => {
            println!("Signature is valid");
            true
        },
        Err(_) => {
            println!("Signature is invalid for {:?}", audited_asset);
            false
        },
    }
}




fn get_message(string_message: String) -> Message {
    let mut sha = Sha256::new();
    sha.input_str(&string_message);
    println!("Verifying message: {}", string_message);
    println!(
        "Effectively verifying hash of the message which is: {:?}",
        sha.result_str()
    );
    let mut hashed_message = [0; 32];
    sha.result(&mut hashed_message);

    Message::from_slice(&hashed_message).unwrap()
}

fn get_message_digest(string_message: String) -> String {
    let mut sha = Sha256::new();
    sha.input_str(&string_message);
    sha.result_str()
}

fn read_json(file_path: String) -> Result<RootAuditedAssets, Error> {
    let path = Path::new(&file_path);
    // Open the file in read-only mode.
    let file = File::open(path).expect("File not found");
    // Read the JSON contents of the file as of vec of keys from different chains
    let assets: RootAuditedAssets = serde_json::from_reader(file)?;
    Ok(assets)
}

fn verify_ecdsa(
    message: Message,
    signature: Signature,
    public_key: PublicKey,
    secp: &Secp256k1,
) -> Result<(), Error> {
    println!("Verifying signature with message: {:?}", message);
    let res = secp.verify(&message, &signature, &public_key).unwrap();
    Ok(res)
}
