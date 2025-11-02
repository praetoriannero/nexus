// TODO: https://stackoverflow.com/questions/57203009/implementing-slice-for-custom-type
// This is so we can index into the arena when parsing types into arenas

struct AllocError {}

pub struct Arena {
    buffer: Vec<u8>,
    offset: usize,
}

impl Arena {
    pub fn with_capacity(cap: usize) -> Self {
        Self {
            buffer: vec![0; cap],
            offset: 0,
        }
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
