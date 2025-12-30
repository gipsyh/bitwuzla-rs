extern crate giputils;
#[cfg(feature = "vendor")]
use giputils::build::copy_build;
use std::{io, process::Command};

#[cfg(feature = "vendor")]
fn build_vendor() -> io::Result<()> {
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

#[cfg(not(feature = "vendor"))]
fn link_system() -> io::Result<()> {
    if let Ok(output) = Command::new("pkg-config")
        .args(["--libs", "--cflags", "bitwuzla"])
        .output()
        && output.status.success()
    {
        let flags = String::from_utf8_lossy(&output.stdout);
        for flag in flags.split_whitespace() {
            if let Some(lib) = flag.strip_prefix("-l") {
                println!("cargo:rustc-link-lib={}", lib);
            } else if let Some(path) = flag.strip_prefix("-L") {
                println!("cargo:rustc-link-search=native={}", path);
            }
        }
        return Ok(());
    }
    println!("cargo:rustc-link-lib=dylib=bitwuzla");
    #[cfg(target_os = "linux")]
    println!("cargo:rustc-link-lib=dylib=stdc++");
    #[cfg(target_os = "macos")]
    println!("cargo:rustc-link-lib=dylib=c++");
    println!("cargo:rustc-link-lib=dylib=gmp");
    println!("cargo:rustc-link-lib=dylib=mpfr");
    Ok(())
}

fn main() -> io::Result<()> {
    #[cfg(feature = "vendor")]
    build_vendor()?;

    #[cfg(not(feature = "vendor"))]
    link_system()?;

    Ok(())
}
