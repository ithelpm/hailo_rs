use std::env;
use std::path::PathBuf;

fn main() {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let workspace_root = manifest_dir.parent().unwrap_or(&manifest_dir).to_path_buf();
    let sysroot_lib = manifest_dir.join("aarch64_sysroot/lib");
    let sysroot_inc = manifest_dir.join("aarch64_sysroot/include");

    // Library search paths — sysroot first (cross-compilation), then system paths (native).
    println!("cargo:rustc-link-search=native={}", sysroot_lib.display());
    println!("cargo:rustc-link-search=native=/usr/aarch64-linux-gnu/lib");
    println!("cargo:rustc-link-search=native=/usr/lib/aarch64-linux-gnu");
    println!("cargo:rustc-link-search=native=/usr/lib/");

    println!("cargo:rerun-if-changed=wrapper.h");

    // Link HailoRT runtime library.
    println!("cargo:rustc-link-lib=hailort");

    // Generate Rust FFI bindings from the HailoRT C header.
    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_arg("-I/usr/include")
        .clang_arg("-I/usr/aarch64-linux-gnu/include")
        .clang_arg(format!("-I{}", workspace_root.display()))
        .clang_arg(format!("-I{}", sysroot_inc.display()))
        .derive_debug(true)
        .derive_default(true)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Failed to generate HailoRT bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("hailort_bindings.rs"))
        .expect("Failed to write bindings");
}
