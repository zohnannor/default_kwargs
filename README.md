# Rust default and keyword macro

[![github](https://img.shields.io/badge/github-zohnannor/default__kwargs-946d8c?logo=github)](https://github.com/zohnannor/default_kwargs)
[![Crates.io](https://img.shields.io/crates/v/syn?color=orange&logo=rust)](https://crates.io/crates/default_kwargs)
[![docs.rs](https://img.shields.io/docsrs/syn?color=12aaaa&logo=data:image/svg+xml;base64,PHN2ZyByb2xlPSJpbWciIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyIgdmlld0JveD0iMCAwIDUxMiA1MTIiPjxwYXRoIGZpbGw9IiNmNWY1ZjUiIGQ9Ik00ODguNiAyNTAuMkwzOTIgMjE0VjEwNS41YzAtMTUtOS4zLTI4LjQtMjMuNC0zMy43bC0xMDAtMzcuNWMtOC4xLTMuMS0xNy4xLTMuMS0yNS4zIDBsLTEwMCAzNy41Yy0xNC4xIDUuMy0yMy40IDE4LjctMjMuNCAzMy43VjIxNGwtOTYuNiAzNi4yQzkuMyAyNTUuNSAwIDI2OC45IDAgMjgzLjlWMzk0YzAgMTMuNiA3LjcgMjYuMSAxOS45IDMyLjJsMTAwIDUwYzEwLjEgNS4xIDIyLjEgNS4xIDMyLjIgMGwxMDMuOS01MiAxMDMuOSA1MmMxMC4xIDUuMSAyMi4xIDUuMSAzMi4yIDBsMTAwLTUwYzEyLjItNi4xIDE5LjktMTguNiAxOS45LTMyLjJWMjgzLjljMC0xNS05LjMtMjguNC0yMy40LTMzLjd6TTM1OCAyMTQuOGwtODUgMzEuOXYtNjguMmw4NS0zN3Y3My4zek0xNTQgMTA0LjFsMTAyLTM4LjIgMTAyIDM4LjJ2LjZsLTEwMiA0MS40LTEwMi00MS40di0uNnptODQgMjkxLjFsLTg1IDQyLjV2LTc5LjFsODUtMzguOHY3NS40em0wLTExMmwtMTAyIDQxLjQtMTAyLTQxLjR2LS42bDEwMi0zOC4yIDEwMiAzOC4ydi42em0yNDAgMTEybC04NSA0Mi41di03OS4xbDg1LTM4Ljh2NzUuNHptMC0xMTJsLTEwMiA0MS40LTEwMi00MS40di0uNmwxMDItMzguMiAxMDIgMzguMnYuNnoiPjwvcGF0aD48L3N2Zz4K)](https://docs.rs/default_kwargs)
[![GitHub Workflow Status](https://img.shields.io/github/workflow/status/zohnannor/default_kwargs/test)](https://github.com/zohnannor/default_kwargs/actions?query=branch%3Amain)

## Example

Basic example of using default arguments:

```rust
use default_kwargs::{default_args, keyword_args};

default_args! {
    fn thing(x: i32, y: i32 = 42) -> (i32, i32) {
        (x, y)
    }
}

fn main() {
    let (r, s) = keyword_args! { thing(42) };
    assert_eq!(r, 42);
    assert_eq!(s, 42);

    let (r, s) = keyword_args! { thing(42, y = 666) };
    assert_eq!(r, 42);
    assert_eq!(s, 666);
}
```

Like in the most languages that have this feature, positional argument have to
come before any arguments with default value.

## Limitations

- No variadics (`fn foo(a: f64, ...)`)
- Complex patterns don't work, i.e. `Point(x, y): Point = Point(5, 20)` would
  produce an error.
- You will have to pass defaulted arguments only using keywords. That is, you
  can't do this:

  ```rust,compile_fail
  use default_kwargs::{default_args, keyword_args};

  default_args! {
      fn foo(x: f64 = 3.14) {}
  }

  fn main() {
      keyword_args! { foo(2.71) } // error, do `foo(x = 2.71)` instead.
  }
  ```

- At least for now, it is required that you use full function path in the
  `keyword_args` macro. The reason is that we can't get the full path to the
  args struct from the name of the function. This might change in the future.

## How does it work

Basically, the `default_args` macro generates a new struct and implements
`Default` for it based on function's name.
The example above expands to roughly this:

```rust
struct ThingArgs {
    y: i32,
}

impl Default for ThingArgs {
    fn default() -> Self {
        Self { y: 42 }
    }
}

fn thing(x: i32, ThingArgs { y }: ThingArgs) -> (i32, i32) {
    (x, y)
}
```

And `keyword_args` does the opposite:

```rust,ignore
fn main() {
    let (r, s) = thing(
        42,
        ThingArgs {
            ..ThingArgs::default()
        },
    );
}
```

<!-- TODO: add "Contribution" section -->

## Credits

Thank you [@dtolnay](https://github.com/dtolnay) for an amazing work in parsing
and macro ecosystems:

- [syn](https://github.com/dtolnay/syn)
- [quote](https://github.com/dtolnay/quote)
- [cargo-expand](https://github.com/dtolnay/cargo-expand)
- and [_many_](https://crates.io/users/dtolnay?sort=downloads) others!

## License

[MIT](./LICENSE-MIT) or [Apache License, Version 2.0](./LICENSE-APACHE) at your
option.
