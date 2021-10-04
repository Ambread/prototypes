use crate::builder::*;

#[test]
fn checks() {
    assert_eq!(
        infer(if_else(
            call(var("even?"), call(var("inc"), number(123))),
            number(456),
            number(789),
        )),
        Ok(ty_name("Number"))
    );
}

#[test]
fn type_mismatch() {
    assert_eq!(
        infer(if_else(number(123), number(123), number(123))),
        Err(err_mismatch(ty_name("Bool"), ty_name("Number")))
    );

    assert_eq!(
        infer(call(var("inc"), call(var("even?"), number(123)))),
        Err(err_mismatch(ty_name("Number"), ty_name("Bool")))
    );
}
