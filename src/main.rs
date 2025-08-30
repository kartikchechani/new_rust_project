#[derive(Debug)]
struct Account {
    id: u32,
    balance: i32,
    holder: String
}

impl Account {
    fn new(id: u32, holder: String) -> Self {
        Account {
            id,
            balance: 0,
            holder
        }
    }

    fn deposit(&mut self, amount: i32) {
        self.balance += amount;
    }

    fn withdraw(&mut self, amount:i32) {
        self.balance -= amount;
    }

    fn summary(&self) -> String {
        format!("Account {}: Holder: {}, Balance: {}", self.id, self.holder, self.balance)
    }
}



#[derive(Debug)]
struct Bank {
    accounts: Vec<Account>
}

impl Bank {
    fn new() -> Self {
        Bank { accounts : vec![] }
    }

    fn add_account(&mut self, account:Account) {
        self.accounts.push(account);
    }

    fn total_balance(&self) -> i32 {
        self.accounts.iter().map(|account| account.balance).sum()
    }

    fn account_summary(&self) -> Vec<String> {
        self.accounts
        .iter()
        .map(|account| account.summary())
        .collect::<Vec<String>>()
    }
}


fn main() {
    let mut bank = Bank::new();
    let mut account = Account::new(1, String::from("me"));
    let mut account2 = Account::new(2, String::from("new"));

    account.deposit(500);
    account.withdraw(200);

    let summary= account.summary();

    println!("{}", summary);

    bank.add_account(account);
    bank.add_account(account2);



    

    println!("{:#?}", bank);

    println!("Total Balance in Bank: {}", bank.total_balance());
    println!("Account Summaries: {:#?}", bank.account_summary());
}
