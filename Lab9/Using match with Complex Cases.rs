use std::io; 
 
fn main() { 
    let mut item_input = String::new(); 
    let mut quantity_input = String::new(); 
 
    // Menu items and their prices 
    println!("Enter a menu item (Burger, Pizza, Pasta):"); 
    io::stdin().read_line(&mut item_input).expect("Failed to read line"); 
    let item = item_input.trim(); 
 
    println!("Enter quantity:"); 
    io::stdin().read_line(&mut quantity_input).expect("Failed to read line"); 
    let quantity: u32 = quantity_input.trim().parse().expect("Please enter a valid number"); 
 
    let price = match item { 
        "Burger" => 150, 
        "Pizza" => 300, 
        "Pasta" => 200, 
        _ => { 
            println!("Item not found."); 
            return; 
        } 
    }; 
 
    let mut total_price = price * quantity; 
     
    // Apply discount based on quantity 
    if quantity >= 3 { 
        total_price -= total_price / 10; // 10% discount 
    } 
 
    println!("Total price for {} {}(s) is: ₹{}", quantity, item, total_price); 
} 

