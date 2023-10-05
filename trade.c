#include <stdio.h>
#include <stdbool.h>

// Global variables
double ethBalance = 100.0; // Initial ETH balance for the bot

// Function to deposit ETH
void deposit(double amount) {
    if (amount > 0) {
        ethBalance += amount;
        printf("Deposited %.2lf ETH. New balance: %.2lf ETH\n", amount, ethBalance);
    } else {
        printf("Invalid deposit amount.\n");
    }
}

// Function to withdraw ETH
void withdraw(double amount) {
    if (amount > 0 && amount <= ethBalance) {
        ethBalance -= amount;
        printf("Withdrawn %.2lf ETH. New balance: %.2lf ETH\n", amount, ethBalance);
    } else {
        printf("Invalid withdrawal amount.\n");
    }
}

// Function to simulate a trade
void simulateTrade(bool isBuy, double amount) {
    if (amount > 0) {
        if (isBuy) {
            if (ethBalance >= amount) {
                ethBalance -= amount;
                printf("Bought %.2lf ETH. New balance: %.2lf ETH\n", amount, ethBalance);
            } else {
                printf("Insufficient ETH balance for buying.\n");
            }
        } else {
            ethBalance += amount;
            printf("Sold %.2lf ETH. New balance: %.2lf ETH\n", amount, ethBalance);
        }
    } else {
        printf("Invalid trade amount.\n");
    }
}

// Function to get the current balance
double getBalance() {
    return ethBalance;
}

int main() {
    int choice;
    double amount;

    printf("Pseudo Trading Bot\n");

    while (1) {
        printf("\n1. Deposit ETH\n");
        printf("2. Withdraw ETH\n");
        printf("3. Simulate Trade\n");
        printf("4. Get Balance\n");
        printf("5. Quit\n");
        printf("Enter your choice: ");
        scanf("%d", &choice);

        switch (choice) {
            case 1:
                printf("Enter the amount to deposit: ");
                scanf("%lf", &amount);
                deposit(amount);
                break;
            case 2:
                printf("Enter the amount to withdraw: ");
                scanf("%lf", &amount);
                withdraw(amount);
                break;
            case 3:
                printf("Buy (1) or Sell (0)? ");
                scanf("%d", &choice);
                printf("Enter the trade amount: ");
                scanf("%lf", &amount);
                simulateTrade(choice == 1, amount);
                break;
            case 4:
                printf("Current balance: %.2lf ETH\n", getBalance());
                break;
            case 5:
                printf("Exiting the program.\n");
                return 0;
            default:
                printf("Invalid choice. Please try again.\n");
        }
    }
}
