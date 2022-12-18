use eyre::Result;
use marine_rs_sdk::marine;

//
#[marine]
pub struct FdbResult {
    pub success: bool,
    pub err_msg: String,
}

impl From<Result<()>> for FdbResult {
    fn from(res: Result<()>) -> FdbResult {
        match res {
            Ok(_v) => FdbResult {
                success: true,
                err_msg: "".into(),
            },
            Err(e) => FdbResult {
                success: false,
                err_msg: e.to_string(),
            },
        }
    }
}
