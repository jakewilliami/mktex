// Fetch resource!
use super::{config, local, remote};
use std::{fs, path::Path};

#[derive(PartialEq)]
pub enum ResourceLocation {
    Local,
    Remote,
}

pub fn fetch_resource(resource: &str, loc: &ResourceLocation) -> String {
    match loc {
        ResourceLocation::Local => fetch_resource_local(resource),
        ResourceLocation::Remote => fetch_resource_remote(resource),
    }
}

fn fetch_resource_local(resource: &str) -> String {
    let resource_dir = local::local_resource_path();
    let resource = resource
        .strip_prefix(format!("{}/{}/", config::GITHUB_USER, config::GITHUB_REPO_NAME).as_str())
        .unwrap_or(resource);

    // Adjoining an absolute path replaces the existing path
    // As such, we need to account for these in the resource
    let resource = Path::new(resource.trim_start_matches('/'));
    let resource_path = resource_dir.join(resource);
    fs::read_to_string(resource_path).unwrap()
}

fn fetch_resource_remote(resource: &str) -> String {
    let resource = resource
        .strip_prefix(format!("{}/{}/", config::GITHUB_USER, config::GITHUB_REPO_NAME).as_str())
        .unwrap_or(resource);
    remote::get_remote_resource(resource, config::MAIN_BRANCH)
}
