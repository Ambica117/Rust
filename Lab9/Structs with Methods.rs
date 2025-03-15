#[derive(Debug)] // Allow us to print the struct using println! 
struct BankAccount { 
    account_number: u32, 
    holder_name: String, 
    balance: f32, 
} 
 
impl BankAccount { 
    // Method to deposit money into the account 
    fn deposit(&mut self, amount: f32) { 
        self.balance += amount; 
        println!("Deposited ₹{:.2}. New balance: ₹{:.2}", amount, self.balance); 
    } 
 
    // Method to withdraw money (with balance check) 
    fn withdraw(&mut self, amount: f32) { 
        if amount <= self.balance { 
            self.balance -= amount; 
            println!("Withdrew ₹{:.2}. New balance: ₹{:.2}", amount, self.balance); 
        } else { 
            println!("Insufficient funds for withdrawal."); 
        } 
    } 
 
    // Method to display account details 
    fn display_account_details(&self) { 
        println!("Account Number: {}", self.account_number); 
        println!("Holder Name: {}", self.holder_name); 
        println!("Balance: ₹{:.2}", self.balance); 
    } 
} 
 
fn main() { 
    let mut account = BankAccount { 
        account_number: 12345678, 
        holder_name: String::from("John Doe"), 
        balance: 5000.0, 
    }; 
 
    account.display_account_details(); 
    account.deposit(1500.0); 
    account.withdraw(1000.0); 
    account.withdraw(6000.0); // This will fail 
} 

