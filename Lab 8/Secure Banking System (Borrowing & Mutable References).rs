struct BankAccount {
    account_number: u32,
    owner_name: String,
    balance: f32,
}

impl BankAccount {
    // Method to view the balance
    fn view_balance(&self) -> f32 {
        self.balance
    }

    // Method to deposit funds
    fn deposit(&mut self, amount: f32) {
        self.balance += amount;
    }

    // Method to withdraw funds
    fn withdraw(&mut self, amount: f32) {
        if self.balance >= amount {
            self.balance -= amount;
        } else {
            println!("Insufficient funds!");
        }
    }

    // Method to display account information
    fn display_account_info(&self) {
        println!("Account Number: {}", self.account_number);
        println!("Owner: {}", self.owner_name);
        println!("Balance: ${}", self.balance);
    }
}

fn main() {
    let mut account = BankAccount {
        account_number: 101,
        owner_name: String::from("John Doe"),
        balance: 1000.0,
    };

    // Display the account information before transactions
    account.display_account_info();

    // View balance
    println!("Balance: ${}", account.view_balance());

    // Perform deposit and withdrawal
    account.deposit(500.0);
    account.withdraw(200.0);

    // Display the account information after transactions
    println!("\nAfter transactions:");
    account.display_account_info();
}

