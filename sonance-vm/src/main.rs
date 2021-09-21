mod memory;

use memory::Memory;

fn main() {
    let input = std::env::current_dir().unwrap().join("../dev/input.hex");
    let input = std::fs::read(input).unwrap();

    let mut vm = VM::new(Memory::from_slice(&input));

    while vm.step() {}
}

#[derive(Debug, Eq, PartialEq, num_enum::FromPrimitive)]
#[repr(u8)]
enum Opcode {
    #[default]
    Null = 0x00,
    Exit = 0x01,  //
    Print = 0x02, // a
    Move = 0x03,  // src, dst
    Const = 0x04, // data, dst
    Add = 0x10,   // a, b, dst
    Sub = 0x11,   // a, b, dst
    Mul = 0x12,   // a, b, dst
    Div = 0x13,   // a, b, dst
    And = 0x14,   // a, b, dst
    Or = 0x15,    // a, b, dst
    Not = 0x16,   // a, dst
}

pub const PROGRAM_COUNTER: u8 = 0x01;

struct VM {
    memory: Memory,
}

impl VM {
    fn new(memory: Memory) -> Self {
        Self { memory }
    }

    fn next(&mut self) -> u8 {
        let value = self.memory[self.memory[PROGRAM_COUNTER]];
        self.memory[PROGRAM_COUNTER] += 1;
        value
    }

    fn step(&mut self) -> bool {
        let opcode = self.next();

        match opcode.into() {
            Opcode::Null => panic!("Unknown opcode"),
            Opcode::Exit => return false,
            Opcode::Print => {
                let a = self.next();
                println!("{}", self.memory[a]);
            }
            Opcode::Move => self.unary_op(|a| a),
            Opcode::Const => {
                let data = self.next();
                let dst = self.next();

                self.memory[dst] = data;
            }
            Opcode::Add => self.binary_op(|a, b| a + b),
            Opcode::Sub => self.binary_op(|a, b| a - b),
            Opcode::Mul => self.binary_op(|a, b| a * b),
            Opcode::Div => self.binary_op(|a, b| a / b),
            Opcode::And => self.binary_op(|a, b| a & b),
            Opcode::Or => self.binary_op(|a, b| a | b),
            Opcode::Not => self.unary_op(|a| !a),
        }

        true
    }

    fn unary_op(&mut self, op: impl Fn(u8) -> u8) {
        let a = self.next();
        let dst = self.next();

        self.memory[dst] = op(self.memory[a]);
    }

    fn binary_op(&mut self, op: impl Fn(u8, u8) -> u8) {
        let a = self.next();
        let b = self.next();
        let dst = self.next();

        self.memory[dst] = op(self.memory[a], self.memory[b]);
    }
}
