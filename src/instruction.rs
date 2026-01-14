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
    // CreateAccount { name: String, balance: u64 },

    // TODO 1.2: Define Transfer variant
    // Hint: Needs from (String), to (String), and amount (u64)
    // Uncomment and complete:
    // Transfer { from: String, to: String, amount: u64 },

    // TODO 1.3: Define UpdateBalance variant
    // Hint: Needs name (String) and amount (u64) to add
    // Uncomment and complete:
    // UpdateBalance { name: String, amount: u64 },

    // TODO 1.4: Define GetBalance variant
    // Hint: Only needs name (String) to retrieve balance
    // Uncomment and complete:
    // GetBalance { name: String },

    // TODO 1.5: Define DeleteAccount variant
    // Hint: Only needs name (String) of the account to delete
    // Uncomment and complete:
    // DeleteAccount { name: String },
}

// TODO 1.6: Implement methods for Instruction
impl Instruction {
    /// Create a new CreateAccount instruction
    pub fn create_account(name: String, balance: u64) -> Self {
        // TODO: Return Instruction::CreateAccount with the given parameters
        todo!("Implement create_account method")
    }

    /// Create a new Transfer instruction
    pub fn transfer(from: String, to: String, amount: u64) -> Self {
        // TODO: Return Instruction::Transfer with the given parameters
        todo!("Implement transfer method")
    }

    /// Create a new UpdateBalance instruction
    pub fn update_balance(name: String, amount: u64) -> Self {
        // TODO: Return Instruction::UpdateBalance with the given parameters
        todo!("Implement update_balance method")
    }

    /// Get a short description of the instruction
    pub fn description(&self) -> String {
        // TODO 1.7: Use pattern matching to return description for each variant
        // Hint:
        // match self {
        //     Instruction::CreateAccount { name, .. } => format!("Create account {}", name),
        //     ...
        // }
        todo!("Implement description method")
    }

    /// Check if the instruction is valid
    pub fn validate(&self) -> Result<(), String> {
        // TODO 1.8: Validate invalid cases
        // - Account name must not be empty
        // - Amount must be > 0 (for CreateAccount, Transfer, UpdateBalance)
        // Hint: Use match and return Err(...) if invalid

        match self {
            // TODO: Add validation logic for each variant
            _ => Ok(()),
        }
    }
}

// TODO 1.9: Implement Display trait for Instruction
// Hint: use std::fmt; and implement fmt::Display
// To print instructions in a nice format
// impl std::fmt::Display for Instruction {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "{}", self.description())
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_account_instruction() {
        // TODO 1.10: Write test case for CreateAccount
        // Create an instruction and check the fields
        todo!("Write test for create_account")
    }

    #[test]
    fn test_transfer_instruction() {
        // TODO 1.11: Write test case for Transfer
        todo!("Write test for transfer")
    }

    #[test]
    fn test_validation() {
        // TODO 1.12: Write test case to check validation
        // - Test with empty name
        // - Test with amount = 0
        todo!("Write validation tests")
    }
}
