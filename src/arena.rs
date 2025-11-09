// TODO: https://stackoverflow.com/questions/57203009/implementing-slice-for-custom-type
// This is so we can index into the arena when parsing types into arenas

// struct AllocError {}

pub struct Arena {
    pub buffer: Vec<u8>,
    offset: usize,
    pages: Vec<Vec<u8>>,
}

impl<T> std::ops::Index<T> for Arena
where
    T: std::slice::SliceIndex<[u8]>,
{
    type Output = T::Output;

    fn index(&self, index: T) -> &Self::Output {
        &self.buffer[index]
    }
}

impl Arena {
    pub fn new(cap: usize) -> Self {
        Self {
            buffer: vec![0; cap],
            offset: 0,
            pages: Vec::new(),
        }
    }

    pub fn alloc_page(&mut self) -> &Vec<u8> {
        self.pages.push(Vec::new());
        &self.pages.last().unwrap()
    }

    pub fn alloc(&mut self, size: usize) -> Option<&mut [u8]> {
        if self.offset + size > self.buffer.len() {
            return None;
        }

        let start = self.offset;
        let end = start + size;
        self.offset = end;
        Some(&mut self.buffer[start..end])
    }

    // pub fn alloc_array(&mut self, size: usize)

    // alloc_array
    // alloc_array_init
    // alloc
    // alloc_init

    pub fn reset(&mut self) {
        self.offset = 0;
    }

    pub fn remaining(&self) -> usize {
        self.buffer.len() - self.offset
    }

    pub fn clear(&mut self) {
        self.buffer.fill(0);
        self.reset();
    }
}
