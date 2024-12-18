#![feature(exit_status_error)]

extern crate giputils;
use giputils::build::copy_build;
use std::process::Command;

fn main() -> Result<(), String> {
    Command::new("git")
        .args(["submodule", "update", "--init"])
        .status()
        .unwrap();
    println!("cargo:rerun-if-changed=./bitwuzla");
    let cb_path = copy_build("bitwuzla", |src| {
        Command::new("python3")
            .arg("configure.py")
            .current_dir(src)
            .status()
            .map_err(|e| e.to_string())?
            .exit_ok()
            .map_err(|e| e.to_string())?;
        Command::new("meson")
            .arg("compile")
            .current_dir(src.join("build"))
            .status()
            .map_err(|e| e.to_string())?
            .exit_ok()
            .map_err(|e| e.to_string())
    })?;
    println!(
        "cargo:rustc-link-search=native={}",
        cb_path.join("build").join("src").display()
    );
    println!("cargo:rustc-link-lib=static=bitwuzla");
    println!("cargo:rustc-link-lib=dylib=stdc++");
    Ok(())
}
