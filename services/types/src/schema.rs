use marine_rs_sdk::marine;

#[marine]
#[derive(Default)]
pub struct FdbKeyPair {
    pub pk: String,
    pub sk: String,
}
