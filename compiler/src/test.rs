use indoc::formatdoc;
use proptest::prelude::*;

use crate::compile;

proptest! {
    #[test]
    fn compile_number(name in "[a-zA-Z_]+", number: u32) {
        let input = format!("func {name}() {{ {number} }};");

        let expected = formatdoc!(
            "
                    export function w ${name}() {{
                    @start
                        ret {number}
                    }}
            "
        );

        assert_eq!(expected, compile(&input));
    }
}
