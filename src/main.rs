type TAccountID = i32;
type TBalanace = f64;

#[derive(Debug)]
struct Account {
    id: TAccountID,
    balance: TBalanace,
    transaction_history: Vec<String>,
}

struct Bank {
    name: String,
    accounts: Vec<Account>,
}

impl Bank {
    fn new(bank_name: String) -> Self {
        Bank {
            name: bank_name,
            accounts: Vec::new(),
        }
    }

    fn create_account(&mut self, id: TAccountID, initial_balance: TBalanace) {
        let account = Account {
            id,
            balance: initial_balance,
            transaction_history: Vec::new(),
        };

        self.accounts.push(account);
    }

    fn deposit(&mut self, id: TAccountID, amount: TBalanace) -> Result<String, String> {
        if amount < 0.0 {
            return Err(String::from("Amount must be Positive"));
        }

        for account in self.accounts.iter_mut() {
            if account.id == id {
                account.balance += amount;
                account
                    .transaction_history
                    .push(String::from(format!("Deposited {}", amount)));
                return Ok(format!(
                    "Deposit successfully: New Balance: {}",
                    account.balance
                ));
            }
        }
        Err(String::from("Account not found"))
    }

    fn withdraw(&mut self, id: TAccountID, amount: TBalanace) -> Result<String, String> {
        if amount < 0.0 {
            return Err(String::from("Amount must be positive!"));
        }

        for account in self.accounts.iter_mut() {
            if account.id == id {
                if account.balance < amount {
                    return Err(String::from("Insufficent Balance"));
                } else {
                    account.balance -= amount;
                    account
                        .transaction_history
                        .push(format!("Withdrew {}", amount));
                    return Ok(format!(
                        "Withdraw successfully: New Balance: {}",
                        account.balance
                    ));
                }
            }
        }
        Err(String::from("Account not found!"))
    }

    fn get_balance(&self, id: TAccountID) -> Result<TBalanace, String> {
        for account in self.accounts.iter() {
            if account.id == id {
                return Ok(account.balance);
            }
        }
        Err(String::from("Account not found"))
    }

    fn get_transaction_history(&self, id: TAccountID) -> Result<&Vec<String>, String> {
        for account in self.accounts.iter() {
            if account.id == id {
                return Ok(&account.transaction_history);
            }
        }
        Err(String::from("Account not Found"))
    }
}

fn main() {
    let mut new_bank = Bank::new(String::from("Punjab National Bank"));
    println!("Bank has been created! {}", new_bank.name);
    new_bank.create_account(1, 1000.0);
    new_bank.create_account(2, 500.0);

    for (_, account) in new_bank.accounts.iter().enumerate() {
        println!(
            "Account: \n
                ID: {}\n
                Balance: {}",
            account.id, account.balance
        )
    }

    // Test deposit:
    match new_bank.deposit(1, 100.0) {
        Ok(val) => println!("{}", val),
        Err(e) => println!("{}", e),
    }
    match new_bank.deposit(1, 149.0) {
        Ok(val) => println!("{}", val),
        Err(e) => println!("{}", e),
    }
    match new_bank.deposit(1, 159.0) {
        Ok(val) => println!("{}", val),
        Err(e) => println!("{}", e),
    }

    match new_bank.withdraw(2, 50.0) {
        Ok(val) => println!("{}", val),
        Err(e) => println!("{}", e),
    }

    println!("After Transactions");
    for (_, account) in new_bank.accounts.iter().enumerate() {
        println!(
            "Account: \n
                ID: {}\n
                Balance: {}",
            account.id, account.balance
        )
    }

    println!("Test balance");

    let balance = new_bank.get_balance(1);
    match balance {
        Ok(val) => println!("Balance is: {}", val),
        Err(e) => println!("Error: {}", e),
    }

    println!("Get Transaction History");
    let transaction_history = new_bank.get_transaction_history(1);
    match transaction_history {
        Ok(val) => {
            for transaction in val.iter() {
                println!("{}", transaction)
            }
        },
        Err(e) => println!("Error getting transaction history: {}", e),
    }
}
