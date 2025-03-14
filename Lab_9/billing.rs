use std::io;

fn main() {
    let mut item = String::new();
    let mut quantity_input = String::new();

    println!("Enter the menu item (Burger, Pizza, Pasta):");
    io::stdin().read_line(&mut item).expect("Failed to read input");
    let item = item.trim().to_lowercase(); 

    println!("Enter the quantity:");
    io::stdin().read_line(&mut quantity_input).expect("Failed to read input");
    let quantity: u32 = quantity_input.trim().parse().expect("Please enter a valid number");

    let price_per_item = match item.as_str() {
        "burger" => 150,
        "pizza" => 250,
        "pasta" => 200,
        _ => {
            println!("Invalid item selected.");
            return;
        }
    };
    let total_price = price_per_item * quantity;
    let final_price = match quantity {
        1..=2 => total_price, 
        3..=5 => (total_price as f32 * 0.9) as u32, 
        6..=10 => (total_price as f32 * 0.85) as u32, 
        _ => (total_price as f32 * 0.8) as u32, 
    };

    println!("Total price after discount: ₹{}", final_price);
}
