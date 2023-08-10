extern crate bindgen;

use anyhow::Result;
use bindgen::EnumVariation;
use std::env;
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;

const CARGO_CONTENT: &str = r#"
[package]
name = "binder_ndk"
authors = ["Android"]
version = "1.0.0"
edition = "2021"
rust-version = "1.67"

[lib]
crate-type = ["cdylib"]
"#;

fn build_stub() -> Result<()> {
    let symbols = std::fs::read_to_string("src/symbols.txt")?;
    let outdir = env::var("OUT_DIR")?;
    let project_path = PathBuf::from(&outdir).join("libbinder_ndk");
    if project_path.exists() {
        std::fs::remove_dir_all(&project_path)?;
    }
    std::fs::create_dir(&project_path)?;

    let project_cargo_path = project_path.join("Cargo.toml");
    std::fs::File::create(&project_cargo_path)?;
    std::fs::write(&project_cargo_path, CARGO_CONTENT)?;
    let src_path = project_path.join("src");
    std::fs::create_dir_all(&src_path)?;
    let mut f = std::fs::File::create(src_path.join("lib.rs"))?;
    for symbol in symbols.split("\n") {
        if !symbol.is_empty() {
            f.write_all(format!("#[no_mangle]\npub extern fn {}() {{}}\n", symbol).as_bytes())?;
        }
    }
    f.flush()?;

    let target = env::var("TARGET")?;
    Command::new("cargo")
        .arg("build")
        .arg("--target")
        .arg(&target)
        .arg("--manifest-path")
        .arg(project_cargo_path)
        .arg("--target-dir")
        .arg(&outdir)
        .current_dir(&project_path)
        .status()?;

    // we always use debug build for stub due to speed!
    println!(
        "cargo:rustc-link-search={}",
        format!("{}/{}/{}", outdir, target, "debug")
    );
    println!("cargo:rustc-link-lib=binder_ndk");

    Ok(())
}

fn main() {
    println!("cargo:rerun-if-changed=src/BinderBindings.hpp");
    println!("cargo:rerun-if-changed=src/symbols.txt");

    build_stub().unwrap();

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("src/BinderBindings.hpp")
        .clang_arg("-Isrc/include_cpp")
        .clang_arg("-Isrc/include_ndk")
        .clang_arg("-Isrc/include_platform")
        .clang_arg("-std=c++17")
        .default_enum_style(EnumVariation::Rust {
            non_exhaustive: true,
        })
        .constified_enum("android::c_interface::consts::.*")
        .allowlist_type("android::c_interface::.*")
        .allowlist_type("AStatus")
        .allowlist_type("AIBinder_Class")
        .allowlist_type("AIBinder")
        .allowlist_type("AIBinder_Weak")
        .allowlist_type("AIBinder_DeathRecipient")
        .allowlist_type("AParcel")
        .allowlist_type("binder_status_t")
        .allowlist_function(".*")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
