#![warn(clippy::all, clippy::pedantic, clippy::nursery, rustdoc::all)]

mod first {
    use default_kwargs::default_args;

    default_args! {
        pub fn nested(x: () = ()) { x }
    }
}

mod second {
    use default_kwargs::{default_args, keyword_args};

    use super::first::nested;

    default_args! {
        pub(super) fn plain(u: () = ()) {
            keyword_args! { nested() }
            // right now you need to do this:
            keyword_args! { super::first::nested() }
        }
    }
}

fn main() {
    use default_kwargs::keyword_args;
    keyword_args! { second::plain(u = ()) };
}
