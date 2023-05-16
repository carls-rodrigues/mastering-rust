// File as a module

mod foo;
use crate::foo::Bar;

fn main() {
    let _bar = Bar::init();
}
