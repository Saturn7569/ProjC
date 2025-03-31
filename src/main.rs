use args::{Cli, Commands};
use build::{build_all, clean_target, link_all};
use clap::Parser;
use colored::*;
use config::Config;
use new_project::{create_directories, write_to_files};

mod args;
mod config;
mod new_project;
mod build;

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::New { name }) => {
            println!(
                "Creating new {} application named \'{}\'...",
                "binary".truecolor(128, 128, 128),
                name.green(),
            );

            match create_directories(&name) {
                Err(err) => {
                    eprintln!(
                        "{}: {}",
                        "ERROR".red().bold().underline(),
                        err,
                    );
                    return;
                },
                _ => {},
            }

            match write_to_files(&name) {
                Err(err) => {
                    eprintln!(
                        "{}: {}",
                        "ERROR".red().bold().underline(),
                        err,
                    );
                    return;
                },
                _ => {},
            }
        },

        Some(Commands::Build) => {
            let conf = Config::new("projc.toml").map_err(|err| {
                eprintln!(
                    "{}: {}",
                    "ERROR".red().bold().underline(),
                    err,
                );
                return;
            }).unwrap();

            build_all(&conf).map_err(|err| {
                eprintln!(
                    "{}: {}",
                    "ERROR".red().bold().underline(),
                    err,
                );
                return;
            }).unwrap();

            link_all(&conf).map_err(|err| {
                eprintln!(
                    "{}: {}",
                    "ERROR".red().bold().underline(),
                    err,
                );
                return;
            }).unwrap();
        },

        Some(Commands::Clean) => {
            clean_target().map_err(|err| {
                eprintln!(
                    "{}: {}",
                    "ERROR".red().bold().underline(),
                    err,
                );
                return;
            }).unwrap();
        },

        None => {
            eprintln!(
                "{}: Invalid run command!\ntry {} for help",
                "ERROR".red().bold().underline(),
                "--help".bold(),
            );
            return;
        },
    }
}