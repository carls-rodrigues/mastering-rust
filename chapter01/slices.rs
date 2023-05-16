// Slices offer a way to safely point to a continuous range in an existing data structure. The
// type of slices is &[T]. Its syntax looks similar to arrays:

// fn main() {
//     let numbers: [u8; 4] = [1, 2, 4, 5];
//     let all_numbers_slice: &[u8] = &numbers[..];
//     let first_two_numbers: &[u8] = &numbers[0..2];
//     println!("All numbers: {:?}", all_numbers_slice);
//     println!(
//         "The second of the first two numbers: {}",
//         first_two_numbers[1]
//     );
// }

fn main() {
    let mut numbers: [u8; 4] = [1, 2, 3, 4];
    {
        let all: &[u8] = &numbers[..];
        println!("All of them: {:?}", all);
    }
    {
        let first_two: &mut [u8] = &mut numbers[0..2];
        first_two[0] = 100;
        first_two[1] = 99;
    }
    println!("Look ma! I can modify through slices: {:?}", numbers);
}
