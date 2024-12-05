// Define the Account trait with deposit and withdraw methods returning Result
trait Account {
    fn deposit(&mut self, amount: f64) -> Result<(), String>;
    fn withdraw(&mut self, amount: f64) -> Result<(), String>;
    fn balance(&self) -> f64;
}

// Implement the BankAccount struct
struct BankAccount {
    account_number: u32,
    holder_name: String,
    balance: f64,
}

impl BankAccount {
    fn new(account_number: u32, holder_name: String) -> Self {
        BankAccount {
            account_number,
            holder_name,
            balance: 0.0,
        }
    }
}

impl Account for BankAccount {
    fn deposit(&mut self, amount: f64) -> Result<(), String> {
        if amount <= 0.0 {
            return Err("Deposit amount must be positive.".to_string());
        }
        self.balance += amount;
        Ok(())
    }

    fn withdraw(&mut self, amount: f64) -> Result<(), String> {
        if amount <= 0.0 {
            return Err("Withdrawal amount must be positive.".to_string());
        }
        if amount > self.balance {
            return Err("Insufficient funds.".to_string());
        }
        self.balance -= amount;
        Ok(())
    }

    fn balance(&self) -> f64 {
        self.balance
    }
}

fn main() {
    // Create two BankAccount instances
    let mut account1 = BankAccount::new(1001, "Alice".to_string());
    let mut account2 = BankAccount::new(1002, "Bob".to_string());

    // Deposit into account1
    match account1.deposit(500.0) {
        Ok(_) => println!("Deposit successful. New balance: ${}", account1.balance()),
        Err(e) => println!("Deposit failed: {}", e),
    }

    // Withdraw from account2
    match account2.withdraw(200.0) {
        Ok(_) => println!("Withdrawal successful. New balance: ${}", account2.balance()),
        Err(e) => println!("Withdrawal failed: {}", e),
    }

    // Print balances
    println!(
        "{}'s balance: ${}",
        account1.holder_name,
        account1.balance()
    );
    println!(
        "{}'s balance: ${}",
        account2.holder_name,
        account2.balance()
    );
}
