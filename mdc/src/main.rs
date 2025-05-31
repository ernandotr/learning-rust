use std::io::{self, Write};

fn read_input(prompt: &str) -> i32 {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().parse::<i32>().expect("Please enter a valid number")
}

fn main() {

    // Calculate the Greatest Common Divisor - GCD, of two numbers using the Euclidean algorithm
    // Example: GCD of 15 and 20
    // The GCD of 15 and 20 is 5
    println!("Calculating GCD using the Euclidean algorithm...");
    let mut a = read_input("Enter the first number: ");
    let mut b = read_input("Enter the second number: ");
    
    while b != 0 {
        let temp = a % b;
        a = b;
        b = temp;
        
    }
    println!("The GCD is {}", a);
}
