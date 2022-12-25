use marine_rs_sdk::marine;
use serde::{Deserialize, Serialize};

#[marine]
#[derive(Debug, Serialize, Deserialize)]
pub struct FdbBlock {
    pub content: String,
    pub previous: String,
}
