#![warn(clippy::all, clippy::pedantic, clippy::nursery, rustdoc::all)]

use default_kwargs::{default_args, keyword_args};

default_args! {
    fn thing(x: i32, y: i32) -> (i32, i32) {
        (x, y)
    }
}

fn main() {
    let (r, s) = keyword_args! { thing(y = 666) };
}
