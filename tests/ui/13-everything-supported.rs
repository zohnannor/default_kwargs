#![warn(clippy::all, clippy::pedantic, clippy::nursery, rustdoc::all)]

use default_kwargs::default_args;

default_args! {
    #[must_use]
    #[allow(unused, clippy::unnecessary_wraps)]
    #[allow(clippy::toplevel_ref_arg, clippy::extra_unused_lifetimes)]
    pub(in self) unsafe fn unsafe_thingy<'asd, T: Sized>(
        #[allow(unused)] ref mut _x: &mut (),
        ref mut y: T = T::default(),
    ) -> Option<*mut T>
    where
        T: Default + Copy,
    {
        Some(Box::into_raw(Box::new(*y)))
    }
}

default_args! {
    #[must_use]
    #[allow(unused, clippy::double_must_use, clippy::unused_async, clippy::future_not_send)]
    pub(in self) async unsafe fn async_thing<T>(
        _x: (), y: T = T::default()) -> Option<T>
    where T: Default,
    {
        Some(y)
    }
}

fn main() {}
