use std::cmp::min;

pub struct BitReader<'a> {
    data: &'a [u8],
    bit_position: usize,
}

impl BitReader<'_> {
    pub fn new<'a>(data: &'a [u8]) -> BitReader<'a> {
        BitReader {
            data,
            bit_position: 0,
        }
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
}
