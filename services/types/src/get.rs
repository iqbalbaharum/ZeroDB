use eyre::Result;
use marine_rs_sdk::marine;

#[marine]
pub struct FdbGetResult {
    pub success: bool,
    pub error: String,
    pub data: String,
}

impl From<Result<String>> for FdbGetResult {
    fn from(result: Result<String>) -> Self {
        match result {
            Ok(data) => Self {
                success: true,
                error: "".to_string(),
                data,
            },
            Err(err) => Self {
                success: false,
                error: err.to_string(),
                data: "".to_string(),
            },
        }
    }
}

#[marine]
pub struct FdbGetResults {
    pub success: bool,
    pub error: String,
    pub datas: Vec<FdbDht>,
}

#[marine]
pub struct FdbDht {
    pub public_key: String,
    pub cid: String,
    pub key: String,
}

impl From<Result<Vec<FdbDht>>> for FdbGetResults {
    fn from(result: Result<Vec<FdbDht>>) -> Self {
        match result {
            Ok(datas) => Self {
                success: true,
                error: "".to_string(),
                datas,
            },
            Err(err) => Self {
                success: false,
                error: err.to_string(),
                datas: Vec::new(),
            },
        }
    }
}
