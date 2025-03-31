use std::{fs, path::Path};

use colored::*;

pub fn create_directories(proj_name:&String) -> Result<(), String> {
    if Path::new(proj_name).exists() {
        return Err(format!(
            "{} {} {}",
            "Failed to create directory".truecolor(128, 128, 128),
            proj_name.underline(),
            "because it already exists".truecolor(128, 128, 128),
        ))
    }

    match fs::create_dir_all(format!("{}/src", proj_name)) {
        Err(err) => {
            return Err(format!("{} {}\n{}", "Failed to create directory", proj_name.underline(), err))
        }
        _ => {},
    }

    Ok(())
}

pub fn write_to_files(proj_name:&String) -> Result<(), String> {
    if !Path::new(proj_name).exists() {
        return Err(format!(
            "{} {} {}",
            "Failed to load directory".truecolor(128, 128, 128),
            proj_name.underline(),
            "because it doesn't exist".truecolor(128, 128, 128),
        ))
    }

    let d_main_c = "int main() {return 0;}";

    match fs::write(format!("{}/src/main.c", proj_name), d_main_c) {
        Err(err) => {
            return Err(format!(
                "{}",
                err,
            ));
        },
        _ => {},
    }

    let d_proj_toml = format!("[workspace]\nsource_dirs = [\"src\"]\nexe_name = \"{}\"", proj_name);

    match fs::write(format!("{}/projc.toml", proj_name), d_proj_toml) {
        Err(err) => {
            return Err(format!(
                "{}",
                err,
            ));
        },
        _ => {},
    }

    Ok(())
}