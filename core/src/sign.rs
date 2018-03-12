// Copyright 2018 Catallaxy

use serde_json::{self, Error};
use std::fs::File;
use std::path::Path;

use crypto::digest::Digest;
use crypto::sha2::Sha256;
use secp256k1::Message;

use btc;
use eth;
use types::{Assets, SignConfig};

/// Sign a JSON file containing a list of private keys and write a audits.json file
pub fn sign(sign_config: SignConfig) {
    let mut assets: Vec<Assets> = Vec::new();
    match read_json(sign_config.file_path) {
        Ok(assets_res) => assets = assets_res,
        Err(e) => println!("Cannot read file, Error: {:?}", e),
    }

    let message = create_message(sign_config.message);

    for asset in assets {
        match asset.chain.to_lowercase().as_ref() {
            "bitcoin" => {
                btc::bitcoin_sign(asset.keys, message);
            },
            "ethereum" => {
                eth::ethereum_sign(asset.keys, message);
            },
            _ => {
                println!("Unsuported chain: {}", asset.chain);
            }
        }
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

fn read_json(file_path: String) -> Result<Vec<Assets>, Error> {
    let path = Path::new(&file_path);
    // Open the file in read-only mode.
    let file = File::open(path).expect("File not found");
    // Read the JSON contents of the file as of vec of keys from different chains
    let assets: Vec<Assets> = serde_json::from_reader(file)?;
    Ok(assets)
}
