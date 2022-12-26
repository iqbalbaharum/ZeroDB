use marine_rs_sdk::marine;
use serde::{Deserialize, Serialize};
#[marine]
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FdbBlock {
    // pub timestamp: u64,
    pub content: String,
    pub previous: String,
}
