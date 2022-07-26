use sonance::vm::{Instruction::*, VM};

fn main() {
    let instructions = vec![In, Out, Halt];
    let mut vm = VM::new(instructions);
    vm.run().unwrap();
}
