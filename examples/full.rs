// compiler_error when we don't have std and don't have alloc
#[cfg(not(feature = "std"))]
compile_error!("This example requires std");

use std::fmt::Display;

#[derive(Hash, Default)]
struct A {
    a: u32,
    b: u32,
}

impl A {
    fn cool() -> Self {
        Self { a: 1, b: 2 }
    }
}

fn main() {
    print(r#"hashmoji::one(42)"#, hashmoji::one(42));
    print(
        r#"hashmoji::one(vec![1, 2, 3])"#,
        hashmoji::one(vec![1, 2, 3]),
    );
    print(
        r#"hashmoji::one(A::default())"#,
        hashmoji::one(A::default()),
    );
    print(r#"hashmoji::one(A::cool())"#, hashmoji::one(A::cool()));
    print(r#"hashmoji::one(())"#, hashmoji::one(()));
    print(r#"hashmoji::one("hashmoji")"#, hashmoji::one("hashmoji"));

    // Similar values will emit same sequence of emojis
    print(
        r#"hashmoji::fixed("hashmoji", 3)"#,
        hashmoji::fixed("hashmoji", 5),
    );
    print(
        r#"hashmoji::variable("hashmoji", 5..10)"#,
        hashmoji::variable("hashmoji", 1..10),
    );

    print(
        "hashmoji::iter().take(30).collect()",
        hashmoji::iter().take(30).collect::<String>(),
    )
}

fn print<'a>(s: &str, v: impl Into<String> + Display + 'a) {
    let s: String = s.into();
    println!("{s:40} -> {v}");
}
