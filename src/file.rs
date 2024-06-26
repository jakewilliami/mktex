use super::{
    config, local, remote,
    resource::{fetch_resource, ResourceLocation},
    sync, texmf,
};
// use super::{config, file::LocalResource, resource::fetch_resource};
use dialoguer::Confirm;
use lazy_static::lazy_static;
use regex::Regex;
use std::{
    fs,
    path::{Path, PathBuf},
};

lazy_static! {
    pub static ref DOCUMENT_CLASS_RE: Regex =
        Regex::new(r"(?<documentclass>\\documentclass(\[(?<opts>.+)\])?\{(?<class>\w+)\})")
            .unwrap();
}

#[derive(Clone)]
pub struct LocalTemplate<'a> {
    pub template_path: String,
    pub out_dir: &'a String,
    pub out_file: &'a String,
}

#[derive(Clone)]
pub struct LocalResource<'a> {
    pub resource_path: String,
    pub resource_location: &'a ResourceLocation,
    pub template: Option<LocalTemplate<'a>>,
}

impl LocalTemplate<'_> {
    fn out_file(&self) -> PathBuf {
        let mut path = PathBuf::from(&self.out_dir);
        path.push(self.out_file);
        path
    }
}

fn write_template(file: LocalResource, dry_run: bool) {
    let template = file.template.unwrap();

    // Make template in target dir
    let out_file = template.out_file();

    // Check that we are not overwriting a file!
    if out_file.exists()
        && !dry_run
        && !Confirm::new()
            .with_prompt(format!(
                "{:?} already exists.  Would you like to overwrite it?",
                &out_file
            ))
            .interact()
            .unwrap()
    {
        println!("[WARN] File {:?} already exists", &out_file);
        return;
    }

    if dry_run {
        println!(
            "[INFO] Would have written template {:?} to {:?}",
            &template.template_path, &out_file
        );
    } else {
        // Write the template file to the specified directory
        let tmpl_contents = fetch_resource(template.template_path.as_str(), file.resource_location);
        let tmpl_contents = add_template_resource_version(tmpl_contents, file.resource_location);

        println!(
            "[INFO] Writing template {:?} to {:?}",
            &template.template_path, &out_file
        );
        fs::write(out_file, tmpl_contents).unwrap();
    }
}

pub fn write_resource(file: LocalResource, dry_run: bool) {
    let file_name = Path::new(&file.resource_path);
    let file_name = file_name
        .strip_prefix(format!(
            "{}/{}/",
            config::GITHUB_USER,
            config::GITHUB_REPO_NAME
        ))
        .unwrap_or(file_name)
        .strip_prefix(config::RESOURCE_PARENT)
        .unwrap_or(file_name)
        .to_path_buf();

    // Ensure parent path exists
    let mut local_path = texmf::texmf_local_resources();
    let file_parent = &file_name.parent();
    if let Some(file_parent) = file_parent {
        local_path.push(file_parent)
    }
    if !local_path.exists() {
        if dry_run {
            println!("[INFO] Would have created the directory {:?}", &local_path);
        } else {
            println!("[INFO] Creating directory {:?}", &local_path);
            fs::create_dir_all(&local_path).unwrap();
        }
    }

    // Append file name to local resource path
    local_path.push(file_name.file_name().unwrap());

    // Write file to local texmf directory
    let contents = fetch_resource(file.resource_path.as_str(), file.resource_location);

    // Need to move file to local texmf if possible
    if !texmf::resource_in_local_texmf(&file_name) {
        if dry_run {
            println!(
                "[INFO] Would have written resource {:?} to {:?}",
                &file_name, &local_path
            );
        } else {
            println!(
                "[INFO] Writing resource {:?} to {:?}",
                &file_name, &local_path
            );
            fs::write(&local_path, &contents).unwrap();
        }
    }

    // If local (texmf) resource is not in sync with remote, ask user if we should update local
    if !sync::check_resource(&local_path, &contents) {
        println!(
            "[WARN] Local resource exists but is out of sync with remote ({:?})",
            file_name
        );
        if !dry_run {
            if Confirm::new() //::with_theme(&dialoguer::theme::ColorfulTheme::default())
                .with_prompt(format!(
                    "Would you like to update the local resource at {:?}?",
                    &local_path
                ))
                .interact()
                .unwrap()
            {
                println!(
                    "[INFO] Updating local resource {:?} at {:?}",
                    &file_name, &local_path
                );
                fs::write(&local_path, &contents).unwrap();
            } else {
                println!("[INFO] Ignoring out-of-sync local file");
            }
        }
    }

    if file.template.is_some() {
        write_template(file, dry_run);
    }
}

fn add_template_resource_version(tmpl_contents: String, loc: &ResourceLocation) -> String {
    let commit_hash = match loc {
        ResourceLocation::Local => local::latest_local_commit_hash(),
        ResourceLocation::Remote => remote::latest_commit_hash(),
    };

    DOCUMENT_CLASS_RE
        .replace(
            &tmpl_contents,
            format!("$documentclass  % class version {}", commit_hash),
        )
        .to_string()
}
