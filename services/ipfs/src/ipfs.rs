#![allow(improper_ctypes)]

use types::*;

use marine_rs_sdk::marine;
use marine_rs_sdk::module_manifest;
use marine_rs_sdk::MountedBinaryResult;
use marine_rs_sdk::WasmLoggerBuilder;

use eyre::Result;

const DEFAULT_TIMEOUT_SEC: u64 = 1u64;
const DEFAULT_IPFS_MULTIADDR: &str = "/ip4/127.0.0.1/tcp/5001";

module_manifest!();

pub fn main() {
    WasmLoggerBuilder::new()
        .with_log_level(log::LevelFilter::Info)
        .build()
        .unwrap();
}

fn make_cmd_args(args: Vec<String>, api_multiaddr: String, timeout_sec: u64) -> Vec<String> {
    args.into_iter()
        .chain(vec![
            String::from("--timeout"),
            get_timeout_string(timeout_sec),
            String::from("--api"),
            api_multiaddr,
        ])
        .collect()
}

fn unwrap_mounted_binary_result(result: MountedBinaryResult) -> Result<String> {
    result
        .into_std()
        .ok_or(eyre::eyre!(
            "stdout or stderr contains non valid UTF8 string"
        ))?
        .map_err(|e| eyre::eyre!("ipfs cli call failed: {}", e))
}

#[inline]
fn get_timeout_string(timeout: u64) -> String {
    format!("{}s", timeout)
}

#[marine]
pub fn dag_put(object: String, api_multiaddr: String, timeout_sec: u64) -> FdbPutResult {
    let address: String;
    let t;

    if api_multiaddr.is_empty() {
        address = DEFAULT_IPFS_MULTIADDR.to_string();
    } else {
        address = api_multiaddr;
    }

    if timeout_sec == 0 {
        t = DEFAULT_TIMEOUT_SEC;
    } else {
        t = timeout_sec;
    }

    log::info!("dag put called with object {}", object);

    let data = format!(r#"{}"#, object);

    let input = String::from("echo '".to_owned() + &data + "' | ipfs dag put");

    let args = vec![String::from("-c"), input];

    let cmd = make_cmd_args(args, address, t);

    log::info!("ipfs put args : {:?}", cmd);

    unwrap_mounted_binary_result(bash(cmd))
        .map(|res| res.trim().to_string())
        .into()
}

#[marine]
pub fn dag_get(hash: String, api_multiaddr: String, timeout_sec: u64) -> FdbGetResult {
    log::info!("get called with hash {}", hash);

    let args = vec![String::from("dag"), String::from("get"), hash];

    let cmd = make_cmd_args(args, api_multiaddr, timeout_sec);

    log::info!("ipfs dag get args {:?}", cmd);

    unwrap_mounted_binary_result(ipfs(cmd))
        .map(|res| res.trim().to_string())
        .into()
}

#[marine]
#[link(wasm_import_module = "host")]
extern "C" {
    /// Execute provided cmd as a parameters of ipfs cli, return result.
    pub fn ipfs(cmd: Vec<String>) -> MountedBinaryResult;

    /// Execute command, return result
    pub fn bash(cmd: Vec<String>) -> MountedBinaryResult;
}
