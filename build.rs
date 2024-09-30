use std::env;
use std::path::PathBuf;

fn main() {
    let vendor_path = PathBuf::from("vendor")
        .join("spral-2024.05.08")
        .canonicalize()
        .expect("cannot canonicalize path");

    if let Some(sparal_lib_dir) = env::var("SPRAL_LIBRARY_DIR").ok() {
        println!("cargo:rustc-link-search=native={}", sparal_lib_dir);
    }
    if cfg!(feature = "shared_libraries") {
        println!("cargo:rustc-link-lib=dylib=spral");
    } else if cfg!(feature = "static_libraries") {
        println!("cargo:rustc-link-lib=static=spral");
    }

    let headers_path = vendor_path.join("include").join("spral.h");
    let headers_path_str = headers_path.to_str().expect("Path is not a valid string");

    let bindings = bindgen::Builder::default()
        .header(headers_path_str)
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
