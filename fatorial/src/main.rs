use std::io;

fn read_int() -> i64 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let num = input.trim().parse::<i64>().expect("Please enter a valid integer");
    num
}

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
