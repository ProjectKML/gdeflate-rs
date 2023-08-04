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
gdeflate = "0.1.0"
```



```Rust
let bytes = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
let compressed_bytes = gdeflate::compress(&bytes, CompressionLevel::Level12).unwrap();
assert_eq!(bytes, &gdeflate::decompress(&compressed_bytes, bytes.len()));
```
