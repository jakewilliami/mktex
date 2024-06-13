use sha2::{Digest, Sha256};
use std::{fs, path::PathBuf};

fn get_hash_from_data(data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    let hash = hasher.finalize();
    format!("{:x}", hash)
}

fn get_hash_from_file(f: &PathBuf) -> String {
    let data = fs::read(f).unwrap();
    get_hash_from_data(&data)
}

// Given a local file path, and some resource, check if they are the same
// Returns true if they are the same
pub fn check_resource(local_path: &PathBuf, remote_resource: &str) -> bool {
    if !local_path.exists() {
        return false;
    }

    let h1 = get_hash_from_file(local_path);
    let h2 = get_hash_from_data(remote_resource.as_bytes());
    h1 == h2
}
