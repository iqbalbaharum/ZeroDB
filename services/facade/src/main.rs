#![allow(improper_ctypes)]

use marine_rs_sdk::marine;
use marine_rs_sdk::module_manifest;
use marine_rs_sdk::WasmLoggerBuilder;

use types::FdbKeyPair;

module_manifest!();

pub fn main() {
    WasmLoggerBuilder::new().build().unwrap();
}

#[marine]
pub fn generate() -> FdbKeyPair {
    generate_keypair()
}

#[marine]
pub fn add(
    key: String,
    data: String,
    public_key: String,
    signature: String,
    message: String,
) -> FdbResult {
    // Check if the string is a json
    // Verify signature
    // Check if theres previous cid
    // Format object
    // Add to dag
    // Add to dht
}

#[marine]
pub fn fork(
    key: String,
    data: String,
    forked_cid: String,
    public_key: String,
    signature: String,
    message: String,
) -> FdbResult {
    // Check if the string is a json
    // Verify signature
    // Check if theres previous cid
    // Format object
    // Add to dag
    // Add to dht
}

/// Importing `fdb_ed25519` module
#[marine]
#[link(wasm_import_module = "fdb_ed25519")]
extern "C" {
    #[link_name = "generate_keypair"]
    pub fn generate_keypair() -> FdbKeyPair;
}
