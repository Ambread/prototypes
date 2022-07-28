use pretty_assertions::assert_eq;

use std::{collections::HashMap, vec};

use crate::{
    builder::InstructionBuilder,
    vm::{
        Frame, Frames,
        Instruction::{self, *},
        VM,
    },
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

fn builder() -> InstructionBuilder {
    InstructionBuilder::new()
}

fn parse(src: &str) -> Vec<Instruction> {
    src.parse::<InstructionBuilder>().unwrap().build()
}

#[test]
fn empty_program() {
    VM {
        instructions: builder().just(Halt).build(),
        instruction_index: 0,
        ..Default::default()
    }
    .run_and_asset();
}

#[test]
fn push_halt() {
    VM {
        instructions: builder().push(42).push(68).just(Halt).build(),
        instruction_index: 2,
        stack: vec![42, 68],
        ..Default::default()
    }
    .run_and_asset();
}

#[test]
fn add() {
    VM {
        instructions: builder().push(1).push(2).just(Add).just(Halt).build(),
        instruction_index: 3,
        stack: vec![3],
        ..Default::default()
    }
    .run_and_asset();
}

#[test]
fn pop() {
    VM {
        instructions: builder().then(Pop, 42).just(Halt).build(),
        instruction_index: 2,
        ..Default::default()
    }
    .run_and_asset();
}

#[test]
fn dupe() {
    VM {
        instructions: builder().then(Dupe, 42).just(Halt).build(),
        instruction_index: 2,
        stack: vec![42, 42],
        ..Default::default()
    }
    .run_and_asset();
}

#[test]
fn jump() {
    VM {
        instructions: builder()
            .then(Jump, "end")
            .label("middle")
            .just(Halt)
            .label("end")
            .then(Jump, "middle")
            .build(),
        instruction_index: 2,
        ..Default::default()
    }
    .run_and_asset();
}

#[test]
fn jump_if() {
    VM {
        instructions: builder()
            .push(0)
            .then(JumpIf, "foo")
            .label("bar")
            .just(Pop)
            .label("foo")
            .push(1)
            .then(JumpIf, "bar")
            .just(Halt)
            .build(),
        instruction_index: 7,
        ..Default::default()
    }
    .run_and_asset();
}

#[test]
fn load_uninitialized() {
    VM {
        instructions: builder().then(Load, 0).just(Halt).build(),
        instruction_index: 2,
        stack: vec![0],
        ..Default::default()
    }
    .run_and_asset();
}

#[test]
fn store() {
    VM {
        instructions: builder().push(42).then(Store, 0).just(Halt).build(),
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
        instructions: builder()
            .push(42)
            .then(Store, 0)
            .then(Load, 0)
            .just(Halt)
            .build(),
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
fn if_else_bad() {
    VM {
        instructions: parse(
            "
            push 6
            store 0

            push 4
            store 1

            load 0
            load 1
            gt
            jump_if else

            load 0
            store 2
            jump done

            else:
            load 1
            store 2

            done:
            halt
        ",
        ),

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
        instructions: builder()
            .then(Call, "func")
            .just(Halt)
            .label("func")
            .just(Return)
            .build(),
        instruction_index: 2,
        ..Default::default()
    }
    .run_and_asset();
}

#[test]
fn call_ret_const() {
    VM {
        instructions: builder()
            .then(Call, "func")
            .just(Halt)
            .label("func")
            .then(Return, 7)
            .build(),
        instruction_index: 2,
        stack: vec![7],
        ..Default::default()
    }
    .run_and_asset();
}

#[test]
fn call_ret_double() {
    VM {
        instructions: builder()
            .push(3)
            .then(Call, "func")
            .just(Halt)
            .label("func")
            .then(Mul, 2)
            .just(Return)
            .build(),
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
