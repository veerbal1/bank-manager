# Bank Manager

A simple banking system implemented in Rust that demonstrates core banking operations with proper error handling and transaction tracking.

## Features

- **Account Management**: Create bank accounts with unique IDs and initial balances
- **Deposits**: Add funds to existing accounts with transaction history
- **Withdrawals**: Remove funds from accounts with balance validation
- **Transfers**: Transfer money between accounts with proper validation
- **Balance Inquiry**: Check current account balance
- **Transaction History**: View complete transaction history for any account
- **Error Handling**: Comprehensive error handling for invalid operations

## Project Structure

```
bank-manager/
├── Cargo.toml          # Project configuration and dependencies
├── src/
│   └── main.rs         # Main application code
└── README.md           # This file
```

## Code Architecture

### Custom Types
- `TAccountID`: Type alias for account IDs (i32)
- `TBalanace`: Type alias for account balances (f64)

### Core Structures

#### Account
```rust
struct Account {
    id: TAccountID,
    balance: TBalanace,
    transaction_history: Vec<String>,
}
```

#### Bank
```rust
struct Bank {
    name: String,
    accounts: Vec<Account>,
}
```

### Available Operations

1. **Create Account**: `create_account(id, initial_balance)`
2. **Deposit**: `deposit(id, amount)` → Returns `Result<String, String>`
3. **Withdraw**: `withdraw(id, amount)` → Returns `Result<String, String>`
4. **Transfer**: `transfer(from_id, to_id, amount)` → Returns `Result<String, String>`
5. **Get Balance**: `get_balance(id)` → Returns `Result<TBalanace, String>`
6. **Get Transaction History**: `get_transaction_history(id)` → Returns `Result<&Vec<String>, String>`

## Usage

### Prerequisites
- Rust (latest stable version)

### Running the Project

1. Clone or navigate to the project directory:
   ```bash
   cd bank-manager
   ```

2. Run the application:
   ```bash
   cargo run
   ```

### Example Output
```
Bank has been created! Punjab National Bank
Account: 
    ID: 1
    Balance: 1000

Account: 
    ID: 2
    Balance: 500

Deposit successfully: New Balance: 1100
Withdraw successfully: New Balance: 450

Transfer successful: 200 transferred from account 1 to account 2
Error: Insufficient balance in source account
```

## Error Handling

The application includes comprehensive error handling for various scenarios:

- **Account Not Found**: When trying to operate on non-existent accounts
- **Insufficient Balance**: When withdrawal or transfer amount exceeds available balance
- **Negative Amounts**: When deposit, withdrawal, or transfer amounts are negative
- **Same Account Transfer**: When attempting to transfer to the same account

## Key Rust Concepts Demonstrated

- **Ownership and Borrowing**: Proper handling of mutable and immutable references
- **Result Type**: Using `Result<T, E>` for error handling instead of panics
- **Type Aliases**: Creating custom types for better code readability
- **Struct Implementation**: Organizing related functionality in `impl` blocks
- **Vector Operations**: Managing collections of accounts
- **String Formatting**: Creating formatted transaction messages
- **Pattern Matching**: Using `match` statements for error handling

## Development Notes

### Borrowing Challenges Solved
The transfer function initially faced Rust's borrowing checker limitations when trying to borrow `self.accounts` mutably multiple times. This was resolved by:

1. Finding account indices first using immutable borrows
2. Using indices to access accounts directly for mutations
3. Avoiding multiple simultaneous mutable borrows

### Future Enhancements
- User authentication system
- Persistent data storage
- Interest calculation
- Account types (savings, checking, business)
- Transaction limits and validation rules
- Command-line interface for interactive banking
- JSON serialization for data persistence

## License

This project is for educational purposes and learning Rust concepts.
