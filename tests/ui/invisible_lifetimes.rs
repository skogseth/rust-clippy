#![allow(unused)]
#![warn(clippy::invisible_lifetimes)]

fn main() {}

// Struct with generic lifetime
struct MyStruct<'a>(&'a str);

fn input_value(_: MyStruct) {}

fn return_value() -> MyStruct {
    MyStruct("Hello")
}

fn identity(s: MyStruct) -> MyStruct {
    s
}
