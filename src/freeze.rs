use super::{
    config,
    input::INPUT_RE,
    remote,
    resource::{self, ResourceLocation},
};
use chrono::prelude::*;
use regex::Captures;

pub fn expand_input_paths(contents_raw: String, loc: &ResourceLocation) -> String {
    // We want to expand/evaluate lines in LaTeX like `\input{...}`
    let expanded = INPUT_RE
        .replace_all(&contents_raw, |caps: &Captures| {
            let input_path = caps.name("path").unwrap().as_str();
            fetch_resource(input_path, loc)
        })
        .to_string();
    add_version_metadata(expanded, loc)
}

fn fetch_resource(input_path: &str, loc: &ResourceLocation) -> String {
    let resource_path = input_path
        .split(config::GITHUB_REPO_NAME)
        .last()
        .expect("Cannot find tex-macros/ repo in input path");
    resource::fetch_resource(resource_path, loc)
}

fn add_version_metadata(contents_raw: String, loc: &ResourceLocation) -> String {
    let local_dt: DateTime<Local> = Local::now();
    let formatted_date = local_dt.format("%I:%M %p on %A, %e %B, %Y %Z").to_string();

    let mut contents = String::new();
    contents.push_str(format!("% Frozen version at {}\n\n", formatted_date).as_str());

    if loc == &ResourceLocation::Remote {
        let latest_commit = remote::latest_commit_hash();
        contents.pop(); // Remove other new line if remote info added
        contents.push_str(format!("% At commit version {} \n\n", latest_commit).as_str());
    }

    contents.push_str(&contents_raw);

    contents
}
