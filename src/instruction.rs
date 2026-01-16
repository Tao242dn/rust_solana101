// ============================================
// EXERCISE 1: INSTRUCTION ENUM DEFINITION
// ============================================
// Objectives: Learn about Enums, Pattern Matching, and Stateless Instructions

/// Instruction represents actions that can be performed on the system
/// In stateless architecture, each instruction contains all necessary data
/// to execute without depending on previous state

#[derive(Debug, Clone)]
pub enum Instruction {
    // TODO 1.1: Define CreateAccount variant
    // Hint: Needs name (String) and balance (u64) fields
    // Uncomment the line below and complete:
    CreateAccount {
        name: String,
        balance: u64,
    },

    // TODO 1.2: Define Transfer variant
    // Hint: Needs from (String), to (String), and amount (u64)
    // Uncomment and complete:
    Transfer {
        from: String,
        to: String,
        amount: u64,
    },

    // TODO 1.3: Define UpdateBalance variant
    // Hint: Needs name (String) and amount (u64) to add
    // Uncomment and complete:
    UpdateBalance {
        name: String,
        amount: u64,
    },

    // TODO 1.4: Define GetBalance variant
    // Hint: Only needs name (String) to retrieve balance
    // Uncomment and complete:
    GetBalance {
        name: String,
    },

    // TODO 1.5: Define DeleteAccount variant
    // Hint: Only needs name (String) of the account to delete
    // Uncomment and complete:
    DeleteAccount {
        name: String,
    },
}

// TODO 1.6: Implement methods for Instruction
impl Instruction {
    /// Create a new CreateAccount instruction
    pub fn create_account(name: String, balance: u64) -> Self {
        // TODO: Return Instruction::CreateAccount with the given parameters
        return Instruction::CreateAccount { name, balance };
    }

    /// Create a new Transfer instruction
    pub fn transfer(from: String, to: String, amount: u64) -> Self {
        // TODO: Return Instruction::Transfer with the given parameters
        return Instruction::Transfer { from, to, amount };
    }

    /// Create a new UpdateBalance instruction
    pub fn update_balance(name: String, amount: u64) -> Self {
        // TODO: Return Instruction::UpdateBalance with the given parameters
        return Instruction::UpdateBalance { name, amount };
    }

    /// Get a short description of the instruction
    pub fn description(&self) -> String {
        // TODO 1.7: Use pattern matching to return description for each variant
        // Hint:
        // match self {
        //     Instruction::CreateAccount { name, .. } => format!("Create account {}", name),
        //     ...
        // }
        match self {
            Instruction::CreateAccount { name, balance } => {
                format!(
                    "Create account '{}' with initial balance of {}",
                    name, balance
                )
            }
            Instruction::Transfer { from, to, amount } => {
                format!("Transfer {} from '{}' to '{}'", amount, from, to)
            }
            Instruction::UpdateBalance { name, amount } => {
                format!("Update balance for '{}' by adding {}", name, amount)
            }
            Instruction::GetBalance { name } => {
                format!("Retrieve balance for account '{}'", name)
            }
            Instruction::DeleteAccount { name } => {
                format!("Delete account '{}'", name)
            }
        }
    }

    /// Check if the instruction is valid
    pub fn validate(&self) -> Result<(), String> {
        // TODO 1.8: Validate invalid cases
        // - Account name must not be empty
        // - Amount must be > 0 (for CreateAccount, Transfer, UpdateBalance)
        // Hint: Use match and return Err(...) if invalid

        match self {
            Instruction::CreateAccount { name, balance } => {
                if name.is_empty() {
                    return Err("Account name cannot be empty".to_string());
                }
                if *balance == 0 {
                    return Err("Initial balance must be greater than 0".to_string());
                }
                Ok(())
            }
            Instruction::Transfer { from, to, amount } => {
                if from.is_empty() || to.is_empty() {
                    return Err("Sender and receive names can not be empty".to_string());
                }
                if from == to {
                    return Err("Cannot transfer to the same account".to_string());
                }
                if *amount == 0 {
                    return Err("Transfer amount must be greater than 0".to_string());
                }
                Ok(())
            }
            Instruction::UpdateBalance { name, amount } => {
                if name.is_empty() {
                    return Err("Account name cannot be empty".to_string());
                }
                if *amount == 0 {
                    return Err("Update amount must be greater than 0".to_string());
                }
                Ok(())
            }
            Instruction::GetBalance { name } => {
                if name.is_empty() {
                    return Err("Account name cannot be empty".to_string());
                }
                Ok(())
            }
            Instruction::DeleteAccount { name } => {
                if name.is_empty() {
                    return Err("Account name cannot be empty".to_string());
                }
                Ok(())
            }
        }
    }
}

// TODO 1.9: Implement Display trait for Instruction
// Hint: use std::fmt; and implement fmt::Display
// To print instructions in a nice format
impl std::fmt::Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.description())
    }
}

#[cfg(test)]    
mod tests {
    use crate::instruction::Instruction;

    #[test]
    fn test_create_account_instruction() {
        // TODO 1.10: Write test case for CreateAccount
        // Create an instruction and check the fields
        let name = "Alice".to_string();
        let balance = 100;
        let inst = Instruction::create_account(name.clone(), balance);

        if let Instruction::CreateAccount {
            name: inst_name,
            balance: inst_balance,
        } = inst
        {
            assert_eq!(inst_name, name);
            assert_eq!(inst_balance, balance);
        } else {
            panic!("Instruction was not CreateAccount")
        }
    }

    #[test]
    fn test_transfer_instruction() {
        // TODO 1.11: Write test case for Transfer
        let from = "Alice".to_string();
        let to = "Bob".to_string();
        let amount = 50;
        let inst = Instruction::transfer(from.clone(), to.clone(), amount);

        if let Instruction::Transfer {
            from: f,
            to: t,
            amount: a,
        } = inst
        {
            assert_eq!(f, from);
            assert_eq!(t, to);
            assert_eq!(a, amount);
        } else {
            panic!("Instruction was not Transfer")
        }
    }

    #[test]
    fn test_validation() {
        // TODO 1.12: Write test case to check validation
        // - Test with empty name
        // - Test with amount = 0
        let valid = Instruction::create_account("Alice".to_string(), 100);
        assert!(valid.validate().is_ok());

        let empty_name = Instruction::create_account("".to_string(), 50);
        assert!(empty_name.validate().is_err());
        assert_eq!(
            empty_name.validate().unwrap_err(),
            "Account name cannot be empty"
        );

        let zero_amount = Instruction::update_balance("Alice".to_string(), 0);
        assert!(zero_amount.validate().is_err());
        assert_eq!(
            zero_amount.validate().unwrap_err(),
            "Update amount must be greater than 0"
        );

        let self_transfer = Instruction::transfer("Alice".to_string(), "Alice".to_string(), 100);
        assert!(self_transfer.validate().is_err());
    }
}
