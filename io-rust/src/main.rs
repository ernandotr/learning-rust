use std::io;

fn convert_to_int(input: &str) -> i32 {
    // Convert the input string to an integer
    // This will panic if the input is not a valid integer
    // For simplicity, we assume the input is always a valid integer
    // Uncomment the line below to see how it works with a panic
    // This is a simple function that takes a string input,
    // trims any whitespace, and attempts to parse it as an i32.
    // If the input is a valid integer, it will return the integer value.
    // If the input is not a valid integer, it will panic.
    // let converted_value = input.trim().parse::<i32>().unwrap();

    // If the input is not a valid integer, we handle the error gracefully
    // by printing an error message and exiting the program
    // This is a simple way to ensure that the program does not crash
    // when the user provides invalid input.
    // We could also use a more complex error handling mechanism,
    // but for this example, we will keep it simple.
    // This function will return the converted integer or exit the program
    // with an error message if the input is invalid.
    let converted_value = match input.trim().parse::<i32>() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input, please enter a valid integer.");
            std::process::exit(1);
        }
    };
    converted_value
}

fn main() {
    let mut number1:String = String::new();
    println!("Enter the first number:");
    // Read input from the user
    io::stdin().read_line(&mut number1).expect("Failed to read the number1");

    let mut number2:String = String::new();
    println!("Enter the second number:");
    // Read input from the user
    io::stdin().read_line(&mut number2).expect("Failed to read the number2");

    let num1 = convert_to_int(&number1);
    let num2 = convert_to_int(&number2);

    // perfom basic comparison operations
    println!("Basic Comparison Operations:");
    println!("Comparing {} and {}", num1, num2);
    // Compare the two numbers and print the result
    if num1 > num2 {
        println!("{} is greater than {}", num1, num2);
    } else if num1 < num2 {
        println!("{} is less than {}", num1, num2);
    } else {
        println!("{} is equal to {}", num1, num2);
    }
    // Perform basic arithmetic operations
    println!("Basic Arithmetic Operations:");
    println!("{} + {} = {}", num1, num2, num1 + num2);
    println!("{} - {} = {}", num1, num2, num1 - num2);
    println!("{} * {} = {}", num1, num2, num1 * num2);
    if num2 != 0 {
        println!("{} / {} = {}", num1, num2, num1 / num2);
        println!("{} % {} = {}", num1, num2, num1 % num2);
    } else {
        println!("Division by zero is not allowed.");
    }


}
