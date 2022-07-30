pub mod memory;

use std::fmt::Debug;

pub trait Device: Debug + Clone + PartialEq {
    fn read(&mut self, index: u32) -> u8;
    fn write(&mut self, index: u32, value: u8);
}
