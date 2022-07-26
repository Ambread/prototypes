use pretty_assertions::assert_eq;

use std::{collections::HashMap, vec};

use crate::{
    Frame,
    Instruction::{self, *},
    VM,
};

fn assert_vm_state(
    instructions: Vec<Instruction>,
    instruction_index: usize,
    stack: Vec<usize>,
    variables: Vec<(usize, usize)>,
) {
    // Too lazy right now to convert all the old tests to use VMState
    let variables: HashMap<usize, usize> = variables.into_iter().collect();

    VM {
        instructions,
        instruction_index,
        stack,
        frames: vec![Frame {
            variables,
            ..Default::default()
        }],
        ..Default::default()
    }
    .run_as_test();
}

impl VM {
    fn run_as_test(self) {
        let mut vm = VM::new(self.instructions.clone());
        vm.run();

        assert_eq!(self, vm);
    }
}

#[test]
fn empty_program() {
    VM {
        instructions: vec![Halt],
        instruction_index: 1,
        ..Default::default()
    }
    .run_as_test();
}

#[test]
fn push_halt() {
    let instructions = vec![Push(42), Push(68), Halt];
    let expected_instruction_index = 3;
    let expected_stack = vec![42, 68];
    let expected_variables = vec![];

    assert_vm_state(
        instructions,
        expected_instruction_index,
        expected_stack,
        expected_variables,
    );
}

#[test]
fn add() {
    let instructions = vec![Push(1), Push(2), Add, Halt];
    let expected_instruction_index = 4;
    let expected_stack = vec![3];
    let expected_variables = vec![];

    assert_vm_state(
        instructions,
        expected_instruction_index,
        expected_stack,
        expected_variables,
    );
}

#[test]
fn pop() {
    let instructions = vec![Push(42), Pop, Halt];
    let expected_instruction_index = 3;
    let expected_stack = vec![];
    let expected_variables = vec![];

    assert_vm_state(
        instructions,
        expected_instruction_index,
        expected_stack,
        expected_variables,
    );
}

#[test]
fn dupe() {
    let instructions = vec![Push(42), Dupe, Halt];
    let expected_instruction_index = 3;
    let expected_stack = vec![42, 42];
    let expected_variables = vec![];

    assert_vm_state(
        instructions,
        expected_instruction_index,
        expected_stack,
        expected_variables,
    );
}

#[test]
fn jump() {
    let instructions = vec![Jump(2), Halt, Jump(1)];
    let expected_instruction_index = 2;
    let expected_stack = vec![];
    let expected_variables = vec![];

    assert_vm_state(
        instructions,
        expected_instruction_index,
        expected_stack,
        expected_variables,
    );
}

#[test]
fn jump_if() {
    let instructions = vec![Push(1), JumpIf(3), Pop, Push(0), JumpIf(2), Halt];
    let expected_instruction_index = 6;
    let expected_stack = vec![];
    let expected_variables = vec![];

    assert_vm_state(
        instructions,
        expected_instruction_index,
        expected_stack,
        expected_variables,
    );
}

#[test]
fn load_uninitialized() {
    let instructions = vec![Load(0), Halt];
    let expected_instruction_index = 2;
    let expected_stack = vec![0];
    let expected_variables = vec![];

    assert_vm_state(
        instructions,
        expected_instruction_index,
        expected_stack,
        expected_variables,
    );
}

#[test]
fn store() {
    VM {
        instructions: vec![Push(42), Store(0), Halt],
        instruction_index: 3,
        frames: vec![Frame {
            return_index: 0,
            variables: HashMap::from([(0, 42)]),
        }],
        ..Default::default()
    }
    .run_as_test();
}

#[test]
fn load_store() {
    VM {
        instructions: vec![Push(42), Store(0), Load(0), Halt],
        instruction_index: 4,
        stack: vec![42],
        frames: vec![Frame {
            return_index: 0,
            variables: HashMap::from([(0, 42)]),
        }],
        ..Default::default()
    }
    .run_as_test();
}

#[test]
fn if_else() {
    VM {
        instructions: vec![
            // let a
            Push(6),
            Store(0),
            // let b
            Push(4),
            Store(1),
            // a > b
            Load(0),
            Load(1),
            Gt,
            JumpIf(11),
            // else
            Load(1),
            Store(2),
            Jump(13),
            // if
            Load(0),
            Store(2),
            // done
            Halt,
        ],

        instruction_index: 14,
        frames: vec![Frame {
            return_index: 0,
            variables: HashMap::from([(0, 6), (1, 4), (2, 6)]),
        }],
        ..Default::default()
    }
    .run_as_test();
}

#[test]
fn while_mul() {
    VM {
        instructions: vec![
            // let a
            Push(6),
            Store(0),
            // let b
            Push(4),
            Store(1),
            // let total
            Push(0),
            Store(2),
            // while cond
            Load(1),
            Push(1),
            Geq,
            Not,
            JumpIf(20),
            // while body
            // total += a
            Load(0),
            Load(2),
            Add,
            Store(2),
            // b -= 1
            Load(1),
            Push(1),
            Sub,
            Store(1),
            // continue
            Jump(6),
            // break
            Halt,
        ],

        instruction_index: 21,
        frames: vec![Frame {
            return_index: 0,
            variables: HashMap::from([(0, 6), (1, 0), (2, 24)]),
        }],
        ..Default::default()
    }
    .run_as_test()
}

#[test]
fn call_ret_empty() {
    VM {
        instructions: vec![Call(2), Halt, Return],
        instruction_index: 2,
        ..Default::default()
    }
    .run_as_test();
}

#[test]
fn call_ret_const() {
    VM {
        instructions: vec![Call(2), Halt, Push(7), Return],
        instruction_index: 2,
        stack: vec![7],
        ..Default::default()
    }
    .run_as_test();
}

#[test]
fn call_ret_double() {
    VM {
        instructions: vec![Push(3), Call(3), Halt, Push(2), Mul, Return],
        instruction_index: 3,
        stack: vec![6],
        ..Default::default()
    }
    .run_as_test();
}
