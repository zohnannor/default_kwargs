#![warn(clippy::all, clippy::pedantic, clippy::nursery, rustdoc::all)]

use default_kwargs::{default_args, keyword_args};

use std::ops::Add;

default_args! {
    fn generic<T>(t: T = T::default()) -> Option<T>
    where T: Default + Add<Output=T> + Copy,
    {
        Some(t + t)
    }
}

fn main() {
    match keyword_args! { generic::<i32>() } {
        Some(thing) => assert_eq!(thing, 0),
        None => panic!("something is wrong..."),
    };

    match keyword_args! { generic(t = 21) } {
        Some(thing) => assert_eq!(thing, 42),
        None => panic!("something is wrong..."),
    };
}
