#[test]
fn tests() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/01-empty.rs");
    t.pass("tests/ui/02-basic.rs");
    t.pass("tests/ui/03-default.rs");
    t.compile_fail("tests/ui/04-missing-positional.rs");
    t.compile_fail("tests/ui/05-positional-only.rs");
    t.pass("tests/ui/06-all-kwargs.rs");
    t.pass("tests/ui/07-custom-type.rs");
    t.pass("tests/ui/08-kwarg-custom-type.rs");
    t.pass("tests/ui/09-positional-pattern.rs");
    t.compile_fail("tests/ui/10-kwarg-pattern.rs");
    t.pass("tests/ui/11-generic.rs");
    t.pass("tests/ui/12-generic-with-bounds.rs");
    t.pass("tests/ui/13-everything-supported.rs");
    t.compile_fail("tests/ui/14-unsupported.rs");
    // TODO: fix this test, somehow we need to get struct's path?
    t.compile_fail("tests/ui/15-visibility.rs");
}
