#![warn(clippy::all, clippy::pedantic, clippy::nursery, rustdoc::all)]

use default_kwargs::{default_args, keyword_args};

#[derive(Debug, PartialEq)]
struct MyStruct;

default_args! {
    fn thing(x: MyStruct, y: i32 = 69) -> (MyStruct, i32) {
        (x, y)
    }
}

fn main() {
    let (r, s) = keyword_args! { thing(MyStruct) };
    assert_eq!(r, MyStruct);
    assert_eq!(s, 69);
}
