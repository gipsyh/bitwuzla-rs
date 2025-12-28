extern crate giputils;
use giputils::build::copy_build;
use std::{io, process::Command};

fn main() -> io::Result<()> {
    giputils::build::git_submodule_update()?;
    println!("cargo:rerun-if-changed=./bitwuzla");
    let cb_path = copy_build("bitwuzla", |src| {
        let status = Command::new("python3")
            .arg("configure.py")
            .current_dir(src)
            .status()?;
        if !status.success() {
            return Err(io::Error::other(format!(
                "configure.py failed with status: {}",
                status
            )));
        }
        let status = Command::new("meson")
            .arg("compile")
            .current_dir(src.join("build"))
            .status()?;
        if !status.success() {
            return Err(io::Error::other(format!(
                "meson compile failed with status: {}",
                status
            )));
        }
        Ok(())
    })?;
    println!(
        "cargo:rustc-link-search=native={}",
        cb_path.join("build").join("src").display()
    );
    println!(
        "cargo:rustc-link-search=native={}",
        cb_path.join("build").join("src").join("lib").display()
    );
    println!("cargo:rustc-link-lib=static=bitwuzla");
    println!("cargo:rustc-link-lib=static=bzlautil");
    println!("cargo:rustc-link-lib=static=bitwuzlabb");
    println!("cargo:rustc-link-lib=static=bitwuzlabv");
    println!("cargo:rustc-link-lib=static=bitwuzlals");
    println!("cargo:rustc-link-lib=static=bzlarng");
    #[cfg(target_os = "linux")]
    println!("cargo:rustc-link-lib=dylib=stdc++");
    #[cfg(target_os = "macos")]
    println!("cargo:rustc-link-lib=dylib=c++");
    println!("cargo:rustc-link-lib=dylib=gmp");
    println!("cargo:rustc-link-lib=dylib=mpfr");
    Ok(())
}
