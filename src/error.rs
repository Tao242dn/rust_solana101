// ============================================
// EXERCISE 2: CUSTOM ERROR TYPES
// ============================================
// Objectives: Learn about Error Handling, Result<T, E>, and Custom Error Types

use std::error;
use std::fmt;

/// Custom error type for the application
/// In Rust, error handling is done through Result<T, E>
#[derive(Debug, Clone, PartialEq)]
pub enum AppError {
    // TODO 2.1: Define error variants
    AccountNotFound(String),      // Account doesn't exist (contains account name)
    AccountAlreadyExists(String), // Account already exists
    InsufficientBalance {
        // Balance is not sufficient
        available: u64,
        required: u64,
    },
    InvalidAmount(u64),         // Invalid amount (0 or negative)
    InvalidAccountName(String), // Invalid account name
    TransferToSelf(String),     // Transfer to self
}

// TODO 2.2: Implement Display trait for AppError
// Display trait allows formatting errors as readable strings
impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::AccountNotFound(name) => {
                write!(f, "Account not found: {}", name)
            }
            AppError::AccountAlreadyExists(name) => {
                write!(f, "Account already exists {}", name)
            }
            AppError::InsufficientBalance {
                available,
                required,
            } => {
                write!(
                    f,
                    "Insufficent balance. Available: {}, Required: {}",
                    available, required
                )
            }
            AppError::InvalidAmount(amount) => {
                write!(f, "Amount invalid {}", amount)
            }
            AppError::InvalidAccountName(name) => {
                write!(f, "Account invalid {}", name)
            }
            AppError::TransferToSelf(name) => {
                write!(f, "Cannot transfer to self. Account: {}", name)
            }
        }
    }
}

// TODO 2.3: Implement std::error::Error trait
// Error trait is the standard trait for all error types in Rust
impl error::Error for AppError {}

// TODO 2.4: Implement helper methods for AppError
impl AppError {
    /// Check if account name is valid
    pub fn validate_account_name(name: &str) -> Result<()> {
        if name.trim().len() < 2 {
            return Err(AppError::InvalidAccountName(name.to_string()));
        }
        Ok(())
    }

    /// Check if amount is valid
    pub fn validate_amount(amount: u64) -> Result<()> {
        if amount == 0 {
            return Err(AppError::InvalidAmount(amount));
        }
        Ok(())
    }

    /// Create error for insufficient balance
    pub fn insufficient_balance(available: u64, required: u64) -> Self {
        AppError::InsufficientBalance {
            available,
            required,
        }
    }

    /// Get error code (for logging or tracking)
    pub fn code(&self) -> &str {
        match self {
            // TODO: Add code for each variant
            AppError::AccountNotFound(_) => "ACC_001",
            AppError::AccountAlreadyExists(_) => "ACC_002",
            AppError::InsufficientBalance { .. } => "BAL_001",
            AppError::InvalidAmount(_) => "VAL_001",
            AppError::InvalidAccountName(_) => "VAL_002",
            AppError::TransferToSelf(_) => "TX_001",
        }
    }

    /// Check if error is recoverable
    pub fn is_recoverable(&self) -> bool {
        // TODO 2.6: Some errors can be recovered (like InsufficientBalance)
        // Some cannot (like InvalidAccountName)
        // Return true/false based on error type
        match self {
            AppError::InsufficientBalance { .. } => true,
            AppError::AccountNotFound(_) => true,
            _ => false,
        }
    }
}

// TODO 2.7: Implement From trait to convert from &str to AppError
// This is useful when you want to quickly create an error from a string
impl From<&str> for AppError {
    fn from(msg: &str) -> Self {
        AppError::InvalidAccountName(msg.to_string())
    }
}

// Type alias to make code more concise
pub type Result<T> = std::result::Result<T, AppError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        // TODO 2.8: Test Display implementation
        // Create an error and check the message
        let err = AppError::AccountNotFound("Alice".to_string());
        assert_eq!(format!("{}", err), "Account not found: Alice");

        let err = AppError::InsufficientBalance { available: 50, required: 100 };
        assert_eq!(format!("{}", err), "Insufficent balance. Available: 50, Required: 100");
        
        let err = AppError::TransferToSelf("Bob".to_string());
        assert_eq!(format!("{}", err), "Cannot transfer to self. Account: Bob");
    }

    #[test]
    fn test_validate_account_name() {
        // TODO 2.9: Test validation logic
        // Test with valid and invalid names
        assert!(AppError::validate_account_name("JD").is_ok());
        assert!(AppError::validate_account_name("Alice").is_ok());

        // Invalid case: empty or only whitespace
        assert!(AppError::validate_account_name("").is_err());
        assert!(AppError::validate_account_name(" ").is_err());

        // Invalid case: too short
        let result = AppError::validate_account_name("A");
        assert!(matches!(result, Err(AppError::InvalidAccountName(_))));
    }

    #[test]
    fn test_error_code() {
        // TODO 2.10: Test error codes
        // Check each error has correct code
        assert_eq!(AppError::AccountNotFound("".to_string()).code(), "ACC_001");
        assert_eq!(AppError::AccountAlreadyExists("".to_string()).code(), "ACC_002");
        assert_eq!(AppError::InsufficientBalance { available: 0, required: 0 }.code(), "BAL_001");
        assert_eq!(AppError::InvalidAmount(0).code(), "VAL_001");
        assert_eq!(AppError::InvalidAccountName("".to_string()).code(), "VAL_002");
        assert_eq!(AppError::TransferToSelf("".to_string()).code(), "TX_001");
    }

    #[test]
    fn test_is_recoverable() {
        // TODO 2.11: Test recoverable logic
        let err_balance = AppError::InsufficientBalance { available: 0, required: 10 };
        let err_not_found = AppError::AccountNotFound("Unknown".to_string());
        assert!(err_balance.is_recoverable());
        assert!(err_not_found.is_recoverable());

        // Non-recoverable errors
        let err_invalid_name = AppError::InvalidAccountName("X".to_string());
        let err_self_transfer = AppError::TransferToSelf("Alice".to_string());
        assert!(!err_invalid_name.is_recoverable());
        assert!(!err_self_transfer.is_recoverable());
    }
}
