extern crate bindgen;
extern crate protoc_grpcio;

use std::env;
use std::path::PathBuf;

fn main() {
    // Compile protobuf service
    let proto_root = "src/proto";
    println!("cargo:rerun-if-changed={}", proto_root);
    protoc_grpcio::compile_grpc_protos(&["echo.proto"], &[proto_root], &proto_root)
        .expect("Failed to compile gRPC definitions!");
    // Look for libraries in the lib/ directory
    println!("cargo:rustc-link-search=./lib");

    // Link libc++ (a macOS thing) and libwiredtiger
    println!("cargo:rustc-link-lib=wiredtiger");
    println!("cargo:rustc-link-lib=c++");

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
