// Slices offer a way to safely point to a continuous range in an existing data structure. The
// type of slices is &[T]. Its syntax looks similar to arrays:

fn main() {
    let numbers: [u8; 4] = [1, 2, 4, 5];
    let all_numbers_slice: &[u8] = &numbers[..];
    let first_two_numbers: &[u8] = &numbers[0..2];
    println!("All numbers: {:?}", all_numbers_slice);
    println!(
        "The second of the first two numbers: {}",
        first_two_numbers[1]
    );
}
