use std::io;

fn main() {
    // Create a mutable String to store user input
    let mut input = String::new();

    // Prompt user for a number
    println!("Enter a number:");
    io::stdin().read_line(&mut input).expect("Failed to read input");

    // Convert input string to integer
    let num: i32 = input.trim().parse().expect("Please enter a valid integer");

    // Check if the number is positive, negative, or zero
    if num > 0 {
        println!("The number is positive.");
    } else if num < 0 {
        println!("The number is negative.");
    } else {
        println!("The number is zero.");
    }
}

