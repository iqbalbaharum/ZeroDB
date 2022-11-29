use std::fmt::format;
use std::ops::Deref;

use ed25519_compact::{KeyPair, PublicKey, Signature};
use marine_rs_sdk::{marine, module_manifest};
use rand::distributions::Alphanumeric;
use rand::Rng;

mod auth;
mod db;
mod ipfs;
mod result;

use auth::*;
use db::*;
use ipfs::vault_dir;
use result::*;

module_manifest!();

pub fn main() {}

const DEFAULT_PATH: &str = "zero_db_test";

#[marine]
pub fn init_service() -> IFResult {
    if !am_i_owner() {
        return IFResult::from_err_str("You are not the owner!");
    }

    let conn = db::get_connection(DEFAULT_PATH);
    let res = db::create_dht_table(&conn);
    IFResult::from_res(res)
}

#[marine]
pub fn reset_service() -> IFResult {
    if !am_i_owner() {
        return IFResult::from_err_str("You are not the owner!");
    }

    let conn = db::get_connection(DEFAULT_PATH);
    let res = db::delete_dht_table(&conn);
    IFResult::from_res(res)
}

#[marine]
pub fn generate_keypair() -> ResKeyPair {
    let kp = KeyPair::generate();
    let base64_pk = base64::encode(kp.pk.deref());

    let base64_sk = base64::encode(kp.sk.deref());

    ResKeyPair {
        pk: base64_pk,
        sk: base64_sk,
    }
}

#[marine]
pub fn verify(public_key: String, signature: String, message: String) -> IFResult {
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
        Ok(_) => IFResult {
            success: true,
            err_msg: String::new(),
        },
        Err(err) => IFResult {
            success: false,
            err_msg: err.to_string(),
        },
    }
}

// add data to ipfs and then to the sqlite hash table
#[marine]
pub fn add(key: String, owner_pk: String, cid: String) -> IFResult {
    // verify of the owner_pk and hash (ED25519)
    // once verified, check if the key exists
    // insert or update the data
    let conn = db::get_connection(DEFAULT_PATH);
    let res = db::add_record(&conn, key, owner_pk, cid);

    IFResult::from_res(res)
}

#[marine]
pub fn get(key: String) -> Record {
    let conn = db::get_connection(DEFAULT_PATH);
    let user = db::get_record(&conn, key);

    user.unwrap_or_default()
}

// Write files to node /tmp storage
#[marine]
pub fn write_file_size(file_path: String) -> IFResult {
    let bytes = std::fs::read(file_path).unwrap();

    let name: String = rand::thread_rng()
        .sample_iter(Alphanumeric)
        .take(16)
        .map(char::from)
        .collect();

    let file = vault_dir().join(&name);
    match std::fs::write(&file, bytes.len().to_string()) {
        Ok(_) => IFResult {
            success: true,
            err_msg: String::new(),
        },
        Err(err) => IFResult {
            success: false,
            err_msg: err.to_string(),
        },
    }
}
