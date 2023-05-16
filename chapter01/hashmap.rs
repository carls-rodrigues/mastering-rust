use std::collections::HashMap;

fn main() {
    let mut configuration = HashMap::new();
    configuration.insert("path", String::from("/home/user/"));
    println!("Configured path is {:?}", configuration.get("path"));
}
