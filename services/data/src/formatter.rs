#![allow(improper_ctypes)]
extern crate serde;

use types::*;

use marine_rs_sdk::marine;
use marine_rs_sdk::module_manifest;
use marine_rs_sdk::WasmLoggerBuilder;
use serde::{Deserialize, Serialize};

module_manifest!();

pub fn main() {
    WasmLoggerBuilder::new()
        .with_log_level(log::LevelFilter::Info)
        .build()
        .unwrap();
}

#[marine]
pub fn fdb_serialize(data: String, previous: String) -> String {
    let data = FdbData { data, previous };

    serde_json::to_string(&data).unwrap()
}

#[marine]
pub fn fdb_deserialize(json: &String) -> String {
    serde_json::from_str(json).unwrap()
}

#[marine]
#[derive(Serialize, Deserialize)]
struct FdbData {
    data: String,
    previous: String,
}
