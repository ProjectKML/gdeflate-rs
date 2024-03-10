use crate::{CompressionLevel, Compressor, Decompressor};

fn compress_and_uncompress(
    uncompressed_bytes: &[u8],
    compression_level: CompressionLevel,
    tile_size: usize,
) {
    let mut compressor = Compressor::new(compression_level).unwrap();

    let result = compressor.compress(uncompressed_bytes, tile_size).unwrap();

    let mut decompressor = Decompressor::new().unwrap();
    let reconstructed_bytes = decompressor.decompress(&result).unwrap();

    assert_eq!(uncompressed_bytes, reconstructed_bytes);
}

#[test]
fn simple_bytes() {
    for i in 0..10 {
        let values = (0..(1 << i) * 1024)
            .into_iter()
            .map(|i| (i & 255) as u8)
            .collect::<Vec<_>>();
        compress_and_uncompress(&values, CompressionLevel::Level12, 65536);
    }
}
