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
