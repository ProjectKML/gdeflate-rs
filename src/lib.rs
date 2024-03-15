#[cfg(test)]
mod tests;

pub mod sys {
    pub use gdeflate_sys::*;
}

use std::{ptr, slice};

use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Failed to create compressor")]
    CompressorCreationFailed,
    #[error("Failed to create decompressor")]
    DecompressorCreationFailed,
}

#[repr(i32)]
#[derive(Copy, Clone, Debug)]
pub enum CompressionLevel {
    None = 0,
    Level1 = 1,
    Level2 = 2,
    Level3 = 3,
    Level4 = 4,
    Level5 = 5,
    Level6 = 6,
    Level7 = 7,
    Level8 = 8,
    Level9 = 9,
    Level10 = 10,
    Level11 = 11,
    Level12 = 12,
}

#[derive(Copy, Clone, Debug)]
pub struct Tile {
    pub compressed_size: usize,
    pub uncompressed_size: usize,
}

#[derive(Clone, Debug)]
pub struct CompressionResult {
    pub bytes: Vec<u8>,
    pub tiles: Vec<Tile>,
    pub tile_size: usize,
}

pub struct Compressor(*mut sys::libdeflate_gdeflate_compressor);

impl Compressor {
    pub fn new(compression_level: CompressionLevel) -> Result<Self, Error> {
        let compressor =
            unsafe { sys::libdeflate_alloc_gdeflate_compressor(compression_level as _) };
        if compressor.is_null() {
            Err(Error::CompressorCreationFailed)
        } else {
            Ok(Self(compressor))
        }
    }

    pub fn compress(
        &mut self,
        uncompressed_bytes: &[u8],
        tile_size: usize,
    ) -> Result<CompressionResult, Error> {
        let num_tiles = (uncompressed_bytes.len() + tile_size - 1) / tile_size;

        let mut num_pages = 0;
        let scratch_size =
            unsafe { sys::libdeflate_gdeflate_compress_bound(self.0, tile_size, &mut num_pages) };
        assert_eq!(num_pages, 1);

        let mut scratch_buffer = vec![0u8; scratch_size];

        let mut bytes = vec![];
        let mut tiles = Vec::with_capacity(num_tiles);

        for i in 0..num_tiles {
            let tile_offset = i * tile_size;

            let mut compressed_page = sys::libdeflate_gdeflate_out_page {
                data: scratch_buffer.as_mut_ptr().cast(),
                nbytes: scratch_size,
            };

            let remaining = uncompressed_bytes.len() - tile_offset;
            let uncompressed_size = remaining.min(tile_size);

            unsafe {
                sys::libdeflate_gdeflate_compress(
                    self.0,
                    uncompressed_bytes.as_ptr().add(tile_offset).cast(),
                    uncompressed_size,
                    &mut compressed_page,
                    1,
                );

                bytes.extend_from_slice(slice::from_raw_parts(
                    compressed_page.data.cast(),
                    compressed_page.nbytes,
                ));

                tiles.push(Tile {
                    compressed_size: compressed_page.nbytes,
                    uncompressed_size,
                })
            }
        }

        Ok(CompressionResult {
            bytes,
            tiles,
            tile_size,
        })
    }
}

impl Drop for Compressor {
    fn drop(&mut self) {
        unsafe { sys::libdeflate_free_gdeflate_compressor(self.0) }
    }
}

pub struct Decompressor(*mut sys::libdeflate_gdeflate_decompressor);

impl Decompressor {
    pub fn new() -> Result<Self, Error> {
        let decompressor = unsafe { sys::libdeflate_alloc_gdeflate_decompressor() };
        if decompressor.is_null() {
            Err(Error::DecompressorCreationFailed)
        } else {
            Ok(Self(decompressor))
        }
    }

    pub fn decompress(&mut self, result: &CompressionResult) -> Result<Vec<u8>, Error> {
        let uncompressed_size = result.tiles.iter().map(|tile| tile.uncompressed_size).sum();

        let mut bytes = vec![0u8; uncompressed_size];

        let mut compressed_offset = 0;
        let mut uncompressed_offset = 0;

        for tile in &result.tiles {
            let mut compressed_page = sys::libdeflate_gdeflate_in_page {
                data: unsafe { result.bytes.as_ptr().add(compressed_offset) }.cast(),
                nbytes: tile.compressed_size,
            };

            unsafe {
                sys::libdeflate_gdeflate_decompress(
                    self.0,
                    &mut compressed_page,
                    1,
                    bytes.as_mut_ptr().add(uncompressed_offset).cast(),
                    tile.uncompressed_size,
                    ptr::null_mut(),
                );
            }

            compressed_offset += tile.compressed_size;
            uncompressed_offset += tile.uncompressed_size;
        }

        Ok(bytes)
    }
}

impl Drop for Decompressor {
    fn drop(&mut self) {
        unsafe { sys::libdeflate_free_gdeflate_decompressor(self.0) }
    }
}
