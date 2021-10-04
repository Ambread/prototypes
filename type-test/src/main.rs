use type_test::{builder::*, data::Context};

fn main() {
    let mut context = Context::new(
        IntoIterator::into_iter([
            (
                "even?".to_owned(),
                ty_func(ty_name("Number"), ty_name("Bool")),
            ),
            (
                "inc".to_owned(),
                ty_func(ty_name("Number"), ty_name("Number")),
            ),
        ])
        .collect(),
    );

    let expr = if_else(
        call(var("even?"), call(var("inc"), number(123))),
        number(456),
        number(789),
    );

    let result = expr.infer(&mut context);

    dbg!(result);
}
