type TAccountID = i32;
type TBalanace = f64;

#[derive(Debug)]
struct Account {
    id: TAccountID,
    balance: TBalanace,
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
                    return Ok(format!(
                        "Withdraw successfully: New Balance: {}",
                        account.balance
                    ));
                }
            }
        }
        Err(String::from("Account not found!"))
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
}
