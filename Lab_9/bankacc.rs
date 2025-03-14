struct BankAccount {
    account_number: u32,
    holder_name: String,
    balance: f64,
}

impl BankAccount {
    // Method to deposit money
    fn deposit(&mut self, amount: f64) {
        self.balance += amount;
        println!("Deposited ₹{:.2}. New Balance: ₹{:.2}", amount, self.balance);
    }

    // Method to withdraw money with balance check
    fn withdraw(&mut self, amount: f64) {
        if amount > self.balance {
            println!("Insufficient balance! Withdrawal failed.");
        } else {
            self.balance -= amount;
            println!("Withdrawn ₹{:.2}. New Balance: ₹{:.2}", amount, self.balance);
        }
    }

    // Method to display account details
    fn display_details(&self) {
        println!(
            "Account Number: {}\nHolder Name: {}\nBalance: ₹{:.2}",
            self.account_number, self.holder_name, self.balance
        );
    }
}

fn main() {
    let mut account = BankAccount {
        account_number: 123456,
        holder_name: String::from("Alice"),
        balance: 5000.0,
    };

    account.display_details();
    account.deposit(2000.0);
    account.withdraw(3000.0);
    account.withdraw(5000.0);
    account.display_details();
}

