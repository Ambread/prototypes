use pretty_assertions::assert_eq;

use std::{collections::HashMap, vec};

use crate::{
    builder::InstructionBuilder,
    vm::{Frame, Frames, Instruction::*, VM},
};

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
            Push(0),
            Push(4),
            JumpIf,
            Pop,
            Push(1),
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
        instructions: InstructionBuilder::new()
            .push(6)
            .then(Store, 0)
            .push(4)
            .then(Store, 1)
            .then(Load, 0)
            .then(Load, 1)
            .just(Gt)
            .then(JumpIf, "else")
            .then(Load, 0)
            .then(Store, 2)
            .then(Jump, "done")
            .label("else")
            .then(Load, 1)
            .then(Store, 2)
            .label("done")
            .just(Halt)
            .build(),

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
        instructions: InstructionBuilder::new()
            // let a
            .push(6)
            .then(Store, 0)
            // let b
            .push(4)
            .then(Store, 1)
            // let c
            .push(0)
            .then(Store, 2)
            // while
            .label("while")
            .then(Load, 1)
            .then(Geq, 1)
            .then(JumpIf, "break")
            // total += a
            .then(Load, 0)
            .then(Load, 2)
            .just(Add)
            .then(Store, 2)
            // b -= 1
            .then(Load, 1)
            .then(Sub, 1)
            .then(Store, 1)
            .then(Jump, "while")
            // break
            .label("break")
            .just(Halt)
            .build(),

        instruction_index: 30,
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
        instructions: vec![Push(3), Call, Halt, Return],
        instruction_index: 2,
        ..Default::default()
    }
    .run_and_asset();
}

#[test]
fn call_ret_const() {
    VM {
        instructions: vec![Push(3), Call, Halt, Push(7), Return],
        instruction_index: 2,
        stack: vec![7],
        ..Default::default()
    }
    .run_and_asset();
}

#[test]
fn call_ret_double() {
    VM {
        instructions: vec![Push(3), Push(4), Call, Halt, Push(2), Mul, Return],
        instruction_index: 3,
        stack: vec![6],
        ..Default::default()
    }
    .run_and_asset();
}

#[test]
fn max() {
    VM {
        instructions: InstructionBuilder::new()
            .push(6)
            .push(4)
            .then(Call, "max")
            .just(Halt)
            // fn max
            .label("max")
            .then(Store, 1)
            .then(Store, 0)
            // if
            .then(Load, 0)
            .then(Load, 1)
            .just(Gt)
            .then(JumpIf, "else")
            // then
            .then(Load, 0)
            .just(Return)
            // else
            .label("else")
            .then(Load, 1)
            .just(Return)
            .build(),

        instruction_index: 4,
        stack: vec![6],
        ..Default::default()
    }
    .run_and_asset()
}
