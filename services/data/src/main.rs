use marine_rs_sdk::marine;
use marine_rs_sdk::module_manifest;
use marine_rs_sdk::WasmLoggerBuilder;

use types::FdbBlock;

module_manifest!();

pub fn main() {
    WasmLoggerBuilder::new()
        .with_log_level(log::LevelFilter::Info)
        .build()
        .unwrap();
}

#[marine]
pub fn serialize(content: String, previous: String) -> String {
    let data = FdbBlock { content, previous };
    serde_json::to_string(&data).unwrap_or("".to_string())
}

#[marine]
pub fn deserialize(json: &String) -> FdbBlock {
    serde_json::from_str(json).unwrap()
}
