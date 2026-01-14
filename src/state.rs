// ============================================
// EXERCISE 3: STATE MANAGEMENT AND DATA STRUCTURES
// ============================================
// Objectives: Learn about Structs, HashMap, Ownership, Borrowing

use std::collections::HashMap;
use crate::error::{AppError, Result};

/// Struct representing an account
#[derive(Debug, Clone, PartialEq)]
pub struct Account {
    // TODO 3.1: Define fields for Account
    // Uncomment and complete:
    // pub name: String,
    // pub balance: u64,
    // pub created_at: u64,  // Timestamp (seconds)
    // pub transaction_count: u32,
}

impl Account {
    /// Create a new account
    pub fn new(name: String, balance: u64) -> Self {
        // TODO 3.2: Implement constructor
        // Hint: Use current timestamp for created_at
        // You can use: std::time::SystemTime::now()
        //              .duration_since(std::time::UNIX_EPOCH)
        //              .unwrap()
        //              .as_secs()
        todo!("Implement Account::new")
    }

    /// Add to balance
    pub fn add_balance(&mut self, amount: u64) -> Result<()> {
        // TODO 3.3: Add amount to balance
        // Check for overflow (balance + amount must not exceed u64::MAX)
        // Increment transaction_count
        // Return Ok(()) if successful
        // Return Err(AppError::InvalidAmount(...)) if overflow
        todo!("Implement add_balance")
    }

    /// Subtract from balance
    pub fn subtract_balance(&mut self, amount: u64) -> Result<()> {
        // TODO 3.4: Subtract amount from balance
        // Check balance >= amount
        // Increment transaction_count
        // Return Err(AppError::InsufficientBalance {...}) if not enough
        todo!("Implement subtract_balance")
    }

    /// Get current balance
    pub fn get_balance(&self) -> u64 {
        // TODO 3.5: Return balance (simple!)
        todo!("Implement get_balance")
    }

    /// Check if account has sufficient balance
    pub fn has_sufficient_balance(&self, amount: u64) -> bool {
        // TODO 3.6: Check balance >= amount
        todo!("Implement has_sufficient_balance")
    }

    /// Get account information as string
    pub fn info(&self) -> String {
        // TODO 3.7: Format account info as nice string
        // Example: "Account(name: Alice, balance: 1000, transactions: 5)"
        todo!("Implement info")
    }
}

/// Struct managing the entire application state
#[derive(Debug)]
pub struct AppState {
    // TODO 3.8: Define fields for AppState
    // Uncomment and complete:
    // accounts: HashMap<String, Account>,  // Key is account name
    // total_transactions: u64,
}

impl AppState {
    /// Create new AppState (empty)
    pub fn new() -> Self {
        // TODO 3.9: Implement constructor
        // Initialize empty HashMap and total_transactions = 0
        todo!("Implement AppState::new")
    }

    /// Add a new account
    pub fn create_account(&mut self, name: String, balance: u64) -> Result<()> {
        // TODO 3.10: Implement create_account
        // 1. Validate account name (use AppError::validate_account_name)
        // 2. Validate amount (use AppError::validate_amount)
        // 3. Check if account already exists (use self.accounts.contains_key)
        // 4. If doesn't exist, create new and insert into HashMap
        // 5. Increment total_transactions
        // Return Err(AppError::AccountAlreadyExists(...)) if already exists
        todo!("Implement create_account")
    }

    /// Get reference to an account (immutable)
    pub fn get_account(&self, name: &str) -> Result<&Account> {
        // TODO 3.11: Implement get_account
        // Use self.accounts.get(name)
        // Return Err(AppError::AccountNotFound(...)) if not found
        // Hint: .ok_or_else(|| AppError::AccountNotFound(...))
        todo!("Implement get_account")
    }

    /// Get mutable reference to an account
    pub fn get_account_mut(&mut self, name: &str) -> Result<&mut Account> {
        // TODO 3.12: Implement get_account_mut (similar to get_account but mutable)
        todo!("Implement get_account_mut")
    }

    /// Delete an account
    pub fn delete_account(&mut self, name: &str) -> Result<Account> {
        // TODO 3.13: Implement delete_account
        // Use self.accounts.remove(name)
        // Return deleted account or Err if doesn't exist
        // Increment total_transactions
        todo!("Implement delete_account")
    }

    /// Check if account exists
    pub fn account_exists(&self, name: &str) -> bool {
        // TODO 3.14: Use contains_key
        todo!("Implement account_exists")
    }

    /// Transfer money between 2 accounts
    pub fn transfer(&mut self, from: &str, to: &str, amount: u64) -> Result<()> {
        // TODO 3.15: Implement transfer (HARD!)
        // This is an exercise about borrowing and ownership
        //
        // PROBLEM: Cannot borrow 2 mutable references at the same time!
        // SOLUTION:
        // 1. Check from != to first
        // 2. Get balance from 'from' first, check if sufficient
        // 3. Subtract money from 'from'
        // 4. Add money to 'to'
        // 5. Increment total_transactions
        //
        // Hint structure:
        // if from == to { return Err(...) }
        // AppError::validate_amount(amount)?;
        // {
        //     let from_account = self.get_account_mut(from)?;
        //     from_account.subtract_balance(amount)?;
        // } // from_account borrow ends here
        // {
        //     let to_account = self.get_account_mut(to)?;
        //     to_account.add_balance(amount)?;
        // }
        // self.total_transactions += 1;
        // Ok(())
        todo!("Implement transfer")
    }

    /// Get total balance across all accounts
    pub fn total_balance(&self) -> u64 {
        // TODO 3.16: Iterate through all accounts and sum balance
        // Hint: self.accounts.values().map(...).sum()
        todo!("Implement total_balance")
    }

    /// Get number of accounts
    pub fn account_count(&self) -> usize {
        // TODO 3.17: Return length of HashMap
        todo!("Implement account_count")
    }

    /// Get list of all account names (sorted)
    pub fn list_accounts(&self) -> Vec<String> {
        // TODO 3.18: Get all keys, sort and return
        // Hint:
        // let mut names: Vec<String> = self.accounts.keys().cloned().collect();
        // names.sort();
        // names
        todo!("Implement list_accounts")
    }

    /// Display state to console
    pub fn display(&self) {
        // TODO 3.19: Print state information
        // Print number of accounts, total balance, list of accounts
        todo!("Implement display")
    }
}

// TODO 3.20: Implement Default trait for AppState
// To be able to use AppState::default()
// impl Default for AppState {
//     fn default() -> Self {
//         Self::new()
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_account() {
        // TODO 3.21: Test creating account
        let account = Account::new(String::from("Alice"), 1000);
        // Assert the fields
        todo!("Write test for create account")
    }

    #[test]
    fn test_add_balance() {
        // TODO 3.22: Test adding balance
        todo!("Write test for add_balance")
    }

    #[test]
    fn test_subtract_balance() {
        // TODO 3.23: Test subtracting balance
        // Test both sufficient and insufficient cases
        todo!("Write test for subtract_balance")
    }

    #[test]
    fn test_app_state_create_account() {
        // TODO 3.24: Test AppState create account
        todo!("Write test for AppState::create_account")
    }

    #[test]
    fn test_app_state_transfer() {
        // TODO 3.25: Test transfer between 2 accounts
        // Test edge cases:
        // - Transfer to self
        // - Insufficient balance
        // - Account not found
        todo!("Write test for transfer")
    }

    #[test]
    fn test_total_balance() {
        // TODO 3.26: Test calculating total balance
        todo!("Write test for total_balance")
    }
}
