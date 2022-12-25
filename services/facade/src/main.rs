#![allow(improper_ctypes)]

mod auth;

use marine_rs_sdk::marine;
use marine_rs_sdk::module_manifest;
use marine_rs_sdk::WasmLoggerBuilder;

use auth::am_i_owner;
use types::*;

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
    // Check if there`s existing block
    let current_cid = get_record(key.clone(), public_key.clone());
    log::info!("DHT cid: {}", current_cid);

    // Format object
    let d = serialize(data, current_cid);
    log::info!("Formatted data: {}", d);
    // Add to dag
    let result: FdbPutResult = dag_put(d.clone(), "".to_string(), 0);
    log::info!("result: {}", result.hash);
    // Add to dht
    if result.hash.len() == 0 {
        FdbResult::from_err_str(format!("Invalid CID produce: {}", result.hash).as_str())
    } else {
        insert(key, result.hash, public_key, signature, d)
    }
}

/**
 * Retrieve latest datasets
 */
#[marine]
pub fn get_latest_datasets(key: String) -> Vec<String> {
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

#[marine]
pub fn get_history(key: String, pk: String) -> Vec<String> {
    let latest_cid = get_record(key.clone(), pk);

    let mut items: Vec<String> = Vec::new();

    let mut prev = "";
    let mut get_result = dag_get(latest_cid, "".to_string(), 0);
    let mut block = deserialize(&get_result.data);

    items.push(block.content.clone());
    prev = block.previous.as_str();

    while prev.len() > 0 {
        get_result = dag_get(prev.to_string(), "".to_string(), 0);
        block = deserialize(&get_result.data);
        items.push(block.content.clone());
        prev = block.previous.as_str();
    }

    items
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

    #[link_name = "get_record"]
    pub fn get_record(key: String, pk: String) -> String;
}

#[marine]
#[link(wasm_import_module = "fdb_ipfs")]
extern "C" {
    #[link_name = "dag_put"]
    pub fn dag_put(object: String, api_multiaddr: String, timeout_sec: u64) -> FdbPutResult;

    #[link_name = "dag_get"]
    pub fn dag_get(hash: String, api_multiaddr: String, timeout_sec: u64) -> FdbGetResult;
}

#[marine]
#[link(wasm_import_module = "fdb_data")]
extern "C" {
    #[link_name = "serialize"]
    pub fn serialize(content: String, previous: String) -> String;

    #[link_name = "deserialize"]
    pub fn deserialize(json: &String) -> FdbBlock;
}
