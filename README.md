<!-- markdownlint-disable-file MD041 -->
<!-- markdownlint-disable-file MD033 -->

<div align="center">

# `🗜️ gdeflate-rs`

**A library for compressing and decompressing the GDeflate format 🦀**

[![crates][crates-badge]][crates-url]
[![license][license-badge]][license-url]
[![dependency-status][dependency-badge]][dependency-url]

[crates-badge]: https://img.shields.io/crates/v/gdeflate.svg
[crates-url]: https://crates.io/crates/gdeflate

[license-badge]: https://img.shields.io/badge/License-MIT/Apache_2.0-blue.svg
[license-url]: LICENSE-MIT


[dependency-badge]: https://deps.rs/repo/github/projectkml/gdeflate-rs/status.svg
[dependency-url]: https://deps.rs/repo/github/projectkml/gdeflate-rs

</div>

```TOML
[dependencies]
gdeflate = "0.3.0"
```

Use the `compress` and `decompress` functions to compress and decompress data.

```Rust
use gdeflate::{CompressionLevel, Compressor, Decompressor};

let uncompressed_data = vec![0, 1, 2]; // your input data

let mut compressor = Compressor::new(CompressionLevel::Level12).unwrap();
let result = compressor.compress(&uncompressed_data, 65536).unwrap();

let mut decompressor = Decompressor::new().unwrap();
let reconstructed_data = decompressor.decompress(&result).unwrap();

assert_eq!(&uncompressed_data, &reconstructed_data);
```

## 🚨 Warning 🚨

This library is still experimental and only supports single-threaded compression and decompression at the moment.
