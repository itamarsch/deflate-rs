use std::cmp::min;

pub struct BitReader<'a> {
    data: &'a [u8],
    bit_position: usize,
}

impl<'b> BitReader<'b> {
    pub fn new<'a>(data: &'a [u8]) -> BitReader<'a> {
        BitReader {
            data,
            bit_position: 0,
        }
    }

    pub fn pos(&self) -> usize {
        self.bit_position
    }

    pub fn read_n_bits(&mut self, mut n: usize) -> usize {
        let mut result: usize = 0;
        let mut shift = 0;
        while n > 0 {
            let byte_index = self.bit_position / 8;
            let bit_index = self.bit_position % 8;
            let bits_in_this_byte = min(n, 8 - bit_index);

            let mask = (1 << bits_in_this_byte) - 1;
            let val = (self.data[byte_index] as usize >> bit_index) & mask;

            result |= val << shift;
            shift += bits_in_this_byte;

            self.bit_position += bits_in_this_byte;
            n -= bits_in_this_byte
        }

        result
    }

    pub fn read_bool(&mut self) -> bool {
        self.read_n_bits(1) == 1
    }

    pub fn read_until_byte_boundry(&mut self) {
        let bit_index = self.bit_position % 8;
        if bit_index != 0 {
            self.read_n_bits(8 - bit_index);
        }
    }

    pub fn read_bytes<'a>(&'a mut self, n: usize) -> &'b [u8] {
        assert!(self.bit_position % 8 == 0);
        let byte_index = self.bit_position / 8;
        let slice = &self.data[byte_index..byte_index + n];
        self.bit_position += n * 8;
        slice
    }

    pub fn read_u16(&mut self) -> usize {
        let byte1 = self.read_n_bits(8);
        let byte2 = self.read_n_bits(8);

        byte2 << 8 | byte1
    }
}
