// This is a simple Rust program that prints a greeting message with a name and age.
// You can change the values of `name` and `age` to see different outputs.
// To run this program, use the command: cargo run

use std::f32::consts::SQRT_2;

use num::integer::Roots;

fn main() {
    let name: &'static str = "Alice"; // or let name = "Bob";
    let age: i16 = 25; // or let age = 30;
    println!("Hello, {} your age is {} !", name, age);

    let number1: i32 = 7;
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
    
 // Importing the Pow trait for power operations
    // Ensure you have the num crate in your Cargo.toml
    // Add the following line to your Cargo.toml:
    // [dependencies]
    // num = "0.4" (or the latest version)
    // Demonstrating the use of the pow function
    let power = number1.pow(2); // Squaring the number
    println!("The square of {} is {}.", number1, power);
    let power_of_three = number2.pow(3); // Cubing the number
    println!("The cube of {} is {}.", number2, power_of_three);

    // Demonstrating the use of square root and cube root functions
    let square_root = number1.sqrt(); // Calculating the square root
    println!("The square root of {} is {:.2}.", number1, square_root);
    let cube_root = number2.cbrt(); // Calculating the cube root
    println!("The cube root of {} is {:.2}.", number2, cube_root);
    let sqrt_2 = SQRT_2; // Using a constant from the std::f32::consts module
    println!("The square root of 2 is approximately {:.2}.", sqrt_2);
    let sqrt_2_f64 = SQRT_2 as f64; // Converting to f64
    println!("The square root of 2 as f64 is approximately {:.2}.", sqrt_2_f64);
    let sqrt_2_f32 = SQRT_2 as f32; // Converting to f32
    println!("The square root of 2 as f32 is approximately {:.2}.", sqrt_2_f32);

    
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

    // Demonstrating the use of a string
    let string_example: String = String::from("Hello, Rust!");
    println!("\nString example: {}", string_example);
    println!("The length of the string is {} characters.", string_example.len());
    println!("The first character of the string is '{}'.", string_example.chars().next().unwrap());
    println!("The last character of the string is '{}'.", string_example.chars().last().unwrap());

    // Demonstrating the use of a hash map
    use std::collections::HashMap;
    let mut hash_map_example: HashMap<&str, i32> = HashMap::new();
    hash_map_example.insert("Alice", 25);
    hash_map_example.insert("Bob", 30);
    println!("\nHashMap example: {:?}", hash_map_example);
    println!("Alice's age is {}.", hash_map_example.get("Alice").unwrap());
    println!("Bob's age is {}.", hash_map_example.get("Bob").unwrap());

    // Demonstrating the use of a set
    use std::collections::HashSet;
    let mut hash_set_example: HashSet<&str> = HashSet::new();
    hash_set_example.insert("apple");
    hash_set_example.insert("banana");
    hash_set_example.insert("cherry");
    println!("\nHashSet example: {:?}", hash_set_example);
    println!("Does the set contain 'apple'? {}", hash_set_example.contains("apple"));
    println!("Does the set contain 'grape'? {}", hash_set_example.contains("grape"));

    // Demonstrating the use of an enum
    #[derive(Debug)]
    enum Direction {
        North,
        South,
        East,
        West,
    }
    let direction = Direction::North;
    println!("\nEnum example: {:?}", direction);
    match direction {
        Direction::North => println!("The direction is North."),
        Direction::South => println!("The direction is South."),
        Direction::East => println!("The direction is East."),
        Direction::West => println!("The direction is West."),
    }

    // Demonstrating the use of a struct
    #[derive(Debug)]
    struct Person {
        name: String,
        age: u32,
    }
    let person = Person {
        name: String::from("Alice"),
        age: 30,
    };
    println!("\nStruct example: {:?}", person);
    println!("Person's name is {} and age is {}.", person.name, person.age);

    // Demonstrating the use of a closure
    let add = |x: i32, y: i32| -> i32 {
        x + y
    };
    let result = add(5, 10);
    println!("\nClosure example: The sum of 5 and 10 is {}.", result);

    // Demonstrating the use of a function
    fn multiply(x: i32, y: i32) -> i32 {
        x * y
    }
    let product_result = multiply(6, 7);
    println!("\nFunction example: The product of 6 and 7 is {}.", product_result);

    // Demonstrating error handling with Result
    fn divide(x: i32, y: i32) -> Result<i32, String> {
        if y == 0 {
            Err(String::from("Cannot divide by zero"))
        } else {
            Ok(x / y)
        }
    }
    match divide(10, 2) {
        Ok(result) => println!("\nError handling example: 10 divided by 2 is {}.", result),
        Err(e) => println!("\nError handling example: {}", e),
    }
    match divide(10, 0) {
        Ok(result) => println!("10 divided by 0 is {}.", result),
        Err(e) => println!("Error: {}", e),
    }

    // Demonstrating the use of pattern matching
    let number = 42;
    println!("\nPattern matching example:");
    match number {
        1 => println!("The number is one."),
        2 => println!("The number is two."),
        42 => println!("The answer to life, the universe, and everything is 42!"),
        _ => println!("The number is something else."),
    }

    // Demonstrating the use of a for loop
    println!("\nLoop 'for' example:");
    for i in 1..=5 {
        println!("Iteration number: {}", i);
    }

    // Demonstrating the use of a while loop
    println!("\nLoop 'while' example:");
    let mut count = 0;
    while count < 5 {
        println!("Count is: {}", count);
        count += 1;
    }

    // Demonstrating the use of a loop with break and continue
    println!("\nLoop with break and continue example:");
    let mut i = 0;
    loop {
        i += 1;
        if i == 3 {
            println!("Skipping number 3.");
            continue; // Skip the rest of the loop for this iteration
        }
        if i > 5 {
            println!("Breaking the loop at number {}.", i);
            break; // Exit the loop
        }
        println!("Current number: {}", i);
    }

    // Performing various comparisons
    println!("\nComparisons between {} and {}:", number1, number2);
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
    
    // Checking if the number is even, odd, positive, negative, zero, or non-zero
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

    // Checking if the number is prime
    println!("\nChecking if {} is a prime number:", number1);
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