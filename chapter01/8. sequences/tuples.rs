// Tuples differ from arrays in the way that arrays are sequences of the same type, while tuple
// elements have varying types:
// They are useful for simple, type-safe compounding of data, generally used when returning
// multiple values from a function.
fn main() {
    let number_and_string: (u8, &str) = (40, "a static string");
    println!("Number and string in a tuple: {:?}", number_and_string);
}
