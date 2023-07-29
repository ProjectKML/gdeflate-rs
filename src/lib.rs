mod header;
use std::{
    io,
    io::{Cursor, Write},
    ptr,
};

use gdeflate_sys::*;
use thiserror::Error;
pub use header::*;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Failed to create compressor")]
    CompressorCreationFailed,
    #[error("Failed to compress data")]
    CompressionFailed,
    #[error("Failed to create decompressor")]
    DecompressorCreationFailed,
    #[error("The header is invalid")]
    InvalidHeader,
    #[error("Io error: {0}")]
    IoError(#[from] io::Error)
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

const DEFAULT_TILE_SIZE: usize = 64 * 1024;

pub struct Compressor(*mut libdeflate_gdeflate_compressor);

#[derive(Default)]
struct Tile {
    uncompressed_size: usize,
    bytes: Vec<u8>,
}

impl Compressor {
    #[inline]
    pub fn new(compression_level: CompressionLevel) -> Result<Self, Error> {
        let compressor = unsafe { libdeflate_alloc_gdeflate_compressor(compression_level as _) };
        if compressor.is_null() {
            Err(Error::CompressorCreationFailed)
        } else {
            Ok(Self(compressor))
        }
    }

    pub fn compress(&mut self, bytes: &[u8]) -> Result<Vec<u8>, Error> {
        let num_tiles = (bytes.len() + DEFAULT_TILE_SIZE - 1) / DEFAULT_TILE_SIZE;

        let mut page_count = 0;
        let scratch_size = unsafe {
            libdeflate_gdeflate_compress_bound(ptr::null_mut(), DEFAULT_TILE_SIZE, &mut page_count)
        };
        assert_eq!(page_count, 1);

        let mut scratch_buffer = vec![0u8; scratch_size];

        let mut tiles = Vec::with_capacity(scratch_size);

        // Compress all tiles
        for i in 0..num_tiles {
            let tile_offset = i * DEFAULT_TILE_SIZE;

            let mut compressed_page = libdeflate_gdeflate_out_page {
                data: scratch_buffer.as_mut_ptr().cast(),
                nbytes: scratch_size,
            };

            let remaining = bytes.len() - tile_offset;
            let uncompressed_size = remaining.min(DEFAULT_TILE_SIZE);

            unsafe {
                libdeflate_gdeflate_compress(
                    self.0,
                    bytes.as_ptr().add(tile_offset).cast(),
                    uncompressed_size,
                    &mut compressed_page,
                    1,
                );
            }

            tiles.push(Tile {
                uncompressed_size,
                bytes: vec![0; compressed_page.nbytes],
            });

            unsafe {
                ptr::copy_nonoverlapping(
                    scratch_buffer.as_ptr(),
                    tiles[i].bytes.as_mut_ptr(),
                    compressed_page.nbytes,
                );
            }
        }

        // Collect header
        let mut data_pos = 0;
        let mut tile_ptrs = tiles
            .iter()
            .map(|tile| {
                data_pos += tile.bytes.len() as u32;
                data_pos
            })
            .collect::<Vec<_>>();

        tile_ptrs[0] = tiles.last().unwrap().bytes.len() as _;

        // Write output stream
        let mut uncompressed_size = tile_ptrs.len() * DEFAULT_TILE_SIZE;
        let tail_size = bytes.len() - (tile_ptrs.len() - 1) * DEFAULT_TILE_SIZE;
        if tail_size < DEFAULT_TILE_SIZE {
            uncompressed_size -= DEFAULT_TILE_SIZE - tail_size;
        }

        let header = Header::new(uncompressed_size);

        let mut writer = Cursor::new(Vec::new());
        header.write(&mut writer)?;
        writer.write_all(bytemuck::cast_slice(&tile_ptrs))?;

        for tile in &tiles {
            writer.write_all(&tile.bytes)?;
        }

        Ok(writer.into_inner())
    }
}

impl Drop for Compressor {
    #[inline]
    fn drop(&mut self) {
        unsafe { libdeflate_free_gdeflate_compressor(self.0) }
    }
}

pub struct Decompressor(*mut libdeflate_gdeflate_decompressor);

impl Decompressor {
    #[inline]
    pub fn new() -> Result<Self, Error> {
        let decompressor = unsafe { libdeflate_alloc_gdeflate_decompressor() };
        if decompressor.is_null() {
            Err(Error::DecompressorCreationFailed)
        } else {
            Ok(Self(decompressor))
        }
    }

    pub fn decompress(&mut self, bytes: &[u8], decompressed_size: usize) -> Result<Vec<u8>, Error> {
        let buffer = vec![0; decompressed_size];

        let mut reader = Cursor::new(bytes);

        let header = Header::from_reader(&mut reader)?;
        if !header.valid() {
            return Err(Error::InvalidHeader)
        }

        let num_tiles = header.num_tiles();
        
        for i in 0..num_tiles {
            let _tile_offset = i * DEFAULT_TILE_SIZE;
            
            let _page = libdeflate_gdeflate_in_page {
                data: ptr::null_mut(),
                nbytes: 0,
            };
        }

        Ok(buffer)
    }
}

impl Drop for Decompressor {
    #[inline]
    fn drop(&mut self) {
        unsafe { libdeflate_free_gdeflate_decompressor(self.0) }
    }
}

#[cfg(test)]
mod tests {
    use crate::{CompressionLevel, Compressor, Decompressor};

    #[test]
    fn test() {
        let data = (0..100000).map(|e| (e % 255) as u8).collect::<Vec<_>>();

        let mut compressor = Compressor::new(CompressionLevel::Level12).unwrap();

        let compressed_data = compressor.compress(&data).unwrap();

        let mut decompressor = Decompressor::new().unwrap();
        let decompressed_data = decompressor
            .decompress(&compressed_data, data.len())
            .unwrap();

        assert_eq!(data, decompressed_data);
    }
}
