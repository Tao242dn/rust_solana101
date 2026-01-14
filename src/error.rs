// ============================================
// EXERCISE 2: CUSTOM ERROR TYPES
// ============================================
// Objectives: Learn about Error Handling, Result<T, E>, and Custom Error Types

use std::fmt;

/// Custom error type for the application
/// In Rust, error handling is done through Result<T, E>
#[derive(Debug, Clone, PartialEq)]
pub enum AppError {
    // TODO 2.1: Define error variants
    // Uncomment and complete the following variants:

    // AccountNotFound(String),      // Account doesn't exist (contains account name)
    // AccountAlreadyExists(String), // Account already exists
    // InsufficientBalance {         // Balance is not sufficient
    //     available: u64,
    //     required: u64,
    // },
    // InvalidAmount(u64),           // Invalid amount (0 or negative)
    // InvalidAccountName(String),   // Invalid account name
    // TransferToSelf(String),       // Transfer to self
}

// TODO 2.2: Implement Display trait for AppError
// Display trait allows formatting errors as readable strings
impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // TODO: Use match to create appropriate message for each error
        // Example:
        // match self {
        //     AppError::AccountNotFound(name) => {
        //         write!(f, "Account not found: {}", name)
        //     },
        //     AppError::InsufficientBalance { available, required } => {
        //         write!(f, "Insufficient balance. Available: {}, Required: {}", available, required)
        //     },
        //     ...
        // }
        todo!("Implement Display for AppError")
    }
}

// TODO 2.3: Implement std::error::Error trait
// Error trait is the standard trait for all error types in Rust
// impl std::error::Error for AppError {}

// TODO 2.4: Implement helper methods for AppError
impl AppError {
    /// Check if account name is valid
    pub fn validate_account_name(name: &str) -> Result<(), Self> {
        // TODO: Check name is not empty and has at least 2 characters
        // Return Ok(()) if valid
        // Return Err(AppError::InvalidAccountName(...)) if invalid
        todo!("Implement validate_account_name")
    }

    /// Check if amount is valid
    pub fn validate_amount(amount: u64) -> Result<(), Self> {
        // TODO: Check amount > 0
        // Return Err(AppError::InvalidAmount(...)) if invalid
        todo!("Implement validate_amount")
    }

    /// Create error for insufficient balance
    pub fn insufficient_balance(available: u64, required: u64) -> Self {
        // TODO: Return AppError::InsufficientBalance with given values
        todo!("Implement insufficient_balance")
    }

    /// Get error code (for logging or tracking)
    pub fn code(&self) -> &str {
        // TODO 2.5: Return code for each error type
        // Example: "ACC_001", "ACC_002", "BAL_001", etc.
        match self {
            // TODO: Add code for each variant
            _ => "UNKNOWN",
        }
    }

    /// Check if error is recoverable
    pub fn is_recoverable(&self) -> bool {
        // TODO 2.6: Some errors can be recovered (like InsufficientBalance)
        // Some cannot (like InvalidAccountName)
        // Return true/false based on error type
        match self {
            // TODO: Add logic for each variant
            _ => false,
        }
    }
}

// TODO 2.7: Implement From trait to convert from &str to AppError
// This is useful when you want to quickly create an error from a string
// impl From<&str> for AppError {
//     fn from(msg: &str) -> Self {
//         AppError::InvalidAccountName(msg.to_string())
//     }
// }

// Type alias to make code more concise
pub type Result<T> = std::result::Result<T, AppError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        // TODO 2.8: Test Display implementation
        // Create an error and check the message
        todo!("Write test for error display")
    }

    #[test]
    fn test_validate_account_name() {
        // TODO 2.9: Test validation logic
        // Test with valid and invalid names
        todo!("Write test for account name validation")
    }

    #[test]
    fn test_error_code() {
        // TODO 2.10: Test error codes
        // Check each error has correct code
        todo!("Write test for error codes")
    }

    #[test]
    fn test_is_recoverable() {
        // TODO 2.11: Test recoverable logic
        todo!("Write test for is_recoverable")
    }
}
