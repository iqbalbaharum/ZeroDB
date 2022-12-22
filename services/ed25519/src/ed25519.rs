#![allow(improper_ctypes)]

use ed25519_compact::{KeyPair, PublicKey, Signature};
use std::ops::Deref;

use types::*;

use marine_rs_sdk::marine;
use marine_rs_sdk::module_manifest;
use marine_rs_sdk::WasmLoggerBuilder;

module_manifest!();

pub fn main() {
    WasmLoggerBuilder::new()
        .with_log_level(log::LevelFilter::Info)
        .build()
        .unwrap();
}

#[marine]
pub fn generate_keypair() -> FdbKeyPair {
    let kp = KeyPair::generate();
    let base64_pk = base64::encode(kp.pk.deref());

    let base64_sk = base64::encode(kp.sk.deref());

    FdbKeyPair {
        pk: base64_pk,
        sk: base64_sk,
    }
}

#[marine]
pub fn verify(public_key: String, signature: String, message: String) -> bool {
    let p_key_decoded = base64::decode(public_key).unwrap();
    let sign_decoded = base64::decode(signature).unwrap();

    let pk: [u8; 32] = p_key_decoded
        .try_into()
        .expect("Error: public_key with incorrect length");

    let sign: [u8; 64] = sign_decoded
        .try_into()
        .expect("Error: Sign with incorrect length");

    let p_key = PublicKey::new(pk);

    match p_key.verify(message, &Signature::new(sign)) {
        Ok(_) => true,
        Err(_) => false,
    }
}
