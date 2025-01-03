pub struct HuffmanDict(Vec<[Option<u16>; 16]>);

impl HuffmanDict {
    pub fn new() -> Self {
        Self(vec![[None; 16]; 32768])
    }
    pub fn insert(&mut self, code: u16, length: u16, value: u16) {
        self.0[code as usize][length as usize] = Some(value);
    }

    pub fn get(&self, code: u16, length: u16) -> Option<u16> {
        self.0[code as usize][length as usize]
    }
}
