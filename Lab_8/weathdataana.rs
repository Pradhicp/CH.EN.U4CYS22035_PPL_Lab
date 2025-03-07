fn calculate_average(temps: &[f32]) -> f32 {
    let sum: f32 = temps.iter().sum();
    sum / temps.len() as f32
}

fn main() {
    let weekly_temperatures: [f32; 7] = [28.5, 30.0, 31.2, 29.8, 32.1, 33.0, 29.5];

    let last_three_days = &weekly_temperatures[4..7];
    let avg_temp = calculate_average(last_three_days);

    println!("Average temperature of the last three days: {:.2}°C", avg_temp);

    match weekly_temperatures.get(10) {
        Some(&temp) => println!("Temperature at index 10: {}°C", temp),
        None => println!("Error: Attempted to access an out-of-bounds index!"),
    }
}

