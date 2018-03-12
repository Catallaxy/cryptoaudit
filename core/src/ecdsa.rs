// Copyright 2018 Catallaxy

use secp256k1::{self, Secp256k1, Signature, Message};
use secp256k1::key::{PublicKey, SecretKey};
use std::{error, fmt};

pub fn sign(message: Message, secret_key: SecretKey, secp: &Secp256k1) -> Result<Signature, Error> {
    println!("Signing message: {:?}", message);
    let sig = secp.sign(&message, &secret_key)?;
    Ok(sig)
}

pub fn verify(message: Message, signature: Signature, public_key: PublicKey, secp: &Secp256k1) -> Result<(), Error> {
    println!("Verifying signature with message: {:?}", message);
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
			_ => "some kind of keychain error",
		}
	}
}

impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match *self {
			_ => write!(f, "some kind of keychain error"),
		}
	}
}
