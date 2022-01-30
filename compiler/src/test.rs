use indoc::formatdoc;

use crate::compile;

#[test]
fn compile_number() {
    let name = "main";
    let number = 12345;

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
