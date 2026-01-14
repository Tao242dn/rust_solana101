// ============================================
// EXERCISE 4: INSTRUCTION PROCESSOR (STATELESS)
// ============================================
// Objectives: Learn about Pattern Matching, Result handling, and Stateless architecture

use crate::instruction::Instruction;
use crate::state::AppState;
use crate::error::Result;

/// Main function to process instruction
/// This is the center of stateless architecture:
/// - Receive instruction (contains all necessary data)
/// - Receive mutable reference to state
/// - Execute operation and update state
/// - Return result or error
pub fn process_instruction(instruction: &Instruction, state: &mut AppState) -> Result<String> {
    // TODO 4.1: Validate instruction before processing
    // instruction.validate()?;

    // TODO 4.2: Pattern match on instruction to handle each type
    // Uncomment and complete:

    // match instruction {
    //     Instruction::CreateAccount { name, balance } => {
    //         process_create_account(name, *balance, state)
    //     }
    //     Instruction::Transfer { from, to, amount } => {
    //         process_transfer(from, to, *amount, state)
    //     }
    //     // TODO: Add other variants
    //     // ...
    // }

    todo!("Implement process_instruction")
}

/// Process create account instruction
fn process_create_account(name: &str, balance: u64, state: &mut AppState) -> Result<String> {
    // TODO 4.3: Implement create account logic
    // 1. Call state.create_account(...)
    // 2. Return success message
    //
    // Hint:
    // state.create_account(name.to_string(), balance)?;
    // Ok(format!("Created account '{}' with balance {}", name, balance))
    todo!("Implement process_create_account")
}

/// Process transfer instruction
fn process_transfer(from: &str, to: &str, amount: u64, state: &mut AppState) -> Result<String> {
    // TODO 4.4: Implement transfer logic
    // 1. Check from != to
    // 2. Call state.transfer(...)
    // 3. Return success message with details
    //
    // Hint:
    // if from == to {
    //     return Err(AppError::TransferToSelf(from.to_string()));
    // }
    // state.transfer(from, to, amount)?;
    // Ok(format!("Transferred {} from '{}' to '{}'", amount, from, to))
    todo!("Implement process_transfer")
}

/// Process update balance instruction
fn process_update_balance(name: &str, amount: u64, state: &mut AppState) -> Result<String> {
    // TODO 4.5: Implement update balance logic
    // 1. Get mutable reference to account
    // 2. Call add_balance(...)
    // 3. Return success message
    //
    // Hint:
    // let account = state.get_account_mut(name)?;
    // account.add_balance(amount)?;
    // Ok(format!("Added {} to account '{}'", amount, name))
    todo!("Implement process_update_balance")
}

/// Process get balance instruction
fn process_get_balance(name: &str, state: &AppState) -> Result<String> {
    // TODO 4.6: Implement get balance logic
    // 1. Get account
    // 2. Get balance
    // 3. Return information
    //
    // Note: This function doesn't need &mut AppState because it only reads!
    //
    // Hint:
    // let account = state.get_account(name)?;
    // let balance = account.get_balance();
    // Ok(format!("Account '{}' balance: {}", name, balance))
    todo!("Implement process_get_balance")
}

/// Process delete account instruction
fn process_delete_account(name: &str, state: &mut AppState) -> Result<String> {
    // TODO 4.7: Implement delete account logic
    // 1. Check balance = 0 before deleting (safety)
    // 2. Call state.delete_account(...)
    // 3. Return success message
    //
    // Hint:
    // let account = state.get_account(name)?;
    // if account.get_balance() > 0 {
    //     return Err(AppError::InvalidAmount(account.get_balance()));
    // }
    // let deleted_account = state.delete_account(name)?;
    // Ok(format!("Deleted account '{}'", deleted_account.name))
    todo!("Implement process_delete_account")
}

// ============================================
// ADVANCED SECTION (OPTIONAL)
// ============================================

/// Batch processing: Process multiple instructions at once
pub fn process_batch(instructions: &[Instruction], state: &mut AppState) -> Vec<Result<String>> {
    // TODO 4.8: ADVANCED - Implement batch processing
    // Iterate through all instructions and process them
    // Collect results into Vec
    //
    // Hint:
    // instructions
    //     .iter()
    //     .map(|instruction| process_instruction(instruction, state))
    //     .collect()
    todo!("Implement process_batch")
}

/// Transaction: Process multiple instructions, rollback if error
pub fn process_transaction(
    instructions: &[Instruction],
    state: &mut AppState,
) -> Result<Vec<String>> {
    // TODO 4.9: ADVANCED - Implement transaction with rollback
    // This is a difficult exercise about cloning and error handling
    //
    // Idea:
    // 1. Clone initial state (to be able to rollback)
    // 2. Process each instruction
    // 3. If error, restore state from clone and return error
    // 4. If all successful, return Vec<String> messages
    //
    // Hint:
    // let original_state = state.clone(); // Need to implement Clone for AppState!
    // let mut results = Vec::new();
    //
    // for instruction in instructions {
    //     match process_instruction(instruction, state) {
    //         Ok(msg) => results.push(msg),
    //         Err(e) => {
    //             *state = original_state; // Rollback!
    //             return Err(e);
    //         }
    //     }
    // }
    //
    // Ok(results)
    todo!("Implement process_transaction")
}

/// Logging wrapper: Log every instruction before processing
pub fn process_with_logging(
    instruction: &Instruction,
    state: &mut AppState,
) -> Result<String> {
    // TODO 4.10: ADVANCED - Add logging
    // Print instruction before and after processing
    //
    // Hint:
    // println!("[LOG] Processing: {:?}", instruction);
    // let result = process_instruction(instruction, state);
    // match &result {
    //     Ok(msg) => println!("[LOG] Success: {}", msg),
    //     Err(e) => println!("[LOG] Error: {}", e),
    // }
    // result
    todo!("Implement process_with_logging")
}

/// Dry run: Check if instruction can be executed (don't modify state)
pub fn dry_run(instruction: &Instruction, state: &AppState) -> Result<String> {
    // TODO 4.11: ADVANCED - Implement dry run
    // Clone state, process on clone, return result but don't modify original state
    //
    // Hint:
    // let mut state_clone = state.clone();
    // process_instruction(instruction, &mut state_clone)
    todo!("Implement dry_run")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::instruction::Instruction;

    #[test]
    fn test_process_create_account() {
        // TODO 4.12: Test processing CreateAccount instruction
        // let mut state = AppState::new();
        // let instruction = Instruction::CreateAccount {
        //     name: String::from("Alice"),
        //     balance: 1000,
        // };
        // let result = process_instruction(&instruction, &mut state);
        // assert!(result.is_ok());
        // assert!(state.account_exists("Alice"));
        todo!("Write test for process_create_account")
    }

    #[test]
    fn test_process_transfer() {
        // TODO 4.13: Test transfer instruction
        // 1. Create 2 accounts
        // 2. Transfer from account 1 to account 2
        // 3. Verify balances
        todo!("Write test for process_transfer")
    }

    #[test]
    fn test_transfer_insufficient_balance() {
        // TODO 4.14: Test transfer when insufficient balance
        // Verify that correct error is returned
        todo!("Write test for insufficient balance")
    }

    #[test]
    fn test_process_batch() {
        // TODO 4.15: Test batch processing
        todo!("Write test for process_batch")
    }

    #[test]
    fn test_process_transaction_rollback() {
        // TODO 4.16: ADVANCED - Test transaction rollback
        // Create batch of instructions, one will fail
        // Verify that state is rolled back to initial state
        todo!("Write test for transaction rollback")
    }
}
