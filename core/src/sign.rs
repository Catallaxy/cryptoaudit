// Copyright 2018 Catallaxy

use serde_json::{self, Error};
use std::fs::File;
use std::path::Path;

use crypto::digest::Digest;
use crypto::sha2::Sha256;
use secp256k1::Message;

use btc;
use eth;
use types::{Assets, RootAuditedAssets, AuditedAssets, SignConfig};

/// Sign a JSON file containing a list of private keys and write a audits.json file
pub fn sign(sign_config: SignConfig) {
    let mut assets: Vec<Assets> = Vec::new();
    match read_json(sign_config.file_path) {
        Ok(assets_res) => assets = assets_res,
        Err(e) => println!("Cannot read file, Error: {:?}", e),
    }

    let message = create_message(sign_config.message.clone());

    let mut audited_assets: Vec<AuditedAssets> = Vec::new();

    for asset in assets {
        match asset.chain.to_lowercase().as_ref() {
            "bitcoin" => {
                audited_assets.push(btc::bitcoin_sign(asset.keys, message));
            },
            "ethereum" => {
                audited_assets.push(eth::ethereum_sign(asset.keys, message));
            },
            _ => {
                println!("Unsuported chain: {}", asset.chain);
            }
        }
    }

    // The final result
    let result = RootAuditedAssets {
        message: sign_config.message,
        audited_assets: audited_assets,
    };

    let output = String::from("signed-assets.json");
    match write_json(output, result) {
        Ok(_) => println!("Successfully signed message with keys"),
        Err(e) => println!("Failed to sign message, Error: {:?}", e),
    }
}

fn create_message(string_message: String) -> Message {
    let mut sha = Sha256::new();
    sha.input_str(&string_message);
    println!("Signing message: {}", string_message);
    println!("Effectively signing hash of the message which is: {:?}", sha.result_str());
    let mut hashed_message = [0; 32];
    sha.result(&mut hashed_message);

    Message::from_slice(&hashed_message).unwrap()
}

fn write_json(file_path: String, root_audited_assets: RootAuditedAssets) -> Result<(), Error> {
    let path = Path::new(&file_path);
    // Open the file in read-only mode.
    let file = File::create(path).expect("Cannot create file");
    // Read the JSON contents of the file as of vec of keys from different chains
    let res = serde_json::to_writer_pretty(file, &root_audited_assets)?;
    Ok(res)
}

fn read_json(file_path: String) -> Result<Vec<Assets>, Error> {
    let path = Path::new(&file_path);
    // Open the file in read-only mode.
    let file = File::open(path).expect("File not found");
    // Read the JSON contents of the file as of vec of keys from different chains
    let assets: Vec<Assets> = serde_json::from_reader(file)?;
    Ok(assets)
}
