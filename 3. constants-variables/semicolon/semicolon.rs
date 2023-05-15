// Take a closer look at the preceding program (if-expression.rs); it highlights an important detail regarding the
// semicolon and blocks. The semicolon is not optional in Rust, but it has a specific meaning.
// The last expression of a block is the one whose value is returned out of a block, and the
// absence of the semicolon in the last line is important; if we were to add a semicolon after the
// strings in the if blocks, Rust would interpret it as you wanting to throw the value away:
// In this case, the result will be empty, which is why we had to change the println!
// expression slightly; this type cannot be printed out in the regular way

fn main() {
    let result = if 1 == 2 {
        "Nothing makes sense";
    } else {
        "Sanity reigns";
    };
    println!("Result of computation: {:?}", result); // result () this is why is it needed :?
}
