// This is a simple Rust program that prints a greeting message with a name and age.
// You can change the values of `name` and `age` to see different outputs.
// To run this program, use the command: cargo run

fn main() {
    let name: &'static str = "Alice"; // or let name = "Bob";
    let age: i16 = 25; // or let age = 30;
    println!("Hello, {} your age is {} !", name, age);

    let number1 = 7;
    let number2 = 20;

    // Performing various arithmetic operations
    println!("\nArithmetic operations between {} and {}:", number1, number2);
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
    
    // Demonstrating the use of a constant
    const PI: f64 = 3.14159;
    println!("\nThe value of PI is approximately {:.5}.", PI);

    // Demonstrating the use of a mutable variable
    let mut mutable_number = 10;
    println!("\nThe initial value of mutable_number is {}.", mutable_number);
    mutable_number += 5; // Incrementing the mutable variable
    println!("After incrementing, mutable_number is now {}.", mutable_number);

    // Demonstrating the use of a tuple
    let tuple_example: (i32, f64, char) = (42, 3.14, 'x');
    println!("\nTuple example: {:?}", tuple_example);
    println!("The first element of the tuple is {}.", tuple_example.0);
    println!("The second element of the tuple is {}.", tuple_example.1);
    println!("The third element of the tuple is {}.", tuple_example.2);

    // Demonstrating the use of an array
    let array_example: [i32; 5] = [1, 2, 3, 4, 5];
    println!("\nArray example: {:?}", array_example);
    println!("The first element of the array is {}.", array_example[0]);
    println!("The second element of the array is {}.", array_example[1]);

    // Demonstrating the use of a vector
    let vector_example: Vec<i32> = vec![10, 20, 30, 40, 50];
    println!("\nVector example: {:?}", vector_example);
    println!("The first element of the vector is {}.", vector_example[0]);
    println!("The second element of the vector is {}.", vector_example[1]);

    
}