use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};
use crate::error::{AppError, Result};


/// Struct representing an account
#[derive(Debug, Clone, PartialEq)]
pub struct Account {
    // TODO 3.1: Define fields for Account
    pub name: String,
    pub balance: u64,
    pub created_at: u64,  // Timestamp (seconds)
    pub transaction_count: u32,
}

impl Account {
    /// Create a new account
    pub fn new(name: String, balance: u64) -> Self {
        // TODO 3.2: Implement constructor
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        Self {
            name,
            balance,
            created_at: timestamp,
            transaction_count: 0,
        }
    }

    /// Add to balance
    pub fn add_balance(&mut self, amount: u64) -> Result<()> {
        // TODO 3.3: Add amount to balance with overflow check
        self.balance = self.balance
            .checked_add(amount)
            .ok_or_else(|| AppError::InvalidAmount(amount))?;
        
        self.transaction_count += 1;
        Ok(())
    }

    /// Subtract from balance
    pub fn subtract_balance(&mut self, amount: u64) -> Result<()> {
        // TODO 3.4: Subtract amount from balance
        if self.balance < amount {
            return Err(AppError::InsufficientBalance {
                available: self.balance,
                required: amount,
            });
        }

        self.balance -= amount;
        self.transaction_count += 1;
        Ok(())
    }

    /// Get current balance
    pub fn get_balance(&self) -> u64 {
        // TODO 3.5: Return balance
        self.balance
    }

    /// Check if account has sufficient balance
    pub fn has_sufficient_balance(&self, amount: u64) -> bool {
        // TODO 3.6: Check balance >= amount
        self.balance >= amount
    }

    /// Get account information as string
    pub fn info(&self) -> String {
        // TODO 3.7: Format account info
        format!(
            "Account(name: {}, balance: {}, transactions: {})",
            self.name, self.balance, self.transaction_count
        )
    }
}

/// Struct managing the entire application state
#[derive(Debug, Clone)]
pub struct AppState {
    // TODO 3.8: Define fields for AppState
    accounts: HashMap<String, Account>,
    total_transactions: u64,
}

impl AppState {
    /// Create new AppState (empty)
    pub fn new() -> Self {
        // TODO 3.9: Implement constructor
        Self {
            accounts: HashMap::new(),
            total_transactions: 0,
        }
    }

    /// Add a new account
    pub fn create_account(&mut self, name: String, balance: u64) -> Result<()> {
        // TODO 3.10: Implement create_account logic
        AppError::validate_account_name(&name)?;
        AppError::validate_amount(balance)?;

        if self.accounts.contains_key(&name) {
            return Err(AppError::AccountAlreadyExists(name));
        }

        let account = Account::new(name.clone(), balance);
        self.accounts.insert(name, account);
        self.total_transactions += 1;
        
        Ok(())
    }

    /// Get reference to an account (immutable)
    pub fn get_account(&self, name: &str) -> Result<&Account> {
        // TODO 3.11: Implement get_account
        self.accounts
            .get(name)
            .ok_or_else(|| AppError::AccountNotFound(name.to_string()))
    }

    /// Get mutable reference to an account
    pub fn get_account_mut(&mut self, name: &str) -> Result<&mut Account> {
        // TODO 3.12: Implement get_account_mut
        self.accounts
            .get_mut(name)
            .ok_or_else(|| AppError::AccountNotFound(name.to_string()))
    }

    /// Delete an account
    pub fn delete_account(&mut self, name: &str) -> Result<Account> {
        // TODO 3.13: Implement delete_account
        let account = self.accounts
            .remove(name)
            .ok_or_else(|| AppError::AccountNotFound(name.to_string()))?;
        
        self.total_transactions += 1;
        Ok(account)
    }

    /// Check if account exists
    pub fn account_exists(&self, name: &str) -> bool {
        // TODO 3.14: Use contains_key
        self.accounts.contains_key(name)
    }

    /// Transfer money between 2 accounts
    pub fn transfer(&mut self, from: &str, to: &str, amount: u64) -> Result<()> {
        // TODO 3.15: Implement transfer with scope-based borrowing
        if from == to {
            return Err(AppError::InvalidAmount(amount));
        }
        AppError::validate_amount(amount)?;

        // Borrow 1: Subtract from source
        {
            let from_account = self.get_account_mut(from)?;
            from_account.subtract_balance(amount)?;
        } 

        // Borrow 2: Add to destination
        {
            let to_account = self.get_account_mut(to)?;
            to_account.add_balance(amount)?;
        }

        self.total_transactions += 1;
        Ok(())
    }

    /// Get total balance across all accounts
    pub fn total_balance(&self) -> u64 {
        // TODO 3.16: Iterate and sum
        self.accounts.values().map(|acc| acc.balance).sum()
    }

    /// Get number of accounts
    pub fn account_count(&self) -> usize {
        // TODO 3.17: Return length
        self.accounts.len()
    }

    /// Get list of all account names (sorted)
    pub fn list_accounts(&self) -> Vec<String> {
        // TODO 3.18: Collect keys and sort
        let mut names: Vec<String> = self.accounts.keys().cloned().collect();
        names.sort();
        names
    }

    /// Display state to console
    pub fn display(&self) {
        // TODO 3.19: Print state
        println!("--- System State ---");
        println!("Accounts: {}", self.account_count());
        println!("Total Balance: {}", self.total_balance());
        println!("Total Transactions: {}", self.total_transactions);
        println!("Account List: {:?}", self.list_accounts());
        println!("--------------------");
    }
}

// TODO 3.20: Implement Default trait
impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_account() {
        // TODO 3.21: Test creating account
        let name = String::from("Alice");
        let account = Account::new(name.clone(), 1000);
        assert_eq!(account.name, name);
        assert_eq!(account.balance, 1000);
        assert_eq!(account.transaction_count, 0);
    }

    #[test]
    fn test_add_balance() {
        // TODO 3.22: Test adding balance
        let mut account = Account::new("Bob".into(), 500);
        account.add_balance(200).unwrap();
        assert_eq!(account.get_balance(), 700);
        assert_eq!(account.transaction_count, 1);
    }

    #[test]
    fn test_subtract_balance() {
        // TODO 3.23: Test subtracting balance
        let mut account = Account::new("Charlie".into(), 1000);
        
        // Success case
        assert!(account.subtract_balance(400).is_ok());
        assert_eq!(account.get_balance(), 600);

        // Failure case (insufficient)
        assert!(account.subtract_balance(1000).is_err());
    }

    #[test]
    fn test_app_state_create_account() {
        // TODO 3.24: Test AppState account creation
        let mut state = AppState::new();
        state.create_account("Alice".into(), 1000).unwrap();
        assert!(state.account_exists("Alice"));
        assert!(state.create_account("Alice".into(), 500).is_err());
    }

    #[test]
    fn test_app_state_transfer() {
        // TODO 3.25: Test transfer
        let mut state = AppState::new();
        state.create_account("Alice".into(), 1000).unwrap();
        state.create_account("Bob".into(), 500).unwrap();

        // Valid transfer
        state.transfer("Alice", "Bob", 300).unwrap();
        assert_eq!(state.get_account("Alice").unwrap().balance, 700);
        assert_eq!(state.get_account("Bob").unwrap().balance, 800);

        // Edge case: Self transfer
        assert!(state.transfer("Alice", "Alice", 100).is_err());
    }

    #[test]
    fn test_total_balance() {
        // TODO 3.26: Test total balance
        let mut state = AppState::new();
        state.create_account("A".into(), 100).unwrap();
        state.create_account("B".into(), 200).unwrap();
        assert_eq!(state.total_balance(), 300);
    }
}