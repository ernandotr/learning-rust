use  std::io;

// Function to convert a string to an integer
// This function attempts to parse a string into an i32.
// If parsing fails, it returns a default value of 0.
// It trims any whitespace from the input string before parsing.
fn convert_to_inte(input: &str) -> i32 {
    match input.trim().parse::<i32>() {
        Ok(num) => num,
        Err(_) => 0, // Default value if parsing fails
    }
    
}

fn main() {
    // Prompt the user for input
    println!("Please enter a number:");

    // Read input from the user
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    // Convert the input to an integer
    let number = convert_to_inte(&input);

    // Print the converted number
    println!("The converted number is: {}", number);
}
