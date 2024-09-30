use std::io::{self, Write};

struct Account {
    account_number: String,
    account_type: String,
    balance: f64,
    is_loyal: bool,
}

impl PartialEq for Account {
    fn eq(&self, other: &Self) -> bool {
        self.account_number == other.account_number
    }
}

fn main() {
    let mut accounts: Vec<Account> = Vec::new();

    loop {
        print_menu();
        let choice = get_user_input("Enter your choice: ");

        match choice.as_str() {
            "1" => open_account(&mut accounts),
            "2" => close_account(&mut accounts),
            "3" => deposit(&mut accounts),
            "4" => withdraw(&mut accounts),
            "5" => print_all_accounts(&accounts),
            "6" => break,
            _ => println!("Invalid choice. Please try again."),
        }
    }

    println!("Thank you for using RU Bank CLI!");
}

fn print_menu() {
    println!("\nRU Bank CLI");
    println!("1. Open an account");
    println!("2. Close an account");
    println!("3. Deposit");
    println!("4. Withdraw");
    println!("5. Print all accounts");
    println!("6. Exit");
}

fn get_user_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn open_account(accounts: &mut Vec<Account>) {
    println!("Opening a new account");
    let account_type = get_user_input("Enter account type (Checking/Savings/MoneyBag): ");
    let initial_balance: f64 = get_user_input("Enter initial balance: ")
        .parse()
        .unwrap_or(0.0);

    let is_loyal = account_type == "MoneyBag" && initial_balance >= 2000.0;
    let new_account = Account {
        account_number: format!("ACC{}", accounts.len() + 1),
        account_type,
        balance: initial_balance,
        is_loyal,
    };

    accounts.push(new_account);
    println!("Account created successfully!");
}

fn close_account(accounts: &mut Vec<Account>) {
    println!("Closing an account");
    let account_number = get_user_input("Enter account number: ");
    if let Some(index) = accounts.iter().position(|a| a.account_number == account_number) {
        accounts.remove(index);
        println!("Account closed successfully!");
    } else {
        println!("Account not found!");
    }
}

fn deposit(accounts: &mut Vec<Account>) {
    println!("Deposit");
    let account_number = get_user_input("Enter account number: ");
    if let Some(account) = accounts.iter_mut().find(|a| a.account_number == account_number) {
        let amount: f64 = get_user_input("Enter deposit amount: ")
            .parse()
            .unwrap_or(0.0);
        account.balance += amount;
        if account.account_type == "MoneyBag" && account.balance >= 2000.0 {
            account.is_loyal = true;
        }
        println!("Deposit successful! New balance: ${:.2}", account.balance);
    } else {
        println!("Account not found!");
    }
}

fn withdraw(accounts: &mut Vec<Account>) {
    println!("Withdraw");
    let account_number = get_user_input("Enter account number: ");
    if let Some(account) = accounts.iter_mut().find(|a| a.account_number == account_number) {
        let amount: f64 = get_user_input("Enter withdrawal amount: ")
            .parse()
            .unwrap_or(0.0);
        
        if account.balance >= amount {
            account.balance -= amount;
            
            // Add logic for Money Bag account withdrawal fee here
            
            println!("Withdrawal successful! New balance: ${:.2}", account.balance);
        } else {
            println!("Insufficient funds!");
        }
    } else {
        println!("Account not found!");
    }
}

/* 
fn interest(accounts: &mut Vec<Account>) {
    // TODO: Implement helper function for interest, and time travel features
}

*/
fn print_all_accounts(accounts: &Vec<Account>) {
    for account in accounts {
        println!("Account Number: {}", account.account_number);
        println!("Type: {}", account.account_type);
        println!("Balance: ${:.2}", account.balance);
        println!("Loyal Customer: {}", if account.is_loyal { "Yes" } else { "No" });
        println!("--------------------");
    }
}