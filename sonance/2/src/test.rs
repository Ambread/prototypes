use pretty_assertions::assert_eq;

use std::collections::HashMap;

use crate::{
    parser,
    vm::{Frame, Frames, VM},
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

fn parse(src: &str) -> Vec<u8> {
    parser::parse(src).unwrap()
}

#[test]
fn empty_program() {
    VM {
        instructions: parse("halt"),
        instruction_index: 0,
        ..Default::default()
    }
    .run_and_asset();
}

#[test]
fn push() {
    VM {
        instructions: parse(
            "
            push 68 42
            halt
            ",
        ),

        instruction_index: 4,
        stack: vec![42, 68],
        ..Default::default()
    }
    .run_and_asset();
}

#[test]
fn add() {
    VM {
        instructions: parse(
            "
            add 1 2
            halt
        ",
        ),

        instruction_index: 5,
        stack: vec![3],
        ..Default::default()
    }
    .run_and_asset();
}

#[test]
fn pop() {
    VM {
        instructions: parse(
            "
            pop 42
            halt
        ",
        ),

        instruction_index: 3,
        ..Default::default()
    }
    .run_and_asset();
}

#[test]
fn dupe() {
    VM {
        instructions: parse(
            "
            dupe 42
            halt
        ",
        ),

        instruction_index: 3,
        stack: vec![42, 42],
        ..Default::default()
    }
    .run_and_asset();
}

#[test]
fn jump() {
    VM {
        instructions: parse(
            "
            jump #end
            #middle
            halt
            #end
            jump #middle
        ",
        ),

        instruction_index: 3,
        ..Default::default()
    }
    .run_and_asset();
}

#[test]
fn jump_if() {
    VM {
        instructions: parse(
            "
            push 0
            jump_if #foo
            #bar
            pop
            #foo
            push 1
            jump_if #bar
            halt
        ",
        ),

        instruction_index: 11,
        ..Default::default()
    }
    .run_and_asset();
}

#[test]
fn load_uninitialized() {
    VM {
        instructions: parse(
            "
            load &a
            halt
        ",
        ),

        instruction_index: 3,
        stack: vec![0],
        ..Default::default()
    }
    .run_and_asset();
}

#[test]
fn store() {
    VM {
        instructions: parse(
            "
            store &a 42
            halt
        ",
        ),

        instruction_index: 5,
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
        instructions: parse(
            "
            store &a 42
            load &a
            halt
        ",
        ),

        instruction_index: 8,
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
        instructions: parse(
            "
            store &a 6
            store &b 4

            load &a
            load &b
            gt
            jump_if #else

            load &a
            store &c
            jump #done

            #else
            load &b
            store &c

            #done
            halt
        ",
        ),

        instruction_index: 35,
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
        instructions: parse(
            "
            store &a 6
            store &b 4
            store &total 0

            #while
            load &b
            geq 1
            jump_if #break

            load &a
            load &total
            add
            store &total

            load &b
            sub 1
            store &b
            jump #while

            #break
            halt
        ",
        ),

        instruction_index: 46,
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
        instructions: parse(
            "
            call #func
            halt
            #func
            return
        ",
        ),
        instruction_index: 3,
        ..Default::default()
    }
    .run_and_asset();
}

#[test]
fn call_ret_const() {
    VM {
        instructions: parse(
            "
            call #func
            halt
            #func
            return 7
        ",
        ),

        instruction_index: 3,
        stack: vec![7],
        ..Default::default()
    }
    .run_and_asset();
}

#[test]
fn call_ret_double() {
    VM {
        instructions: parse(
            "
            call #func 3
            halt
            #func
            mul 2
            return
        ",
        ),

        instruction_index: 5,
        stack: vec![6],
        ..Default::default()
    }
    .run_and_asset();
}

#[test]
fn max() {
    VM {
        instructions: parse(
            "
            call #max 4 6
            halt

            #max
            store &b
            store 0

            load &a
            load &b
            gt
            jump_if #else

            load &a
            return

            #else
            load &b
            return
        ",
        ),

        instruction_index: 7,
        stack: vec![6],
        ..Default::default()
    }
    .run_and_asset()
}

#[test]
fn large_number() {
    VM {
        instructions: parse(
            "
            push 4_294_967_295_u32
            halt
        ",
        ),
        instruction_index: 5,
        stack: vec![255, 255, 255, 255],
        ..Default::default()
    }
    .run_and_asset()
}
