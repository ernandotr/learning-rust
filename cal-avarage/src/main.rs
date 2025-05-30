use  std::io;

fn read_int() -> i32 {
    // Function to read an integer from standard input
    // It reads a line, trims it, and attempts to parse it into an i32.
    // If parsing fails, it returns 0.
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    convert_to_int(&input)
    
}

// Function to convert a string to an integer
// This function attempts to parse a string into an i32.
// If parsing fails, it returns a default value of 0.
// It trims any whitespace from the input string before parsing.
fn convert_to_int(input: &str) -> i32 {
    match input.trim().parse::<i32>() {
        Ok(num) => num,
        Err(_) => 0, // Default value if parsing fails
    }
    
}

fn main() {

    println!("Enter the number of grades:");
    let number = read_int();
    let mut sum = 0;

    let mut  count = 0;
    while count <  number {
    println!("Enter grade {}:", count + 1);
    // Read the grade from input
        let grade  = read_int();
        sum += grade;
        count += 1;
    }

    // Print the sum of the grades
    println!("Sum of grades: {}", sum);

    // Check if the number of grades is valid
    if number == 0 {
        println!("No grades entered.");
        return;
    }

    // Check if the number of grades is negative
    if number < 0 {
        println!("Number of grades cannot be negative.");
        return;
    }

    // Calculate the average of the grades
    // The average is calculated by dividing the sum of grades by the number of grades.
    if number == 0 {
        println!("No grades to calculate average.");
        return;
    }
    if sum < 0 {
        println!("Sum of grades cannot be negative.");
        return;
    }
    if sum == 0 {
        println!("Sum of grades is zero, average cannot be calculated.");
        return;
    }
    if sum > 1000 {
        println!("Sum of grades is too high, average cannot be calculated.");
        return;
    }
    let avarage = sum as f32 / number as f32;
    println!("Average grade: {:.2}", avarage);
    

}
