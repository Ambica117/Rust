struct Student { 
    name: String, 
    age: u32, 
    grade: String, 
} 
 
impl Student { 
    fn update_grade(&mut self, new_grade: String) { 
        self.grade = new_grade; 
    } 
 
    fn display(&self) { 
        println!("Name: {}, Age: {}, Grade: {}", self.name, self.age, self.grade); 
    } 
} 
 
fn main() { 
    let mut student = Student { 
        name: String::from("Alice"), 
        age: 20, 
        grade: String::from("B"), 
    }; 
 
    student.display(); 
 
    student.update_grade(String::from("A")); 
    student.display(); 
} 
