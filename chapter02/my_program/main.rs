// Note: In the older Rust 2015 edition, submodules don't need a sibling foo.rs
// alongside the foo directory, and instead use a mod.rs file within foo to convey to
// the compiler that the directory is a module. Both of these approaches are
// supported in Rust 2018 edition.

mod foo;
use foo::Bar;
fn main() {
    foo::do_foo();
    Bar::hello();
}
