// Copyright 2018 Catallaxy

extern crate bitcoin;
extern crate crypto;
extern crate rustc_serialize;
extern crate secp256k1;

pub mod types;
pub mod example;

pub use types::{SignConfig, VerifyConfig};
