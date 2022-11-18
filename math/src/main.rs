fn main() {
    println!("Hello, world!");
}

macro_rules! op_struct {
    ($($ident:ident => $op:tt),*) => {$(
        #[derive(Debug, Clone, Copy)]
        struct $ident<L, R>(L, R);

        impl<L, R> Value for $ident<L, R>
        where
            L: Value,
            R: Value,
        {
            fn at(&self, x: f64) -> f64 {
                self.0.at(x) $op self.1.at(x)
            }
        }
    )*};
}

op_struct!(Add => +, Sub => -, Mul => *, Div => /);

macro_rules! impl_ops {
    ($($ident:ident $(< $($generic:ident),+ >)? $(,)* )+) => {$(
        impl<__R: Value, $($($generic),+)? > std::ops::Add<__R> for $ident $(< $($generic),+ >)? {
            type Output = Add<Self, __R>;
            fn add(self, rhs: __R) -> Self::Output {
                Add(self, rhs)
            }
        }
    )*};
}

impl_ops!(X, Const, Add<L, R>);

macro_rules! operators {
    () => {};
}

operators!();

trait Value {
    fn at(&self, x: f64) -> f64;
}

#[derive(Debug, Clone, Copy)]
struct X;

impl Value for X {
    fn at(&self, x: f64) -> f64 {
        x
    }
}

#[derive(Debug, Clone, Copy)]
struct Const(f64);

impl Value for Const {
    fn at(&self, _: f64) -> f64 {
        self.0
    }
}
