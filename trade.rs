use ink_lang as ink;

#[ink::contract]
mod pseudo_trading_bot {
    #[ink(storage)]
    pub struct PseudoTradingBot {
        owner: AccountId,
        eth_balance: Balance,
    }

    impl PseudoTradingBot {
        #[ink(constructor)]
        pub fn new(initial_balance: Balance) -> Self {
            Self {
                owner: Self::env().caller(),
                eth_balance: initial_balance,
            }
        }

        #[ink(message)]
        pub fn deposit(&mut self, amount: Balance) {
            assert!(amount > 0, "Deposit amount must be greater than zero.");
            self.eth_balance += amount;
        }

        #[ink(message)]
        pub fn withdraw(&mut self, amount: Balance) {
            assert!(amount > 0 && amount <= self.eth_balance, "Invalid withdrawal amount.");
            self.eth_balance -= amount;
        }

        #[ink(message)]
        pub fn simulate_trade(&mut self, is_buy: bool, amount: Balance) {
            assert!(amount > 0, "Trade amount must be greater than zero.");
            if is_buy {
                assert!(self.eth_balance >= amount, "Insufficient ETH balance for buying.");
                self.eth_balance -= amount;
            } else {
                self.eth_balance += amount;
            }
        }

        #[ink(message)]
        pub fn get_balance(&self) -> Balance {
            self.eth_balance
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[ink::test]
        fn it_works() {
            let mut bot = PseudoTradingBot::new(100);
            assert_eq!(bot.get_balance(), 100);
            bot.deposit(50);
            assert_eq!(bot.get_balance(), 150);
            bot.simulate_trade(true, 25);
            assert_eq!(bot.get_balance(), 125);
            bot.withdraw(75);
            assert_eq!(bot.get_balance(), 50);
        }
    }
}
