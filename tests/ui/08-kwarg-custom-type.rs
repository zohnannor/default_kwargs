#![warn(clippy::all, clippy::pedantic, clippy::nursery, rustdoc::all)]

use default_kwargs::{default_args, keyword_args};

#[derive(Debug, PartialEq)]
struct MyStruct;

default_args! {
    fn thing(x: i32, y: MyStruct = MyStruct) -> (i32, MyStruct) {
        (x, y)
    }
}

fn main() {
    let (r, s) = keyword_args! { thing(666) };
    assert_eq!(r, 666);
    assert_eq!(s, MyStruct);
}
