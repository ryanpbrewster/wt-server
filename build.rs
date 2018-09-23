extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    // Look for libraries in the lib/ directory
    println!("cargo:rustc-link-search=./lib");

    // Link libc++ (a macOS thing) and libwiredtiger
    println!("cargo:rustc-link-lib=c++");
    println!("cargo:rustc-link-lib=wiredtiger");

    let bindings = bindgen::Builder::default()
        // Do not generate comments. They are invalid and cause compilation errors.
        .generate_comments(false)
        // The input header we would like to generate bindings for.
        .header("lib/wiredtiger.h")
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
