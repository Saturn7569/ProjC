use std::{fs, path::{Path, PathBuf}, process::Command};

use colored::Colorize;

use crate::config::Config;

pub fn compile_c_file(path:&PathBuf) -> Result<(), String> {
    //let f_path = Path::new(&path);
    //let file_name = f_path.file_stem().map(|f| f.to_str().unwrap()).unwrap_or("");
    println!(
        "{} {}...",
        "Compiling".green().bold().underline(),
        path.file_stem().and_then(|f| f.to_str()).unwrap().underline(),
    );
    let _ = Command::new("gcc")
        .args(&["-c", &path.to_string_lossy().to_string(), "-o", &format!("target/deps/{}.o", path.file_stem().and_then(|f| f.to_str()).unwrap())])
        .status()
        .map_err(|err| {format!(
            "{}", err
        )})?;
    Ok(())
}

pub fn build_o_files(conf:&Config) -> Result<(), String> {
    for _ in &conf.workspace.source_dirs {
        let source_files = fs::read_dir("src").unwrap()
            .filter_map(|entry| entry.ok())
            .filter(|entry| entry.path().extension().map(|ext| ext == "c").unwrap_or(false));
        for file in source_files {
            let pth = file.path();
            compile_c_file(&pth)?;
        }
    }

    Ok(())
}

pub fn link_all(conf:&Config) -> Result<(), String> {
    if !Path::new("target/deps").exists() {
        return Err(format!(
            "{} {} {}",
            "Directory".truecolor(128, 128, 128),
            "target/deps".underline(),
            "does not exist".truecolor(128, 128, 128)
        ))
    }

    println!(
        "{} {}...",
        "Building".green().bold().underline(),
        conf.workspace.exe_name.underline()
    );

    let _ = Command::new("gcc")
        .args(&["-o", &format!("target/{}", conf.workspace.exe_name), "target/deps/*.o"])
        .status()
        .map_err(|err| {format!(
            "{}", err
    )})?;

    Ok(())
}

pub fn build_all(conf:&Config) -> Result<(), String> {
    fs::create_dir_all("target/deps").map_err(|err| {format!(
        "{}", err
    )})?;

    build_o_files(conf)?;
    Ok(())
}

pub fn clean_target() -> Result<(), String> {
    if !Path::new("target").exists() {
        return Err("Nothing to clean".to_string());
    }

    println!("{} target...", "Cleaning".green().bold().underline());
    fs::remove_dir_all("target").map_err(|err| format!("Failed to delete target:\n{}", err))?;
    println!("{} all", "Cleaned".green().bold().underline());

    Ok(())
}