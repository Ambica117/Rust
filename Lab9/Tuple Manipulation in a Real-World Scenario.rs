fn main() { 
    let employee = (101, "John Doe", 45000); // Tuple (ID, Name, Salary) 
     
    let updated_employee = apply_salary_hike(employee); 
     
    println!("Updated Employee: {:?}", updated_employee); 
} 
 
fn apply_salary_hike(employee: (u32, &str, u32)) -> (u32, &str, u32) { 
    if employee.2 < 50000 { 
        let new_salary = (employee.2 as f32 * 1.1).round() as u32; // 10% hike 
        (employee.0, employee.1, new_salary) 
    } else { 
        employee 
    } 
} 

