use crate::parse::Expr;
use std::io::{self, Read};

#[derive(Debug, Clone)]
pub struct Memory {
    cells: Vec<u8>,
    index: usize,
}

impl Memory {
    pub fn new(start_size: usize) -> Self {
        Self {
            cells: Vec::with_capacity(start_size),
            index: 0,
        }
    }

    pub fn index(&self) -> &usize {
        &self.index
    }

    pub fn index_mut(&mut self) -> &mut usize {
        &mut self.index
    }

    pub fn get(&mut self, index: usize) -> &mut u8 {
        if index >= self.cells.len() {
            self.cells.resize(index + 1, 0);
        }

        self.cells.get_mut(index).expect("cell to exist")
    }

    pub fn run(&mut self, input: &[Expr]) {
        for expr in input {
            match expr {
                Expr::Right => {
                    self.index = self.index.checked_add(1).unwrap();
                }

                Expr::Left => {
                    self.index = self.index.checked_sub(1).unwrap();
                }

                Expr::Inc => {
                    let cur = self.get(self.index);
                    *cur = cur.wrapping_add(1);
                }

                Expr::Dec => {
                    let cur = self.get(self.index);
                    *cur = cur.wrapping_sub(1);
                }

                Expr::Output => {
                    print!("{}", *self.get(self.index) as char);
                }

                Expr::Input => {
                    let mut buffer = [0];
                    io::stdin().read_exact(&mut buffer).unwrap();
                    *self.get(self.index) = buffer[0];
                }

                Expr::Loop(body) => {
                    while *self.get(self.index) != 0 {
                        self.run(body)
                    }
                }
            }
        }
    }

    pub fn clear(&mut self) {
        self.cells.clear()
    }
}
