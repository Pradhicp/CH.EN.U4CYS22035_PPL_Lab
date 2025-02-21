use std::io;

fn main() {
    // Create a mutable String to store user input
    let mut input1 = String::new();
    let mut input2 = String::new();

    // Prompt user for first number
    println!("Enter the first number:");
    io::stdin().read_line(&mut input1).expect("Failed to read input");

    // Prompt user for second number
    println!("Enter the second number:");
    io::stdin().read_line(&mut input2).expect("Failed to read input");

    // Convert input strings to integers
    let a: i32 = input1.trim().parse().expect("Please enter a valid integer");
    let b: i32 = input2.trim().parse().expect("Please enter a valid integer");

    // Perform arithmetic operations
    println!("Addition: {}", a + b);
    println!("Subtraction: {}", a - b);
    println!("Multiplication: {}", a * b);

    // Handle division and modulo carefully to avoid division by zero
    if b != 0 {
        println!("Division: {}", a / b);
        println!("Modulo: {}", a % b);
    } else {
        println!("Division and Modulo are not possible with zero as the second number.");
    }
}

