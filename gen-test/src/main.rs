fn main() {
    let gen = Generator::new(|| {
        println!("first");
        1.proceed(|| {
            println!("second");
            2.proceed(|| {
                println!("third");
                3.end()
            })
        })
    });

    for item in gen {
        println!("{:?}", item);
    }
}

struct Generator<'a, T> {
    next: Option<Box<dyn FnOnce() -> Emit<'a, T> + 'a>>,
}

impl<'a, T> Generator<'a, T> {
    fn new<F>(next: F) -> Self
    where
        F: FnOnce() -> Emit<'a, T> + 'a,
    {
        Self {
            next: Some(Box::new(next)),
        }
    }
}

impl<T> Iterator for Generator<'_, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let emit = self.next.take()?();
        self.next = emit.next;
        Some(emit.value)
    }
}

struct Emit<'a, T> {
    value: T,
    next: Option<Box<dyn FnOnce() -> Emit<'a, T> + 'a>>,
}

trait EmitExt {
    fn proceed<'a, F>(self, next: F) -> Emit<'a, Self>
    where
        Self: std::marker::Sized,
        F: FnOnce() -> Emit<'a, Self> + 'a,
    {
        Emit {
            value: self,
            next: Some(Box::new(next)),
        }
    }

    fn end<'a>(self) -> Emit<'a, Self>
    where
        Self: std::marker::Sized,
    {
        Emit {
            value: self,
            next: None,
        }
    }
}

impl<T> EmitExt for T {}
