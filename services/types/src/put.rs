use eyre::Result;
use marine_rs_sdk::marine;

#[marine]
pub struct FdbPutResult {
    pub success: bool,
    pub error: String,
    pub hash: String,
}

impl From<Result<String>> for FdbPutResult {
    fn from(result: Result<String>) -> Self {
        match result {
            Ok(hash) => Self {
                success: true,
                error: "".to_string(),
                hash,
            },
            Err(err) => Self {
                success: false,
                error: err.to_string(),
                hash: "".to_string(),
            },
        }
    }
}
