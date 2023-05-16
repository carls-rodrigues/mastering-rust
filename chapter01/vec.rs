// Vectors are like arrays except that their contents or length don't have to be known in
// advance. They are created with either calling the constructor Vec::new or by using the
// vec! macro:

fn main() {
    let mut numbers_vec: Vec<u8> = Vec::new();
    numbers_vec.push(1);
    numbers_vec.push(2);
    let mut numbers_vec_with_macro = vec![1];
    numbers_vec_with_macro.push(2);
    println!(
        "Both vectors have equal contents: {}",
        numbers_vec == numbers_vec_with_macro
    );
}
