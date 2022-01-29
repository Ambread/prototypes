pub fn compile(input: &str) -> String {
    let n: usize = input[14..(input.len() - 3)].parse().unwrap();
    format!("function w $main() {{ @start %r =w call $puts(l $str) ret {n} }}")
}

#[cfg(test)]
mod test {
    use proptest::prelude::*;

    use crate::compile;

    proptest! {
        #[test]
        fn compile_number(n: usize) {
            let input = format!("func main() {{ {n} }};");
            let expected = format!("function w $main() {{ @start %r =w call $puts(l $str) ret {n} }}");
            assert_eq!(expected, compile(&input));
        }
    }
}
