use pretty_assertions::assert_eq;

use std::{collections::HashMap, vec};

use crate::vm::{Frame, Frames, Instruction::*, VM};

impl VM {
    /// Create a fresh VM with this VM's instructions, run it to completion, and assert that it reaches the same state as this VM
    fn run_and_asset(self) {
        let mut vm = VM::new(self.instructions.clone());
        match vm.run() {
            Ok(()) => assert_eq!(vm, self),
            Err(error) => panic!("{error}"),
        }
    }
}

#[test]
fn empty_program() {
    VM {
        instructions: vec![Halt],
        instruction_index: 0,
        ..Default::default()
    }
    .run_and_asset();
}

#[test]
fn push_halt() {
    VM {
        instructions: vec![Push(42), Push(68), Halt],
        instruction_index: 2,
        stack: vec![42, 68],
        ..Default::default()
    }
    .run_and_asset();
}

#[test]
fn add() {
    VM {
        instructions: vec![Push(1), Push(2), Add, Halt],
        instruction_index: 3,
        stack: vec![3],
        ..Default::default()
    }
    .run_and_asset();
}

#[test]
fn pop() {
    VM {
        instructions: vec![Push(42), Pop, Halt],
        instruction_index: 2,
        ..Default::default()
    }
    .run_and_asset();
}

#[test]
fn dupe() {
    VM {
        instructions: vec![Push(42), Dupe, Halt],
        instruction_index: 2,
        stack: vec![42, 42],
        ..Default::default()
    }
    .run_and_asset();
}

#[test]
fn jump() {
    VM {
        instructions: vec![Push(3), Jump, Halt, Push(2), Jump],
        instruction_index: 2,
        ..Default::default()
    }
    .run_and_asset();
}

#[test]
fn jump_if() {
    VM {
        instructions: vec![
            Push(1),
            Push(4),
            JumpIf,
            Pop,
            Push(0),
            Push(3),
            JumpIf,
            Halt,
        ],
        instruction_index: 7,
        ..Default::default()
    }
    .run_and_asset();
}

#[test]
fn load_uninitialized() {
    VM {
        instructions: vec![Push(0), Load, Halt],
        instruction_index: 2,
        stack: vec![0],
        ..Default::default()
    }
    .run_and_asset();
}

#[test]
fn store() {
    VM {
        instructions: vec![Push(42), Push(0), Store, Halt],
        instruction_index: 3,
        frames: Frames::new(vec![Frame {
            return_index: 0,
            variables: HashMap::from([(0, 42)]),
        }]),
        ..Default::default()
    }
    .run_and_asset();
}

#[test]
fn load_store() {
    VM {
        instructions: vec![Push(42), Push(0), Store, Push(0), Load, Halt],
        instruction_index: 5,
        stack: vec![42],
        frames: Frames::new(vec![Frame {
            return_index: 0,
            variables: HashMap::from([(0, 42)]),
        }]),
        ..Default::default()
    }
    .run_and_asset();
}

#[test]
fn if_else() {
    VM {
        instructions: vec![
            // let a
            Push(6), // 0
            Push(0), // 1
            Store,   // 2
            // let b
            Push(4), // 3
            Push(1), // 4
            Store,   // 5
            // a > b
            Push(0),  // 6
            Load,     // 7
            Push(1),  // 8
            Load,     // 9
            Gt,       // 10
            Push(19), // 11
            JumpIf,   // 12
            // else
            Push(1),  // 13
            Load,     // 14
            Push(2),  // 15
            Store,    // 16
            Push(24), // 17
            Jump,     // 18
            // if
            Push(0), // 19
            Load,    // 20
            Push(2), // 21
            Store,   // 22
            // done
            Halt, // 23
        ],

        instruction_index: 23,
        frames: Frames::new(vec![Frame {
            return_index: 0,
            variables: HashMap::from([(0, 6), (1, 4), (2, 6)]),
        }]),
        ..Default::default()
    }
    .run_and_asset();
}

#[test]
fn while_mul() {
    VM {
        instructions: vec![
            // let a
            Push(6), // 0
            Push(0), // 1
            Store,   // 2
            // let b
            Push(4), // 3
            Push(1), // 4
            Store,   // 5
            // let total
            Push(0), // 6
            Push(2), // 7
            Store,   // 8
            // while
            Push(1),  // 9
            Load,     // 10
            Push(1),  // 11
            Geq,      // 12
            BoolNot,  // 13
            Push(31), // 14
            JumpIf,   // 15
            // do
            // total += a
            Push(0), // 16
            Load,    // 17
            Push(2), // 18
            Load,    // 19
            Add,     // 20
            Push(2), // 21
            Store,   // 22
            // b -= 1
            Push(1), // 23
            Load,    // 24
            Push(1), // 25
            Sub,     // 26
            Push(1), // 27
            Store,   // 28
            // continue
            Push(9), // 29
            Jump,    // 30
            // break
            Halt, // 31
        ],

        instruction_index: 31,
        frames: Frames::new(vec![Frame {
            return_index: 0,
            variables: HashMap::from([(0, 6), (1, 0), (2, 24)]),
        }]),
        ..Default::default()
    }
    .run_and_asset()
}

#[test]
fn call_ret_empty() {
    VM {
        instructions: vec![Call(2), Halt, Return],
        instruction_index: 1,
        ..Default::default()
    }
    .run_and_asset();
}

#[test]
fn call_ret_const() {
    VM {
        instructions: vec![Call(2), Halt, Push(7), Return],
        instruction_index: 1,
        stack: vec![7],
        ..Default::default()
    }
    .run_and_asset();
}

#[test]
fn call_ret_double() {
    VM {
        instructions: vec![Push(3), Call(3), Halt, Push(2), Mul, Return],
        instruction_index: 2,
        stack: vec![6],
        ..Default::default()
    }
    .run_and_asset();
}

#[test]
fn max() {
    VM {
        instructions: vec![
            Push(6), // 0
            Push(4), // 1
            Call(4), // 2
            Halt,    // 3
            // fn max
            Push(1), // 4
            Store,   // 5
            Push(0), // 6
            Store,   // 7
            // if
            Push(0),  // 8
            Load,     // 9
            Push(1),  // 10
            Load,     // 11
            Gt,       // 12
            Push(18), // 13
            JumpIf,   // 14
            // then
            Push(1), // 15
            Load,    // 16
            Return,  // 17
            // else
            Push(0), // 18
            Load,    // 19
            Return,  // 20
        ],
        instruction_index: 3,
        stack: vec![6],
        ..Default::default()
    }
    .run_and_asset()
}
