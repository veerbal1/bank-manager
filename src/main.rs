#[derive(Debug)]
struct Account {
    id: i32,
    balance: f64,
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

    fn create_account(&mut self, id: i32, initial_balance: f64) {
        let account = Account {
            id,
            balance: initial_balance,
        };

        self.accounts.push(account);
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
}
