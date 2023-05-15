// In Rust, if is not a statement but an expression. This distinction means that if always
// returns a value. The value may be an empty type that you don't have to use, or it may be an
// actual value. This means that you can use the if expression as tertiary expressions are used
// in some languages:

fn main() {
    let result = if 1 == 2 {
        "Nothing makes sense"
    } else {
        "Sanity reigns"
    };
    println!("Result of computation: {}", result);
}
