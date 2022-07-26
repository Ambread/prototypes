use std::{collections::HashMap, vec};

use crate::{
    Instruction::{self, *},
    VM,
};

fn default<T>() -> T
where
    T: Default,
{
    T::default()
}

fn assert_vm_state(
    instructions: Vec<Instruction>,
    expected_instruction_index: usize,
    expected_stack: Vec<usize>,
    expected_variables: Vec<(usize, usize)>,
) {
    // Too lazy right now to convert all the old tests to use VMState
    VMState {
        instructions,
        expected_instruction_index,
        expected_stack,
        expected_variables,
    }
    .assert();
}

#[derive(Debug, Clone, Default)]
struct VMState {
    instructions: Vec<Instruction>,
    expected_instruction_index: usize,
    expected_stack: Vec<usize>,
    expected_variables: Vec<(usize, usize)>,
}

impl VMState {
    fn assert(self) {
        let mut vm = VM::new(self.instructions);
        vm.run();

        assert!(vm.is_halted);
        assert_eq!(self.expected_instruction_index, vm.instruction_index);
        assert_eq!(self.expected_stack, vm.stack);

        let expected_variables: HashMap<usize, usize> =
            self.expected_variables.into_iter().collect();
        assert_eq!(expected_variables, vm.variables);
    }
}

#[test]
fn empty_program() {
    VMState {
        instructions: vec![Halt],
        expected_instruction_index: 1,
        ..default()
    }
    .assert();
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
    VMState {
        instructions: vec![Push(42), Store(0), Halt],
        expected_instruction_index: 3,
        expected_stack: vec![],
        expected_variables: vec![(0, 42)],
    }
    .assert();
}

#[test]
fn load_store() {
    VMState {
        instructions: vec![Push(42), Store(0), Load(0), Halt],
        expected_instruction_index: 4,
        expected_stack: vec![42],
        expected_variables: vec![(0, 42)],
    }
    .assert();
}
