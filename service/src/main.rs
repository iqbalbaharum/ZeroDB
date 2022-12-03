use log::{info, warn, Log};
use std::ops::Deref;

use ed25519_compact::KeyPair;
use marine_rs_sdk::{marine, module_manifest};
use rand::distributions::Alphanumeric;
use rand::Rng;

mod auth;
mod db;
mod ed25519;
mod ipfs;
mod result;

use auth::*;
use db::*;
use ed25519::*;
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

// add data to ipfs and then to the sqlite hash table
#[marine]
pub fn add(
    key: String,
    cid: String,
    public_key: String,
    signature: String,
    message: String,
) -> IFResult {
    let verify = verify(public_key.clone(), signature, message);

    if !verify {
        return IFResult::from_err_str("You are not the owner!");
    }

    let conn = db::get_connection(DEFAULT_PATH);

    // Check if PK exist
    match get_record_by_pk(&conn, public_key.clone()) {
        Ok(value) => {
            if value.is_none() {
                let res = db::add_record(&conn, key, public_key, cid);
                IFResult::from_res(res)
            } else {
                let res = db::update_record(&conn, public_key, cid);
                IFResult::from_res(res)
            }
        }
        Err(err) => IFResult::from_err_str(&err.message.unwrap()),
    }
}

#[marine]
pub fn get(key: String) -> Record {
    let conn = db::get_connection(DEFAULT_PATH);
    let user = db::get_record(&conn, key);

    user.unwrap_or_default()
}

#[marine]
pub fn get_records() -> Vec<Record> {
    let conn = db::get_connection(DEFAULT_PATH);
    let records = db::get_records(&conn);

    records.unwrap_or_default()
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
