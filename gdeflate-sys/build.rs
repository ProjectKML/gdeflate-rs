#[cfg(feature = "generate_bindings")]
use bindgen::{Builder, Formatter};

#[cfg(feature = "generate_bindings")]
fn generate_bindings() {
    let bindings = Builder::default()
        .header("vendor/libdeflate/libdeflate.h")
        .formatter(Formatter::Rustfmt)
        .size_t_is_usize(true)
        .allowlist_function("libdeflate_.*")
        .layout_tests(false)
        .generate()
        .expect("Failed to generate bindings");

    std::fs::create_dir_all("gen").unwrap();
    bindings.write_to_file("gen/bindings.rs").unwrap();
}

fn main() {
    let mut build = cc::Build::new();

    build
        .include("vendor/libdeflate")
        .file("vendor/libdeflate/lib/gdeflate_compress.c")
        .file("vendor/libdeflate/lib/gdeflate_decompress.c")
        .file("vendor/libdeflate/lib/utils.c");

    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    build.file("vendor/libdeflate/lib/x86/cpu_features.c");

    #[cfg(any(target_arch = "arm", target_arch = "aarch64"))]
    build.file("vendor/libdeflate/lib/arm/cpu_features.c");

    build.compile("gdeflate_sys_cc");

    #[cfg(feature = "generate_bindings")]
    generate_bindings();
}
