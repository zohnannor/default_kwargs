#![warn(clippy::all, clippy::pedantic, clippy::nursery, rustdoc::all)]

use default_kwargs::default_args;

#[derive(Debug, PartialEq)]
struct MyStruct(i32);

default_args! {
    fn thing(x: i32, MyStruct(y): MyStruct = MyStruct(69)) -> (i32, i32) {
        (x, y)
    }
}

fn main() {}
