// This is a simple Rust program that prints a greeting message with a name and age.
// You can change the values of `name` and `age` to see different outputs.
// To run this program, use the command: cargo run

fn main() {
    let name: &'static str = "Alice"; // or let name = "Bob";
    let age: i16 = 25; // or let age = 30;
    println!("Hello, {} your age is {} !", name, age);
}
