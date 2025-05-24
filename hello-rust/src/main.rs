// This is a simple Rust program that prints a greeting message with a name and age.
// You can change the values of `name` and `age` to see different outputs.
// To run this program, use the command: cargo run

fn main() {
    let name: &'static str = "Alice"; // or let name = "Bob";
    let age: i16 = 25; // or let age = 30;
    println!("Hello, {} your age is {} !", name, age);

    let number1 = 7;
    let number2 = 20;
    let sum = number1 + number2;
    println!("The sum of {} and {} is {}.", number1, number2, sum);
    let product = number1 * number2;
    println!("The product of {} and {} is {}.", number1, number2, product);
    let difference = number1 - number2;
    println!("The difference of {} and {} is {}.", number1, number2, difference);
    let quotient = number1 / number2;
    println!("The quotient of {} and {} is {}.", number1, number2, quotient);
    let remainder = number1 % number2;
    println!("The remainder of {} divided by {} is {}.", number1, number2, remainder);
    let is_greater = number1 > number2;
    println!("Is {} greater than {}? {}", number1, number2, is_greater);
    let is_less = number1 < number2;
    println!("Is {} less than {}? {}", number1, number2, is_less);
    let is_equal = number1 == number2;
    println!("Is {} equal to {}? {}", number1, number2, is_equal);
    let is_not_equal = number1 != number2;
    println!("Is {} not equal to {}? {}", number1, number2, is_not_equal);
    let is_greater_or_equal = number1 >= number2;
    println!("Is {} greater than or equal to {}? {}", number1, number2, is_greater_or_equal);
    let is_less_or_equal = number1 <= number2;
    println!("Is {} less than or equal to {}? {}", number1, number2, is_less_or_equal);
    let is_even = number1 % 2 == 0;
    println!("Is {} even? {}", number1, is_even);
    let is_odd = number1 % 2 != 0;
    println!("Is {} odd? {}", number1, is_odd);
    let is_positive = number1 > 0;
    println!("Is {} positive? {}", number1, is_positive);
    let is_negative = number1 < 0;
    println!("Is {} negative? {}", number1, is_negative);
    let is_zero = number1 == 0;
    println!("Is {} zero? {}", number1, is_zero);
    let is_non_zero = number1 != 0;
    println!("Is {} non-zero? {}", number1, is_non_zero);
    let is_prime = is_prime(number1);
    println!("Is {} prime? {}", number1, is_prime); 
}

// Function to check if a number is prime
fn is_prime(n: i32) -> bool {
    if n <= 1 {
        return false;
    }
    for i in 2..=((n as f64).sqrt() as i32) {
        if n % i == 0 {
            return false;
        }
    }
    true
}