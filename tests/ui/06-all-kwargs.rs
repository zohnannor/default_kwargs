#![warn(clippy::all, clippy::pedantic, clippy::nursery, rustdoc::all)]

use default_kwargs::{default_args, keyword_args};

default_args! {
    fn thing(x: i32 = 42, y: i32 = 69) -> (i32, i32) {
        (x, y)
    }
}

fn main() {
    let (r, s) = keyword_args! { thing() };
    assert_eq!(r, 42);
    assert_eq!(s, 69);

    let (r, s) = keyword_args! { thing(y = 666) };
    assert_eq!(r, 42);
    assert_eq!(s, 666);

    let (r, s) = keyword_args! { thing(y = 666, x = 123) };
    assert_eq!(r, 123);
    assert_eq!(s, 666);
}
