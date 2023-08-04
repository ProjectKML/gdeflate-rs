fn main() {
    cxx_build::bridge("src/lib.rs")
        .include("src")
        .file("src/GDeflateCompress.cpp")
        .file("src/GDeflateDecompress.cpp")
        .include("vendor/libdeflate")
        .file("vendor/libdeflate/lib/gdeflate_compress.c")
        .file("vendor/libdeflate/lib/gdeflate_decompress.c")
        .file("vendor/libdeflate/lib/utils.c")
        .compile("gdeflate_cxx");
}
