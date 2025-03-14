use std::io;

fn main() {
    let mut age_input = String::new();
    let mut income_input = String::new();
    
    println!("Enter your age:");
    io::stdin().read_line(&mut age_input).expect("Failed to read line");
    let age: u32 = age_input.trim().parse().expect("Please enter a valid number");

    println!("Enter your monthly income:");
    io::stdin().read_line(&mut income_input).expect("Failed to read line");
    let income: u32 = income_input.trim().parse().expect("Please enter a valid number");

    if age < 21 {
        println!("You are ineligible for the loan.");
    } else if age >= 21 && age <= 60 {
        if income > 50000 {
            println!("You are eligible for the loan.");
        } else {
            println!("Your income is below ₹50,000. You are not eligible for the loan.");
        }
    } else {
        println!("You are above 60. You need a guarantor for the loan.");
    }
}

