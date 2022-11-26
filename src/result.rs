use marine_rs_sdk::marine;
use marine_sqlite_connector::Result;

#[marine]
pub struct IFResult {
    pub success: bool,
    pub err_msg: String,
}

impl IFResult {
    pub fn from_res(res: Result<()>) -> IFResult {
        match res {
            Ok(_v) => IFResult {
                success: true,
                err_msg: "".into(),
            },
            Err(e) => IFResult {
                success: false,
                err_msg: e.to_string(),
            },
        }
    }

    pub fn from_err_str(e: &str) -> IFResult {
        IFResult {
            success: false,
            err_msg: e.to_string(),
        }
    }
}
