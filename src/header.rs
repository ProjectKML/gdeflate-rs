use std::{
    io,
    io::{Read, Write},
};

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

use crate::DEFAULT_TILE_SIZE;

const GDEFLATE_ID: u8 = 4;

pub struct Header {
    id: u8,
    magic: u8,

    num_tiles: u16,

    value: u32,
}

impl Header {
    pub fn new(uncompressed_size: usize) -> Self {
        let num_tiles = uncompressed_size / DEFAULT_TILE_SIZE;
        let last_tile_size = uncompressed_size - num_tiles * DEFAULT_TILE_SIZE;

        Self {
            id: GDEFLATE_ID,
            magic: GDEFLATE_ID ^ 0xff,

            num_tiles: num_tiles as _,

            value: 1 << 30 | (last_tile_size as u32 & 0x3ffff) << 12,
        }
    }

    pub fn from_reader(reader: &mut impl Read) -> io::Result<Self> {
        Ok(Self {
            id: reader.read_u8()?,
            magic: reader.read_u8()?,

            num_tiles: reader.read_u16::<LittleEndian>()?,

            value: reader.read_u32::<LittleEndian>()?,
        })
    }

    pub fn write(&self, writer: &mut impl Write) -> io::Result<()> {
        writer.write_u8(self.id)?;
        writer.write_u8(self.magic)?;

        writer.write_u16::<LittleEndian>(self.num_tiles)?;

        writer.write_u32::<LittleEndian>(self.value)
    }

    #[inline]
    pub fn valid(&self) -> bool {
        self.id == self.magic ^ 0xff && self.id == GDEFLATE_ID
    }

    #[inline]
    pub fn last_tile_size(&self) -> usize {
        ((self.value >> 12) & 0x3ffff) as _
    }

    #[inline]
    pub fn num_tiles(&self) -> usize {
        self.num_tiles as _
    }

    pub fn uncompressed_size(&self) -> usize {
        let last_tile_size = self.last_tile_size();

        (self.num_tiles as usize) * DEFAULT_TILE_SIZE
            - if last_tile_size == 0 {
                0
            } else {
                DEFAULT_TILE_SIZE - last_tile_size
            }
    }
}
