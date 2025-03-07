#[derive(Clone)]
struct Book {
    title: String,
    author: String,
    isbn: String,
    is_issued: bool,
}

impl Book {
    fn issue_book(&mut self) {
        self.is_issued = true;
    }

    fn new(title: &str, author: &str, isbn: &str) -> Book {
        Book {
            title: title.to_string(),
            author: author.to_string(),
            isbn: isbn.to_string(),
            is_issued: false,
        }
    }

    // Method to display the details of the book
    fn display(&self) {
        println!("Title: {}", self.title);
        println!("Author: {}", self.author);
        println!("ISBN: {}", self.isbn);
        println!("Is Issued: {}", self.is_issued);
    }
}

fn main() {
    let mut book1 = Book::new("Rust Programming", "Steve", "123456");

    // Display the book details before issuing
    println!("Before issuing:");
    book1.display();
    
    // Issue the book and update its status
    book1.issue_book();

    // Display the book details after issuing
    println!("\nAfter issuing:");
    book1.display();
}

