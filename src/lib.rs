use thiserror::Error;

#[cxx::bridge(namespace = "GDeflate")]
mod ffi {
    unsafe extern "C++" {
        include!("GDeflate.h");

        fn CompressBound(size: usize) -> usize;
        unsafe fn Compress(
            output: *mut u8,
            output_size: *mut usize,
            in_: *const u8,
            in_size: usize,
            level: u32,
            flags: u32,
        ) -> bool;
        unsafe fn Decompress(
            output: *mut u8,
            output_size: usize,
            in_: *const u8,
            in_size: usize,
            num_workers: u32,
        ) -> bool;
    }
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

#[derive(Error, Debug)]
pub enum Error {
    #[error("Compression failed")]
    CompressionFailed,
    #[error("Decompression failed")]
    DecompressionFailed,
}

const COMPRESS_SINGLE_THREAD: u32 = 0x200;

pub fn compress(bytes: &[u8], level: CompressionLevel) -> Result<Vec<u8>, Error> {
    let mut size = ffi::CompressBound(bytes.len());

    let mut buffer = vec![0; size];

    let result = unsafe {
        ffi::Compress(
            buffer.as_mut_ptr(),
            &mut size,
            bytes.as_ptr(),
            bytes.len(),
            level as _,
            COMPRESS_SINGLE_THREAD,
        )
    };
    buffer.resize(size, 0);

    if !result {
        return Err(Error::CompressionFailed)
    }

    Ok(buffer)
}

pub fn decompress(bytes: &[u8], decompressed_size: usize) -> Result<Vec<u8>, Error> {
    let mut buffer = vec![0; decompressed_size];

    let result = unsafe {
        ffi::Decompress(
            buffer.as_mut_ptr(),
            decompressed_size,
            bytes.as_ptr(),
            bytes.len(),
            1,
        )
    };

    if !result {
        return Err(Error::DecompressionFailed)
    }

    Ok(buffer)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compress() {
        let bytes = b"Hello, world!";
        let compressed = compress(bytes, CompressionLevel::Level1).unwrap();
        let decompressed = decompress(&compressed, bytes.len()).unwrap();
        assert_eq!(bytes, &decompressed);
    }
}
