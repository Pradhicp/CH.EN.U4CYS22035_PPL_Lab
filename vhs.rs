use std::io;
enum FuelType {
    Petrol,
    Diesel,
    Electric,
}

struct Vehicle {
    brand: String,
    model: String,
    fuel_type: FuelType,
}

impl Vehicle {
    fn display_details(&self) {
        let fuel = match &self.fuel_type {
            FuelType::Petrol => "Petrol",
            FuelType::Diesel => "Diesel",
            FuelType::Electric => "Electric",
        };
        println!("Brand: {}, Model: {}, Fuel Type: {}", self.brand, self.model, fuel);
    }
}

fn filter_electric_vehicles(vehicles: &Vec<Vehicle>) -> Vec<&Vehicle> {
    vehicles.iter().filter(|&v| matches!(v.fuel_type, FuelType::Electric)).collect()
}

fn main() {
    let mut vehicles = Vec::new();
    loop {
        println!("Enter vehicle details (or type 'exit' to stop):");

        let mut brand = String::new();
        let mut model = String::new();
        let mut fuel_type_input = String::new();

        println!("Enter vehicle brand:");
        io::stdin().read_line(&mut brand).expect("Failed to read input");
        let brand = brand.trim().to_string();
        if brand.to_lowercase() == "exit" { break; }

        println!("Enter vehicle model:");
        io::stdin().read_line(&mut model).expect("Failed to read input");
        let model = model.trim().to_string();
        if model.to_lowercase() == "exit" { break; }

        println!("Enter fuel type (Petrol, Diesel, Electric):");
        io::stdin().read_line(&mut fuel_type_input).expect("Failed to read input");
        let fuel_type_input = fuel_type_input.trim().to_lowercase();

        let fuel_type = match fuel_type_input.as_str() {
            "petrol" => FuelType::Petrol,
            "diesel" => FuelType::Diesel,
            "electric" => FuelType::Electric,
            _ => {
                println!("Invalid fuel type. Try again.");
                continue;
            }
        };

        vehicles.push(Vehicle {
            brand,
            model,
            fuel_type,
        });
    }

    let electric_vehicles = filter_electric_vehicles(&vehicles);
    println!("\nElectric Vehicles:");
    for vehicle in electric_vehicles {
        vehicle.display_details();
    }
}

