// Rust deviates from the mainstream here by making constants the default variable type. If
// you need a variable that can be mutated, you use the let mut keyword
fn main() {
    let mut target = "world";
    println!("Howdy, {}", target);
    target = "mate";
    println!("Howdy, {}", target);
}
