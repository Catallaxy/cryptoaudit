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

use bitcoin::blockdata::script;
use bitcoin::network::constants::Network;
use bitcoin::util::base58::ToBase58;
use bitcoin::util::address::Address;
use bitcoin::util::hash::Hash160;

use secp256k1::key::PublicKey;

pub fn get_address(public_key: PublicKey) -> Vec<String> {
    let mut addresses = Vec::new();
    let p2pkh = Address::from_key(Network::Bitcoin, &public_key, true);
    addresses.push(p2pkh.to_base58check());

    let hash160 = Hash160::from_data(&public_key.serialize());
    let script = script::Builder::new()
        .push_int(0)
        .push_slice(&hash160[..])
        .into_script();

    let p2sh_p2wsh = Address::from_script(Network::Bitcoin, &script);
    addresses.push(p2sh_p2wsh.to_base58check());
    addresses
}
