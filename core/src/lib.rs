// Copyright 2018 Catallaxy

extern crate bitcoin;
extern crate crypto;
extern crate rustc_serialize;
extern crate secp256k1;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

pub mod btc;
pub mod ecdsa;
pub mod eth;
pub mod example;
pub mod types;
pub mod sign;


pub use types::{SignConfig, VerifyConfig};
