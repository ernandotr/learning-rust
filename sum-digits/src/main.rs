use std::io;

// Function to read an integer from standard input and return it
fn read_int() -> i32 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let num = input.trim().parse::<i32>().expect("Please enter a valid integer");
    num
}

// Main function to calculate the sum of digits of an integer
fn main() {
    println!("Enter an integer:");
    let mut number = read_int();
    let mut sum = 0;

    while number !=  0 {
        let rest  = number % 10;
        sum += rest;
        number = number / 10;
    }

    print!("Sum of digits: {}", sum);

}
