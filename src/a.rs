
fn main() {
    let value = Some(String::from("hello"));

    match value {
        Some(ref s) => println!("Got a reference to {}", s),
        None => println!("Got nothing"),
    }
}