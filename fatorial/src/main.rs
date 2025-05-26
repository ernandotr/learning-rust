use std::io;

// This program calculates the factorial of a given integer using a while loop.
// It reads an integer from the user, calculates its factorial, and prints the result.
// The factorial of a number n is the product of all positive integers less than or equal to n.
// For example, the factorial of 5 (denoted as 5!) is 5 * 4 * 3 * 2 * 1 = 120.
// The program handles input and output using the standard library's io module.


// The read_int function reads an integer from standard input and returns it.
fn read_int() -> i64 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let num = input.trim().parse::<i64>().expect("Please enter a valid integer");
    num
}

// The main function orchestrates the reading of input, calculation of the factorial, and printing of the result.
fn main() {
    println!("Enter the first integer:");
    let mut number: i64 = read_int();

    let mut fatorial: i64 = 1;

    while number > 1 {
        fatorial *= number;
        number -= 1;
        
    }
    println!("The factorial is: {}", fatorial);
}
