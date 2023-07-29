use crate::DEFAULT_TILE_SIZE;

const GDEFLATE_ID: u8 = 4;

#[repr(C)]
struct TileStream {
    id: u8,
    magic: u8,

    num_tiles: u16,

    value: u32,
}

impl TileStream {
    fn new(uncompressed_size: usize) -> Self {
        let num_tiles = (uncompressed_size / DEFAULT_TILE_SIZE);
        let last_tile_size = uncompressed_size - num_tiles * DEFAULT_TILE_SIZE;

        Self {
            id: GDEFLATE_ID,
            magic: GDEFLATE_ID ^ 0xff,

            num_tiles: num_tiles as _,

            value: 1 << 30 | (last_tile_size as u32) << 12, //TODO: endianness?
        }
    }

    fn valid(&self) -> bool {
        self.id == (self.magic ^ 0xff)
    }

    fn last_tile_size(&self) -> usize {
        ((self.value >> 12) & 0x3ffff) as _
    }

    fn uncompressed_size(&self) -> usize {
        let last_tile_size = self.last_tile_size();

        (self.num_tiles as usize) * DEFAULT_TILE_SIZE
            - if last_tile_size == 0 {
                0
            } else {
                DEFAULT_TILE_SIZE - last_tile_size
            }
    }
}
