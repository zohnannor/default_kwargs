#![warn(clippy::all, clippy::pedantic, clippy::nursery, rustdoc::all)]

use default_kwargs::{default_args, keyword_args};

default_args! {
    fn thing(x: i32) -> i32 {
        x
    }
}

fn main() {
    let r = keyword_args! { thing(42) };
    assert_eq!(r, 42);
}
