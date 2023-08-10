fn main() {
    cc::Build::new()
        .include("vendor/libdeflate")
        .file("vendor/libdeflate/lib/gdeflate_compress.c")
        .file("vendor/libdeflate/lib/gdeflate_decompress.c")
        .file("vendor/libdeflate/lib/utils.c")
        .compile("gdeflate_c");

    cxx_build::bridge("src/lib.rs")
        .include("src")
        .include("vendor/libdeflate")
        .file("src/GDeflateCompress.cpp")
        .file("src/GDeflateDecompress.cpp")
        .compile("gdeflate_cxx");
}
