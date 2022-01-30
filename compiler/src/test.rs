use indoc::indoc;

use crate::compile;

#[test]
fn compile_number() {
    let input = indoc!(
        "
        func test() {
            1234
        };
        "
    );

    let expected = indoc!(
        "
        export function w $test() {
        @start
            ret 1234
        }
        "
    );

    assert_eq!(expected, compile(input));
}

#[test]
fn multiple_expressions() {
    let input = indoc!(
        "
        func test() {
            123;
            456;
            789
        };
        "
    );

    let expected = indoc!(
        "
        export function w $test() {
        @start
            ret 789
        }
        "
    );

    assert_eq!(expected, compile(input));
}
