use clap::{crate_authors, crate_version, ArgAction, Parser, Subcommand};
use std::{path::Path, process};

mod config;
mod file;
mod freeze;
mod input;
mod local;
mod remote;
mod resource;
mod sync;
mod texmf;

use config::*;
use file::{LocalResource, LocalTemplate};
use resource::{fetch_resource, ResourceLocation};

// TODO:
//   - better logging
//   - add version history to readme
//   - fix broken pipe: https://stackoverflow.com/a/65760807/12069968
//   - fix -c freeze crash
//   - warn if -l passed without -c or something (-l only relevant with other things)
//   - do not allow freeze with other options
//   - allow freeze options (e.g., don't assume the user wants to use freeze with -c)
//   - more idiomatic result handling
//   - allow freeze to accept commit id
//   - class option local with no texmf
//   - freeze more than just class
//   - no-option default?
//   - decouple from tex-macros repo as much as possible
//   - author
//   - general class option?
//   - bibligraphy file option
//   - formal letter option
//   - figure option
//   - poi option

#[derive(Parser)]
#[command(
    name = "mktex",
    author = crate_authors!("\n"),
    version = crate_version!(),
    allow_missing_positional = true,
    subcommand_negates_reqs = true,
    arg_required_else_help = true,
)]
/// Make LaTeX projects with custom macros.
struct Cli {
    /// Output file name
    #[arg(
        action = ArgAction::Set,
        num_args = 0..=1,
        value_name = "file name",
        default_value = "document.tex",
    )]
    file: Option<String>,

    /// Output directory
    #[arg(
        action = ArgAction::Set,
        num_args = 0..=1,
        value_name = "output directory",
        default_value = ".",
    )]
    dir: Option<String>,

    /// Try to use local files rather than remote
    #[arg(
        short = 'l',
        long = "local",
        action = ArgAction::SetTrue,
        num_args = 0,
    )]
    local: Option<bool>,

    /// Use article class
    #[arg(
        short = 'a',
        long = "article",
        action = ArgAction::SetTrue,
        num_args = 0,
    )]
    article: Option<bool>,

    #[arg(
        short = 'c',
        long = "class",
        action = ArgAction::SetTrue,
        num_args = 0,
        hide = true,
    )]
    class: Option<bool>,

    /// Use letter class
    #[arg(
        short = 'L',
        long = "letter",
        action = ArgAction::SetTrue,
        num_args = 0,
    )]
    letter: Option<bool>,

    /// Use make letter formal
    #[arg(
        short = 'f',
        long = "formal",
        action = ArgAction::SetTrue,
        num_args = 0,
        requires("letter"),
    )]
    formal: Option<bool>,

    /// Use beamer class
    #[arg(
        short = 'b',
        long = "beamer",
        action = ArgAction::SetTrue,
        num_args = 0,
    )]
    beamer: Option<bool>,

    /// Do the process without writing anything
    #[arg(
        short = 'n',
        long = "dry-run",
        action = ArgAction::SetTrue,
        num_args = 0,
    )]
    dry_run: Option<bool>,

    #[command(subcommand)]
    command: Option<Commands>,

    // Trailing arguments
    // https://stackoverflow.com/a/77512007/
    #[arg(trailing_var_arg = true, allow_hyphen_values = true, hide = true)]
    rem: Vec<String>,
}

#[derive(Subcommand)]
enum Commands {
    /// Freeze latest article class files
    Freeze,
    /// Print local texmf directory
    Texmf,
}

fn main() {
    let mut cli = Cli::parse();

    let resource_location = if let Some(local) = cli.local {
        if local {
            ResourceLocation::Local
        } else {
            ResourceLocation::Remote
        }
    } else {
        ResourceLocation::Remote
    };

    // Parse subcommands and exit
    match cli.command {
        Some(Commands::Freeze) => {
            let cls_contents = fetch_resource(CLS_RESOURCE, &resource_location);
            println!(
                "{}",
                freeze::expand_input_paths(cls_contents, &resource_location)
            );
            process::exit(0);
        }
        Some(Commands::Texmf) => {
            if let Some(texmf_path) = texmf::texmf() {
                println!("{}", texmf_path.display());
            } else {
                eprintln!("[ERROR] Could not find local texmf directory");
                process::exit(1);
            }
            process::exit(0);
        }
        None => {}
    }

    let mut opt_used = false;
    let out_dir = cli.dir.unwrap().to_string();
    let out_file = cli.file.unwrap().to_string();
    let dry_run = if let Some(dry_run) = cli.dry_run {
        dry_run
    } else {
        false
    };

    // Make article class file
    if let Some(use_class) = cli.class {
        if use_class {
            opt_used = true;
            eprintln!("[WARN] --class option is deprecated since v1.8.1.  Use --article instead.");
            cli.article = Some(true);
        }
    }
    if let Some(use_article) = cli.article {
        if use_article {
            opt_used = true;
            let cls = LocalResource {
                resource_path: CLS_RESOURCE.to_string(),
                resource_location: &resource_location,
                template: Some(LocalTemplate {
                    template_path: TMPL_RESOURCE.to_string(),
                    out_dir: &out_dir,
                    out_file: &out_file,
                }),
            };
            file::write_resource(cls.clone(), dry_run);

            // Write sourced files required by the class
            println!("[INFO] Checking sync status of local source files...");
            for source_file in input::sourced_files(cls) {
                file::write_resource(source_file, dry_run)
            }
            println!("[INFO] Done")
        }
    };

    // Make letter file
    if let Some(use_letter) = cli.letter {
        if use_letter {
            opt_used = true;
            let template = if let Some(formal_letter) = cli.formal {
                if formal_letter {
                    LTR_FML_TMPL_RESOURCE
                } else {
                    LTR_TMPL_RESOURCE
                }
            } else {
                LTR_TMPL_RESOURCE
            };
            let cls = LocalResource {
                resource_path: LTR_RESOURCE.to_string(),
                resource_location: &resource_location,
                template: Some(LocalTemplate {
                    template_path: template.to_string(),
                    out_dir: &out_dir,
                    out_file: &out_file,
                }),
            };
            file::write_resource(cls.clone(), dry_run);

            // Write sourced files required by the class
            println!("[INFO] Checking sync status of local source files...");
            for source_file in input::sourced_files(cls) {
                file::write_resource(source_file, dry_run)
            }
            println!("[INFO] Done")
        }
    };

    // Make beamer file
    if let Some(use_beamer) = cli.beamer {
        if use_beamer {
            opt_used = true;
            // Custom Beamer theme files
            for file in [
                BMR_THEME_COLOUR,
                BMR_THEME_INNER,
                BMR_THEME_OUTER,
                BMR_THEME_MAIN,
            ] {
                let theme_file = Path::new(BMR_THEME_PATH).join(file);
                let sty = LocalResource {
                    resource_path: theme_file.display().to_string(),
                    resource_location: &resource_location,
                    template: None,
                };
                file::write_resource(sty, dry_run);
            }

            // Main Beamer class file
            let cls = LocalResource {
                resource_path: BMR_RESOURCE.to_string(),
                resource_location: &resource_location,
                template: Some(LocalTemplate {
                    template_path: BMR_TMPL_RESOURCE.to_string(),
                    out_dir: &out_dir,
                    out_file: &out_file,
                }),
            };
            file::write_resource(cls, dry_run);
        }
    }

    // Check if dry run is given without other options
    if dry_run && !opt_used {
        eprintln!("[ERROR] --dry-run argument passed without another option.  Cannot dry run with prespecified no intent.  Use -h for help.");
        process::exit(1);
    }

    // Check that file is parsed with some other options
    if !opt_used {
        eprintln!("[ERROR] Must used on of the command line options if a file is specified.  Use --h for help.  File specified: {:?}", &out_file);
        process::exit(1);
    }

    // Exit programme
    process::exit(0);
}
