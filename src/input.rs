use lazy_static::lazy_static;
use regex::{Captures, Regex};

use crate::config;
use crate::file::LocalResource;
use crate::resource::fetch_resource;

lazy_static! {
    pub static ref INPUT_RE: Regex = Regex::new(
        format!(
            r"\\input\{{(?:(?:{})/(?:{})/)?(?P<path>.+)\}}",
            config::GITHUB_USER,
            config::GITHUB_REPO_NAME
        )
        .as_str()
    )
    .unwrap();
}

pub fn sourced_files(cls: LocalResource) -> Vec<LocalResource> {
    let contents = fetch_resource(cls.resource_path.as_str(), cls.resource_location);

    INPUT_RE
        .captures_iter(&contents)
        .map(|caps: Captures| LocalResource {
            resource_path: format!(
                "{}/{}/{}",
                config::GITHUB_USER,
                config::GITHUB_REPO_NAME,
                caps.name("path").unwrap().as_str()
            ),
            resource_location: cls.resource_location,
            template: None,
        })
        .collect()
}
