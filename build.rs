extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    // Tell cargo to tell rustc to link the system bzip2
    // shared library.
    println!("cargo:rustc-link-lib=wiredtiger");

    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=wrapper.h");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let wt_top = PathBuf::from(env::var("WT_TOP").unwrap());
    let wt_inc = wt_top.join("build_posix");
    let wt_lib = wt_top.join("build_posix").join(".libs");
    let wt_inc_str: String = wt_inc.as_os_str().to_str().unwrap().to_string();
    let wt_lib_str: String = wt_lib.as_os_str().to_str().unwrap().to_string();
    let inc_arg = format!("-I{}", wt_inc_str);
    let lib_arg = format!("-L{}", wt_lib_str);

    eprintln!("INC_ARG: {}\n", inc_arg);
    eprintln!("LIB_ARG: {}\n", lib_arg);

    // This doesn't seem to work, we need to set the environment path before
    // calling "cargo test".
    //env::set_var("LD_LIBRARY_PATH", "/Users/dda/wt/git/develop/build_posix/.libs");
    //env::set_var("DYLD_LIBRARY_PATH", "/Users/dda/wt/git/develop/build_posix/.libs");
    println!("cargo:rustc-link-search=native=/home/ubuntu/work/rust/wt-dev-rust/build_posix/.libs");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("wrapper.h")
        .clang_arg(inc_arg)
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

