#![allow(unused)]
#![warn(clippy::invisible_lifetimes)]

fn main() {}

// Struct with generic lifetime
struct SecretStr<'a>(&'a str);

fn input_value(_: SecretStr) {}

fn identity(s: SecretStr) -> SecretStr {
    s
}

// We don't want to activate on references
mod false_positive {
    fn input_value(_: &i32) {}

    fn identity(i: &i32) -> &i32 {
        i
    }
}

// Input and output are associated, but this is not clear!
// Want to lint on this and suggest <'_>
fn make_secret(s: &str) -> SecretStr {
    SecretStr(s)
}

// Input and output are associated, but the relationship is not clear!
// Want to lint on this and suggest <'_>
fn reveal_secret(s: SecretStr) -> &str {
    s.0
}

// Input and output is not associated!
// Want to lint on this and suggest <'static>?
fn confusing_lifetimes(s: &str) -> SecretStr {
    let _ = s; // Imagine we do something with this
    SecretStr("Hello world")
}
