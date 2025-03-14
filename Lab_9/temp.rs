fn average_temperature(temps: &Vec<f32>) -> f32 {
    let sum: f32 = temps.iter().sum();
    sum / temps.len() as f32
}

fn min_max_temperature(temps: &Vec<f32>) -> (f32, f32) {
    let min_temp = temps.iter().cloned().fold(f32::INFINITY, f32::min);
    let max_temp = temps.iter().cloned().fold(f32::NEG_INFINITY, f32::max);
    (min_temp, max_temp)
}

fn main() {
    let temperatures = vec![32.5, 31.0, 35.2, 36.8, 30.1, 33.6, 34.0];

    let avg_temp = average_temperature(&temperatures);
    let (min_temp, max_temp) = min_max_temperature(&temperatures);

    println!("Average Temperature: {:.2}°C", avg_temp);
    println!("Lowest Temperature: {:.2}°C", min_temp);
    println!("Highest Temperature: {:.2}°C", max_temp);
}

