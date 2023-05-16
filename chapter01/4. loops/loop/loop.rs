// Simple loops are programmed with either the while loop (if a condition for the looping is
// wanted) or with loop (if no condition is wanted). The break keyword gets you out of the
// loop.

fn main() {
    let mut x = 1000;
    loop {
        if x < 0 {
            break;
        }
        println!("{} more runs to go", x);
        x -= 1;
    }
}
