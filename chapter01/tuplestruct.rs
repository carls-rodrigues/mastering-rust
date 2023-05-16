// For defining custom data types, there are structs. The simpler form is called a tuple struct,
// where the individual fields are not named but are referred to by their position. This should
// mostly be used when your data consists of only one or a few fields to achieve better levels
// of type safety.

#[derive(PartialEq)]
struct Fahrenheit(i64);

#[derive(PartialEq)]
struct Celsius(i64);

fn main() {
    let temperature1 = Fahrenheit(10);
    let temperature2 = Celsius(10);

    // println!(
    //     "Is temperature 1 the same as temperature 2? Answer: {}",
    //     temperature1 == temperature2
    // ); // will throw an error because the types are different

    //   What is inside the tuple struct can be accessed by the .<number> operation, where the
    //   number refers to the position of the field in the struct
    println!("Temperature 1 is {} fahrenheit", temperature1.0);
    println!("Temperature 2 is {} celsius", temperature2.0);
}
