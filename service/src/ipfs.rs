use marine_rs_sdk::get_call_parameters;
use std::path::{Path, PathBuf};

pub fn vault_dir() -> PathBuf {
    let particle_id = get_call_parameters().particle_id;
    let vault = Path::new("/tmp").join("vault").join(particle_id);

    vault
}
