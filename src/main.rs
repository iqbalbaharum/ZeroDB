use marine_rs_sdk::marine;
use marine_rs_sdk::module_manifest;
use marine_sqlite_connector::Result;

mod auth;
mod db;
mod result;

use auth::*;
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

// add data to ipfs and then to the sqlite hash table
#[marine]
pub fn add(key: String, owner_pk: String, cid: String, hash: String) -> IFResult {
    // verify of the owner_pk and hash (ED25519)
    // once verified, check if the key exists
    // insert or update the data
    let conn = db::get_connection(DEFAULT_PATH);
    let res = db::add_record(&conn, key, owner_pk, cid);

    IFResult::from_res(res)
}
