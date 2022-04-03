#![warn(clippy::all, clippy::pedantic, clippy::nursery, rustdoc::all)]

use default_kwargs::default_args;

unsafe extern "C" fn variadic_fn(a: i32, ...) {}

default_args! {
    unsafe extern "C" fn variadic_kw(a: i32, ...) {}
}

fn main() {}
