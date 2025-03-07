struct BankAccount {
    account_number: String,
    owner_name: String,
    balance: f64,
}

impl BankAccount {
    fn new(account_number: &str, owner_name: &str, balance: f64) -> BankAccount {
        BankAccount {
            account_number: account_number.to_string(),
            owner_name: owner_name.to_string(),
            balance,
        }
    }

    fn view_balance(&self) {
        println!("Account: {}, Owner: {}, Balance: ${:.2}", 
                 self.account_number, self.owner_name, self.balance);
    }

    fn deposit(&mut self, amount: f64) {
        if amount > 0.0 {
            self.balance += amount;
            println!("Deposited ${:.2} into {}'s account.", amount, self.owner_name);
        } else {
            println!("Invalid deposit amount.");
        }
    }

    fn withdraw(&mut self, amount: f64) {
        if amount > 0.0 && amount <= self.balance {
            self.balance -= amount;
            println!("Withdrawn ${:.2} from {}'s account.", amount, self.owner_name);
        } else {
            println!("Insufficient balance or invalid amount.");
        }
    }
}

fn main() {
    let mut account = BankAccount::new("123456789", "Alice", 1000.0);

    account.view_balance();
    account.view_balance();

    account.deposit(500.0);
    account.view_balance();

    account.withdraw(300.0);
    account.view_balance();
}

