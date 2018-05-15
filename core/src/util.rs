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

use bitcoin::util::base58::FromBase58;

use rustc_serialize::hex::{FromHex, ToHex};
use secp256k1::Secp256k1;
use secp256k1::key::SecretKey;

pub fn get_secret_key(private_key_string: String, secp: &Secp256k1) -> Option<SecretKey> {
    let private_key: Vec<u8>;

    let length = private_key_string.len();
    if length == 52 {
        // Compressed WIF
        let private_key_check: Vec<u8> = FromBase58::from_base58check(&private_key_string).unwrap();
        let private_key_hex_check = private_key_check.to_hex();
        // Removing network parameters and checksum
        let private_key_hex: String =
            String::from(&private_key_hex_check[2..private_key_hex_check.len() - 2]);
        //println!("Private key: {:?}", private_key_hex);
        private_key = private_key_hex.from_hex().unwrap();

        let secret_key = SecretKey::from_slice(&secp, &private_key).unwrap();
        Some(secret_key)
    } else if length == 64 {
        // HEX Format
        private_key = private_key_string.from_hex().unwrap();
        //println!("Private key: {:?}", private_key.to_hex());

        let secret_key = SecretKey::from_slice(&secp, &private_key).unwrap();
        Some(secret_key)
    } else {
        println!(
            "Unrecognized private key format for {}. Use Compressed WIF or HEX format",
            private_key_string
        );
        None
    }
}
