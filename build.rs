use std::env;

fn main() {
    cc::Build::new()
        .include("vendor/libdeflate")
        .file("vendor/libdeflate/lib/gdeflate_compress.c")
        .file("vendor/libdeflate/lib/gdeflate_decompress.c")
        .file("vendor/libdeflate/lib/utils.c")
        .compile("gdeflate_c");

    let target = env::var("TARGET").unwrap();

    let mut build = cxx_build::bridge("src/lib.rs");

    if target.contains("gnu") || target.contains("darwin") {
        build.flag("-std=c++20");
    }

    build
        .include("src")
        .include("vendor/libdeflate")
        .file("src/GDeflateCompress.cpp")
        .file("src/GDeflateDecompress.cpp")
        .compile("gdeflate_cxx");
}
