use type_test::{builder::*, data::Context};

fn main() {
    let env = [
        (
            "even?".to_owned(),
            ty_func(ty_name("Number"), ty_name("Bool")),
        ),
        (
            "inc".to_owned(),
            ty_func(ty_name("Number"), ty_name("Number")),
        ),
    ];

    let exprs = [
        if_else(
            call(var("even?"), call(var("inc"), number(123))),
            number(456),
            number(789),
        ),
        if_else(number(123), number(123), number(123)),
        call(var("inc"), call(var("even?"), number(123))),
    ];

    let mut context = Context::new(IntoIterator::into_iter(env).collect());
    for expr in exprs {
        let _ = dbg!(expr.infer(&mut context));
    }
}
