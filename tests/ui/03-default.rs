#![warn(clippy::all, clippy::pedantic, clippy::nursery, rustdoc::all)]

use default_kwargs::{default_args, keyword_args};

default_args! {
    fn thing(x: i32, y: i32 = 69) -> (i32, i32) {
        (x, y)
    }
}

fn main() {
    let (r, s) = keyword_args! { thing(42) };
    assert_eq!(r, 42);
    assert_eq!(s, 69);

    let (r, s) = keyword_args! { thing(42, y = 666) };
    assert_eq!(r, 42);
    assert_eq!(s, 666);
}
