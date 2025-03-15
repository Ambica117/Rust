#[derive(Debug)] // Enable pretty printing
enum FuelType {
    Petrol,
    Diesel,
    Electric,
}

#[derive(Debug)]
struct Vehicle {
    brand: String,
    model: String,
    fuel_type: FuelType,
}

fn filter_electric_vehicles(vehicles: &Vec<Vehicle>) -> Vec<&Vehicle> {
    vehicles.iter().filter(|v| matches!(v.fuel_type, FuelType::Electric)).collect()
}

fn main() {
    let vehicles = vec![
        Vehicle {
            brand: String::from("Tesla"),
            model: String::from("Model 3"),
            fuel_type: FuelType::Electric,
        },
        Vehicle {
            brand: String::from("Ford"),
            model: String::from("Mustang"),
            fuel_type: FuelType::Petrol,
        },
        Vehicle {
            brand: String::from("Chevrolet"),
            model: String::from("Bolt"),
            fuel_type: FuelType::Electric,
        },
        Vehicle {
            brand: String::from("Volkswagen"),
            model: String::from("Golf"),
            fuel_type: FuelType::Diesel,
        },
    ];

    let electric_vehicles = filter_electric_vehicles(&vehicles);

    println!("Electric Vehicles:");
    for vehicle in electric_vehicles {
        println!("Brand: {}, Model: {}", vehicle.brand, vehicle.model);
    }
}

