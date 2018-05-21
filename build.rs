extern crate bindgen;
extern crate gcc;
extern crate pkg_config;

use bindgen::CodegenConfig;

use std::env;
use std::path::PathBuf;

fn main() {
    //println!("cargo:rustc-link-lib=gflags");

    let gflags = pkg_config::probe_library("gflags").expect("probe gflags");

    let mut build = gcc::Build::new();
    build.file("gflags.cpp").cpp(true).flag("-std=c++11");
    for inc in gflags.include_paths {
        build.include(inc);
    }
    build.compile("gflags-binding");

    let bindings = bindgen::Builder::default()
        .clang_arg("-xc++")
        .header("gflags.h")
        .rustfmt_bindings(true)
        .blacklist_type("str_slice")
        .whitelist_function("flag_registerer")
        .whitelist_function("free_flag_registerer")
        .whitelist_function("get_commandline_option")
        .whitelist_type("std::string")
        .opaque_type("std::string")
        .with_codegen_config(CodegenConfig {
            functions: true,
            types: true,
            ..CodegenConfig::nothing()
        })
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
