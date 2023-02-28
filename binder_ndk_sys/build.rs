extern crate bindgen;

use bindgen::EnumVariation;
use std::env;
use std::io::Write;
use std::path::PathBuf;

fn build_stub(){
    let symbols = std::fs::read_to_string("src/symbols.txt").unwrap();
    let mut f = std::fs::File::create("src/libbinder_ndk/jni/stub.c").unwrap();
    for symbol in symbols.split("\n"){
        if symbol != ""{
            f.write(format!("void {}(){{ return; }}\n", symbol).as_bytes()).unwrap();
        }
    }
    f.flush().unwrap();

    let ndk_path = env::var("ANDROID_NDK_HOME").expect("Please set ANDROID_NDK_HOME");
    let ndk_build_path = format!("{}/ndk-build", ndk_path);

    std::process::Command::new(ndk_build_path)
        .current_dir("src/libbinder_ndk")
        .output()
        .unwrap();
    
    let target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap();
    let target_arch = match target_arch.as_str() {
        "x86" => "x86",
        "x86_64" => "x86_64",
        "aarch64" => "arm64-v8a",
        "arm" => "armeabi-v7a",
        _ => panic!("Unsupported target architecture"),
    };
    let mut path = std::env::current_dir().unwrap();
    path = path.join("src/libbinder_ndk/libs").join(target_arch);
    println!("cargo:rustc-link-search={}", path.to_str().unwrap());
    println!("cargo:rustc-link-lib=binder_ndk");
}

fn main() {
    println!("cargo:rerun-if-changed=src/BinderBindings.hpp");
    println!("cargo:rerun-if-changed=src/symbols.txt");

    build_stub();

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
