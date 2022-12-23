#![allow(improper_ctypes)]

mod auth;

use marine_rs_sdk::marine;
use marine_rs_sdk::module_manifest;
use marine_rs_sdk::WasmLoggerBuilder;

use auth::am_i_owner;
use types::FdbGetResult;
use types::{FdbGetResults, FdbKeyPair, FdbPutResult, FdbResult};

module_manifest!();

pub fn main() {
    WasmLoggerBuilder::new().build().unwrap();
}

#[marine]
pub fn generate_new_keypair() -> FdbKeyPair {
    generate_keypair()
}

#[marine]
pub fn init_service() -> FdbResult {
    if !am_i_owner() {
        return FdbResult::from_err_str("You are not the owner!");
    }

    init_dht()
}

#[marine]
pub fn add(key: String, data: String, public_key: String, signature: String) -> FdbResult {
    log::info!("data: {:?}", data);
    // Format object
    // Add to dag
    let result: FdbPutResult = dag_put(data.clone(), "".to_string(), 0);
    log::info!("error: {}", result.error);
    // Add to dht
    if result.hash.len() == 0 {
        FdbResult::from_err_str(format!("Invalid CID produce: {}", result.hash).as_str())
    } else {
        insert(key, result.hash, public_key, signature, data)
    }
}

/**
 * Retrieve all data from the history
 * TODO: Incomplete - currently only read the latest data only
 */
#[marine]
pub fn get_history(key: String) -> Vec<String> {
    let results = get_cids_from_dht(key);

    let mut datas: Vec<String> = Vec::new();

    for cid in results.datas.iter() {
        match cid {
            cid => {
                let r = ipfs_dag_get(cid.to_string());
                datas.push(r.data.clone());
            }
        }
    }

    datas
}

/**
 * For fast retrieval - must your aqua to run do parallel
 */
#[marine]
pub fn get_cids_from_dht(key: String) -> FdbGetResults {
    let cids = get_records_by_key(key);

    log::info!("{:?}", cids);

    FdbGetResults {
        success: true,
        error: "".to_string(),
        datas: cids,
    }
}

/**
 * Expose IPFS DAG get API to be access in service
 */
#[marine]
pub fn ipfs_dag_get(cid: String) -> FdbGetResult {
    dag_get(cid, "".to_string(), 0)
}

// #[marine]
// pub fn fork(
//     key: String,
//     data: String,
//     forked_cid: String,
//     public_key: String,
//     signature: String,
//     message: String,
// ) -> FdbResult {
//     // Verify signature
//     // Check if theres previous cid
//     // Format object
//     // Add to dag
//     // Add to dht
// }

/// Importing `fdb_ed25519` module
#[marine]
#[link(wasm_import_module = "fdb_ed25519")]
extern "C" {
    #[link_name = "generate_keypair"]
    pub fn generate_keypair() -> FdbKeyPair;
}

/// Importing `fdb_data` module
#[marine]
#[link(wasm_import_module = "fdb_dht")]
extern "C" {

    #[link_name = "init_dht"]
    pub fn init_dht() -> FdbResult;

    #[link_name = "insert"]
    pub fn insert(
        key: String,
        cid: String,
        public_key: String,
        signature: String,
        message: String,
    ) -> FdbResult;

    #[link_name = "get_records_by_key"]
    pub fn get_records_by_key(key: String) -> Vec<String>;
}

#[marine]
#[link(wasm_import_module = "fdb_ipfs")]
extern "C" {
    #[link_name = "dag_put"]
    pub fn dag_put(object: String, api_multiaddr: String, timeout_sec: u64) -> FdbPutResult;

    #[link_name = "dag_get"]
    pub fn dag_get(hash: String, api_multiaddr: String, timeout_sec: u64) -> FdbGetResult;
}
