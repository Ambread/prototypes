use std::collections::HashMap;

use crate::{
    Instruction::{self, *},
    VM,
};

fn assert_vm_state(
    instructions: Vec<Instruction>,
    expected_instruction_index: usize,
    expected_stack: Vec<usize>,
    expected_variables: Vec<(usize, usize)>,
) {
    let mut vm = VM::new(instructions);
    vm.run();

    assert!(vm.is_halted);
    assert_eq!(expected_instruction_index, vm.instruction_index);
    assert_eq!(expected_stack, vm.stack);
    let expected_variables: HashMap<usize, usize> = expected_variables.into_iter().collect();
    assert_eq!(expected_variables, vm.variables);
}

#[derive(Debug, Clone)]
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
    let instructions = vec![Halt];
    let expected_instruction_index = 1;
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
