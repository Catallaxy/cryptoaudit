// Copyright 2018 Catallaxy
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use serde_json::{self, Error};
use std::fs::File;
use std::path::Path;

use crypto::digest::Digest;
use crypto::sha2::Sha256;
use secp256k1::Message;

use ecdsa;
use types::{Assets, AuditedAssets, RootAuditedAssets, SignConfig};

static SUPPORTED_CHAIN: &'static [&str] = &["bitcoin", "bitcoincash", "counterparty", "ethereum"];
/// Sign a JSON file containing a list of private keys and write a audits.json file
pub fn sign(sign_config: SignConfig) {
    let mut assets: Vec<Assets> = Vec::new();
    match read_json(sign_config.file_path) {
        Ok(assets_res) => assets = assets_res,
        Err(e) => println!("Cannot read file, Error: {:?}", e),
    }

    let message = get_message(sign_config.message.clone());

    let mut audited_assets: Vec<AuditedAssets> = Vec::new();

    for asset in assets {
        let chain = asset.chain.to_lowercase();
        if (&SUPPORTED_CHAIN).into_iter().any(|v| v == &chain) {
            audited_assets.push(ecdsa::sign_with_keys(asset.chain, asset.keys, message));
        } else {
            println!("Unsuported chain: {}", chain);
        }
    }

    // The final result
    let result = RootAuditedAssets {
        message: sign_config.message.clone(),
        message_digest: get_message_digest(sign_config.message),
        audited_assets: audited_assets,
    };

    let output = String::from("signed-assets.json");
    match write_json(output, result) {
        Ok(_) => println!("Successfully signed message with keys"),
        Err(e) => println!("Failed to sign message, Error: {:?}", e),
    }
}

fn get_message(string_message: String) -> Message {
    let mut sha = Sha256::new();
    sha.input_str(&string_message);
    println!("Signing message: {}", string_message);
    println!(
        "Effectively signing hash of the message which is: {:?}",
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
