#![warn(clippy::all, clippy::pedantic, clippy::nursery, rustdoc::all)]

use default_kwargs::{default_args, keyword_args};

#[derive(Debug, PartialEq)]
struct MyStruct(i32);

default_args! {
    fn thing(x: i32, MyStruct(y): MyStruct) -> (i32, i32) {
        (x, y)
    }
}

fn main() {
    let (r, s) = keyword_args! { thing(666, MyStruct(69)) };
    assert_eq!(r, 666);
    assert_eq!(s, 69);
}
