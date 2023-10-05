#include <iostream>
#include <iomanip>

class TradingBot {
public:
    TradingBot(double initialBalance) : ethBalance(initialBalance) {}

    void deposit(double amount) {
        if (amount > 0) {
            ethBalance += amount;
            std::cout << "Deposited " << amount << " ETH. New balance: " << ethBalance << " ETH" << std::endl;
        } else {
            std::cerr << "Invalid deposit amount." << std::endl;
        }
    }

    void withdraw(double amount) {
        if (amount > 0 && amount <= ethBalance) {
            ethBalance -= amount;
            std::cout << "Withdrawn " << amount << " ETH. New balance: " << ethBalance << " ETH" << std::endl;
        } else {
            std::cerr << "Invalid withdrawal amount." << std::endl;
        }
    }

    void simulateTrade(bool isBuy, double amount) {
        if (amount > 0) {
            if (isBuy) {
                if (ethBalance >= amount) {
                    ethBalance -= amount;
                    std::cout << "Bought " << amount << " ETH. New balance: " << ethBalance << " ETH" << std::endl;
                } else {
                    std::cerr << "Insufficient ETH balance for buying." << std::endl;
                }
            } else {
                ethBalance += amount;
                std::cout << "Sold " << amount << " ETH. New balance: " << ethBalance << " ETH" << std::endl;
            }
        } else {
            std::cerr << "Invalid trade amount." << std::endl;
        }
    }

    double getBalance() const {
        return ethBalance;
    }

private:
    double ethBalance;
};

int main() {
    TradingBot bot(100.0); // Initial ETH balance

    while (true) {
        std::cout << "\nTrading Bot Menu:" << std::endl;
        std::cout << "1. Deposit ETH" << std::endl;
        std::cout << "2. Withdraw ETH" << std::endl;
        std::cout << "3. Simulate Trade" << std::endl;
        std::cout << "4. Get Balance" << std::endl;
        std::cout << "5. Quit" << std::endl;

        int choice;
        std::cout << "Enter your choice: ";
        std::cin >> choice;

        switch (choice) {
            case 1:
                double depositAmount;
                std::cout << "Enter the amount to deposit: ";
                std::cin >> depositAmount;
                bot.deposit(depositAmount);
                break;
            case 2:
                double withdrawalAmount;
                std::cout << "Enter the amount to withdraw: ";
                std::cin >> withdrawalAmount;
                bot.withdraw(withdrawalAmount);
                break;
            case 3:
                bool isBuy;
                double tradeAmount;
                std::cout << "Buy (1) or Sell (0)? ";
                std::cin >> isBuy;
                std::cout << "Enter the trade amount: ";
                std::cin >> tradeAmount;
                bot.simulateTrade(isBuy, tradeAmount);
                break;
            case 4:
                std::cout << "Current balance: " << std::fixed << std::setprecision(2) << bot.getBalance() << " ETH" << std::endl;
                break;
            case 5:
                std::cout << "Exiting the program." << std::endl;
                return 0;
            default:
                std::cerr << "Invalid choice. Please try again." << std::endl;
        }
    }

    return 0;
}
