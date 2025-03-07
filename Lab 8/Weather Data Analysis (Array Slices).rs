fn calculate_average(temps: &[f32]) -> f32 { 
    let sum: f32 = temps.iter().sum(); 
    sum / temps.len() as f32 
} 
 
fn main() { 
    let temperatures = [30.0, 32.5, 28.3, 35.0, 33.2, 31.1, 29.0]; 
     
    let last_three_days = &temperatures[4..]; 
     
    println!("Temperatures of last three days: {:?}", last_three_days); 
    println!("Average temperature: {}", calculate_average(last_three_days)); 
}
