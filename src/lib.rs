use gdeflate_sys::*;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Failed to create compressor")]
    CompressorCreationFailed,
    #[error("Failed to compress data")]
    CompressionFailed,
    #[error("Failed to create decompressor")]
    DecompressorCreationFailed,
    #[error("Failed to decompress data")]
    DecompressionFailed,
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

pub struct Compressor {
    compressor: *mut libdeflate_gdeflate_compressor,
}

impl Compressor {
    pub fn new(compression_level: CompressionLevel) -> Result<Self, Error> {
        let compressor = unsafe { libdeflate_alloc_gdeflate_compressor(compression_level as _) };
        if compressor == std::ptr::null_mut() {
            Err(Error::CompressorCreationFailed)
        } else {
            Ok(Self { compressor })
        }
    }

    pub fn compress(&mut self, bytes: &[u8]) -> Result<Vec<u8>, Error> {
        let mut pages_needed = 0;
        let upper_bound = unsafe {
            libdeflate_gdeflate_compress_bound(
                self.compressor,
                bytes.len(),
                &mut pages_needed as *mut _,
            )
        };
        let upper_bound_per_page = upper_bound / pages_needed;

        let mut pages = vec![libdeflate_gdeflate_out_page {
            data: std::ptr::null_mut(),
            nbytes: 0,
        }];
        for page in &mut pages {
            page.data = unsafe { libc::malloc(upper_bound_per_page) };
        }

        let compressed_size = unsafe {
            libdeflate_gdeflate_compress(
                self.compressor,
                bytes.as_ptr().cast(),
                bytes.len(),
                pages.as_mut_ptr(),
                pages.len(),
            )
        };

        if compressed_size == 0 {
            Err(Error::CompressionFailed)
        } else {
            let mut compressed_data = vec![0u8; compressed_size];
            let mut pos = 0usize;

            for page in &pages {
                unsafe {
                    libc::memcpy(
                        compressed_data.as_mut_ptr().add(pos).cast(),
                        page.data,
                        page.nbytes,
                    );
                    pos += page.nbytes;

                    libc::free(page.data)
                };
            }

            Ok(compressed_data)
        }
    }
}

impl Drop for Compressor {
    #[inline]
    fn drop(&mut self) {
        unsafe { libdeflate_free_gdeflate_compressor(self.compressor) }
    }
}

pub struct Decompressor {
    decompressor: *mut libdeflate_gdeflate_decompressor,
}

impl Decompressor {
    pub fn new() -> Result<Self, Error> {
        let decompressor = unsafe { libdeflate_alloc_gdeflate_decompressor() };
        if decompressor == std::ptr::null_mut() {
            Err(Error::DecompressorCreationFailed)
        } else {
            Ok(Self { decompressor })
        }
    }

    pub fn decompress(&mut self, bytes: &[u8], decompressed_size: usize) -> Result<Vec<u8>, Error> {
        let mut page = libdeflate_gdeflate_in_page {
            data: bytes.as_ptr().cast(),
            nbytes: bytes.len(),
        };

        let mut decompressed_data = vec![0u8; decompressed_size];

        let mut actual_decompressed = 0;

        let result = unsafe {
            libdeflate_gdeflate_decompress(
                self.decompressor,
                &mut page as *mut _,
                1,
                decompressed_data.as_mut_ptr().cast(),
                decompressed_size,
                &mut actual_decompressed as *mut _,
            )
        };

        if actual_decompressed != decompressed_size {
            return Err(Error::DecompressionFailed)
        }

        match result {
            libdeflate_result_LIBDEFLATE_SUCCESS => Ok(decompressed_data),
            _ => Err(Error::DecompressionFailed),
        }
    }
}

impl Drop for Decompressor {
    #[inline]
    fn drop(&mut self) {
        unsafe { libdeflate_free_gdeflate_decompressor(self.decompressor) }
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
