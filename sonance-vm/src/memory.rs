use std::ops::{Index, IndexMut};

pub struct Memory([u8; u8::MAX as usize + 1]);

impl Memory {
    pub fn from_slice(slice: &[u8]) -> Self {
        let mut memory = Memory::default();
        memory.0[..slice.len()].copy_from_slice(slice);
        memory
    }
}

impl Default for Memory {
    fn default() -> Self {
        Self([0; u8::MAX as usize + 1])
    }
}

impl Index<u8> for Memory {
    type Output = u8;

    fn index(&self, index: u8) -> &Self::Output {
        &self.0[index as usize]
    }
}

impl IndexMut<u8> for Memory {
    fn index_mut(&mut self, index: u8) -> &mut Self::Output {
        &mut self.0[index as usize]
    }
}
