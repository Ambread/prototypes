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

#[test]
fn let_checks() {
    assert_eq!(
        infer(let_in(
            "foo",
            call(var("even?"), number(123)),
            if_else(var("foo"), number(456), number(789))
        )),
        Ok(ty_name("Number"))
    );

    assert_eq!(
        infer(let_in(
            "foo",
            func("bar", call(var("even?"), call(var("inc"), var("bar")))),
            call(var("foo"), number(123))
        )),
        Ok(ty_name("Bool"))
    );

    assert_eq!(
        infer(let_in("foo", var("even?"), call(var("foo"), number(123)))),
        Ok(ty_name("Bool"))
    );

    assert_eq!(
        infer(let_in(
            "foo",
            var("any"),
            if_else(var("any"), number(456), var("any"))
        )),
        Err(err_mismatch(ty_name("Bool"), ty_name("Number")))
    );
}

#[test]
fn let_mismatch() {
    assert_eq!(
        infer(let_in(
            "foo",
            number(123),
            if_else(var("foo"), number(456), number(789))
        )),
        Err(err_mismatch(ty_name("Bool"), ty_name("Number")))
    );

    assert_eq!(
        infer(let_in(
            "foo",
            var("any"),
            if_else(var("any"), number(456), var("any"))
        )),
        Err(err_mismatch(ty_name("Bool"), ty_name("Number")))
    );
}
