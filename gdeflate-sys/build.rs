use std::fs;
use std::path::Path;

fn generate_bindings() {
    let bindings = bindgen::Builder::default()
        .header("vendor/libdeflate/libdeflate.h")
        .rustfmt_bindings(true)
        .size_t_is_usize(true)
        .allowlist_function("libdeflate_.*")
        .layout_tests(false)
        .generate()
        .expect("Failed to generate bindings");

    fs::create_dir_all("gen").unwrap();
    bindings.write_to_file("gen/bindings.rs").unwrap();
}

fn main() {
    let mut build = cc::Build::new();

    build
        .include("vendor/libdeflate/libdeflate.h")
        .file("vendor/libdeflate/lib/gdeflate_compress.c")
        .file("vendor/libdeflate/lib/gdeflate_decompress.c")
        .compile("gdeflate_sys_cc");

    generate_bindings();
}