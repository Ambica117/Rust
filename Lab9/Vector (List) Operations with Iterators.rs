fn main() { 
    let temperatures = vec![30, 32, 29, 28, 31, 35, 33]; // Temperatures for the week 
 
    let avg_temp = average_temperature(&temperatures); 
    println!("The average temperature for the week is: {}", avg_temp); 
 
    let (highest, lowest) = find_highest_and_lowest(&temperatures); 
    println!("The highest temperature is: {}, The lowest temperature is: {}", highest, lowest); 
} 
 
fn average_temperature(temperatures: &Vec<i32>) -> f32 { 
    let sum: i32 = temperatures.iter().sum(); // Using iter() to iterate over the vector and sum the temperatures 
    sum as f32 / temperatures.len() as f32 // Return the average 
} 
 
fn find_highest_and_lowest(temperatures: &Vec<i32>) -> (i32, i32) { 
    let highest = *temperatures.iter().max().unwrap(); // Find the highest temperature using iterators 
    let lowest = *temperatures.iter().min().unwrap(); // Find the lowest temperature using iterators 
    (highest, lowest) 
} 

