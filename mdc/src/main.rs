fn main() {

    // Calculate the Greatest Common Divisor - GCD, of two numbers using the Euclidean algorithm
    // Example: GCD of 15 and 20
    // The GCD of 15 and 20 is 5
    println!("Calculating GCD using the Euclidean algorithm...");
    let mut a = 15;
    let mut b = 20;
    while b != 0 {
        let temp = a % b;
        a = b;
        b = temp;
        
    }
    println!("The GCD is {}", a);
}
