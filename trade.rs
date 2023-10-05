use std::io;

// Struct to represent the trading bot
struct TradingBot {
    eth_balance: f64,
}

impl TradingBot {
    fn new(initial_balance: f64) -> TradingBot {
        TradingBot {
            eth_balance: initial_balance,
        }
    }

    fn deposit(&mut self, amount: f64) {
        if amount > 0.0 {
            self.eth_balance += amount;
            println!("Deposited {:.2} ETH. New balance: {:.2} ETH", amount, self.eth_balance);
        } else {
            println!("Invalid deposit amount.");
        }
    }

    fn withdraw(&mut self, amount: f64) {
        if amount > 0.0 && amount <= self.eth_balance {
            self.eth_balance -= amount;
            println!("Withdrawn {:.2} ETH. New balance: {:.2} ETH", amount, self.eth_balance);
        } else {
            println!("Invalid withdrawal amount.");
        }
    }

    fn simulate_trade(&mut self, is_buy: bool, amount: f64) {
        if amount > 0.0 {
            if is_buy {
                if self.eth_balance >= amount {
                    self.eth_balance -= amount;
                    println!("Bought {:.2} ETH. New balance: {:.2} ETH", amount, self.eth_balance);
                } else {
                    println!("Insufficient ETH balance for buying.");
                }
            } else {
                self.eth_balance += amount;
                println!("Sold {:.2} ETH. New balance: {:.2} ETH", amount, self.eth_balance);
            }
        } else {
            println!("Invalid trade amount.");
        }
    }

    fn get_balance(&self) -> f64 {
        self.eth_balance
    }
}

fn main() {
    let mut bot = TradingBot::new(100.0); // Initial ETH balance

    loop {
        println!("\nTrading Bot Menu:");
        println!("1. Deposit ETH");
        println!("2. Withdraw ETH");
        println!("3. Simulate Trade");
        println!("4. Get Balance");
        println!("5. Quit");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read input");
        let choice: u32 = choice.trim().parse().expect("Invalid input");

        match choice {
            1 => {
                println!("Enter the amount to deposit: ");
                let mut input = String::new();
                io::stdin().read_line(&mut input).expect("Failed to read input");
                let amount: f64 = input.trim().parse().expect("Invalid input");
                bot.deposit(amount);
            }
            2 => {
                println!("Enter the amount to withdraw: ");
                let mut input = String::new();
                io::stdin().read_line(&mut input).expect("Failed to read input");
                let amount: f64 = input.trim().parse().expect("Invalid input");
                bot.withdraw(amount);
            }
            3 => {
                println!("Buy (1) or Sell (0)? ");
                let mut input = String::new();
                io::stdin().read_line(&mut input).expect("Failed to read input");
                let is_buy: bool = input.trim().parse().expect("Invalid input");
                println!("Enter the trade amount: ");
                let mut input = String::new();
                io::stdin().read_line(&mut input).expect("Failed to read input");
                let amount: f64 = input.trim().parse().expect("Invalid input");
                bot.simulate_trade(is_buy, amount);
            }
            4 => {
                let balance = bot.get_balance();
                println!("Current balance: {:.2} ETH", balance);
            }
            5 => {
                println!("Exiting the program.");
                break;
            }
            _ => {
                println!("Invalid choice. Please try again.");
            }
        }
    }
}
