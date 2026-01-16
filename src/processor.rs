// ============================================
// EXERCISE 4: INSTRUCTION PROCESSOR (STATELESS)
// ============================================
// Objectives: Learn about Pattern Matching, Result handling, and Stateless architecture

use crate::instruction::Instruction;
use crate::state::AppState;
use crate::error::{AppError, Result};

/// Main function to process instruction
pub fn process_instruction(instruction: &Instruction, state: &mut AppState) -> Result<String> {
    // TODO 4.1: Validate instruction before processing
    // Map the String error from Instruction::validate to AppError
    instruction.validate().map_err(AppError::InvalidAccountName)?;

    // TODO 4.2: Pattern match on instruction to handle each type
    match instruction {
        Instruction::CreateAccount { name, balance } => {
            process_create_account(name, *balance, state)
        }
        Instruction::Transfer { from, to, amount } => {
            process_transfer(from, to, *amount, state)
        }
        Instruction::UpdateBalance { name, amount } => {
            process_update_balance(name, *amount, state)
        }
        Instruction::GetBalance { name } => {
            process_get_balance(name, state)
        }
        Instruction::DeleteAccount { name } => {
            process_delete_account(name, state)
        }
    }
}

/// Process create account instruction
fn process_create_account(name: &str, balance: u64, state: &mut AppState) -> Result<String> {
    // TODO 4.3: Implement create account logic
    state.create_account(name.to_string(), balance)?;
    Ok(format!("Created account '{}' with balance {}", name, balance))
}

/// Process transfer instruction
fn process_transfer(from: &str, to: &str, amount: u64, state: &mut AppState) -> Result<String> {
    // TODO 4.4: Implement transfer logic
    if from == to {
        return Err(AppError::TransferToSelf(from.to_string()));
    }
    state.transfer(from, to, amount)?;
    Ok(format!("Transferred {} from '{}' to '{}'", amount, from, to))
}

/// Process update balance instruction
fn process_update_balance(name: &str, amount: u64, state: &mut AppState) -> Result<String> {
    // TODO 4.5: Implement update balance logic
    let account = state.get_account_mut(name)?;
    account.add_balance(amount)?;
    Ok(format!("Added {} to account '{}'", amount, name))
}

/// Process get balance instruction
fn process_get_balance(name: &str, state: &AppState) -> Result<String> {
    // TODO 4.6: Implement get balance logic
    let account = state.get_account(name)?;
    let balance = account.get_balance();
    Ok(format!("Account '{}' balance: {}", name, balance))
}

/// Process delete account instruction
fn process_delete_account(name: &str, state: &mut AppState) -> Result<String> {
    // TODO 4.7: Implement delete account logic
    let account = state.get_account(name)?;
    if account.get_balance() > 0 {
        return Err(AppError::InvalidAmount(account.get_balance()));
    }
    let deleted_account = state.delete_account(name)?;
    Ok(format!("Deleted account '{}'", deleted_account.name))
}

// ============================================
// ADVANCED SECTION (OPTIONAL)
// ============================================

/// Batch processing: Process multiple instructions at once
pub fn process_batch(instructions: &[Instruction], state: &mut AppState) -> Vec<Result<String>> {
    // TODO 4.8: ADVANCED - Implement batch processing
    instructions
        .iter()
        .map(|instruction| process_instruction(instruction, state))
        .collect()
}

/// Transaction: Process multiple instructions, rollback if error
/// Note: This requires AppState to implement Clone (derived in state.rs)
pub fn process_transaction(
    instructions: &[Instruction],
    state: &mut AppState,
) -> Result<Vec<String>> {
    // TODO 4.9: ADVANCED - Implement transaction with rollback
    let original_state = state.clone(); 
    let mut results = Vec::new();

    for instruction in instructions {
        match process_instruction(instruction, state) {
            Ok(msg) => results.push(msg),
            Err(e) => {
                *state = original_state; // Rollback!
                return Err(e);
            }
        }
    }

    Ok(results)
}

/// Logging wrapper: Log every instruction before processing
pub fn process_with_logging(
    instruction: &Instruction,
    state: &mut AppState,
) -> Result<String> {
    // TODO 4.10: ADVANCED - Add logging
    println!("[LOG] Processing: {:?}", instruction);
    let result = process_instruction(instruction, state);
    match &result {
        Ok(msg) => println!("[LOG] Success: {}", msg),
        Err(e) => println!("[LOG] Error: {}", e),
    }
    result
}

/// Dry run: Check if instruction can be executed (don't modify state)
pub fn dry_run(instruction: &Instruction, state: &AppState) -> Result<String> {
    // TODO 4.11: ADVANCED - Implement dry run
    let mut state_clone = state.clone();
    process_instruction(instruction, &mut state_clone)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::instruction::Instruction;

    #[test]
    fn test_process_create_account() {
        // TODO 4.12: Test processing CreateAccount instruction
        let mut state = AppState::new();
        let instruction = Instruction::CreateAccount {
            name: String::from("Alice"),
            balance: 1000,
        };
        let result = process_instruction(&instruction, &mut state);
        assert!(result.is_ok());
        assert!(state.account_exists("Alice"));
        assert_eq!(state.get_account("Alice").unwrap().balance, 1000);
    }

    #[test]
    fn test_process_transfer() {
        // TODO 4.13: Test transfer instruction
        let mut state = AppState::new();
        state.create_account("Alice".into(), 1000).unwrap();
        state.create_account("Bob".into(), 500).unwrap();

        let inst = Instruction::transfer("Alice".into(), "Bob".into(), 300);
        let result = process_instruction(&inst, &mut state);
        
        assert!(result.is_ok());
        assert_eq!(state.get_account("Alice").unwrap().balance, 700);
        assert_eq!(state.get_account("Bob").unwrap().balance, 800);
    }

    #[test]
    fn test_transfer_insufficient_balance() {
        // TODO 4.14: Test transfer when insufficient balance
        let mut state = AppState::new();
        state.create_account("Alice".into(), 100).unwrap();
        state.create_account("Bob".into(), 500).unwrap();

        let inst = Instruction::transfer("Alice".into(), "Bob".into(), 200);
        let result = process_instruction(&inst, &mut state);
        
        assert!(matches!(result, Err(AppError::InsufficientBalance { .. })));
    }

    #[test]
    fn test_process_batch() {
        // TODO 4.15: Test batch processing
        let mut state = AppState::new();
        let insts = vec![
            Instruction::create_account("Alice".into(), 1000),
            Instruction::create_account("Bob".into(), 1000),
            Instruction::transfer("Alice".into(), "Bob".into(), 500),
        ];
        
        let results = process_batch(&insts, &mut state);
        assert_eq!(results.len(), 3);
        assert!(results.iter().all(|r| r.is_ok()));
        assert_eq!(state.get_account("Alice").unwrap().balance, 500);
    }

    #[test]
    fn test_process_transaction_rollback() {
        // TODO 4.16: ADVANCED - Test transaction rollback
        let mut state = AppState::new();
        state.create_account("Alice".into(), 1000).unwrap();

        let insts = vec![
            Instruction::update_balance("Alice".into(), 500), // Should succeed (1500)
            Instruction::transfer("Alice".into(), "NonExistent".into(), 100), // Should fail
        ];

        let result = process_transaction(&insts, &mut state);
        
        assert!(result.is_err());
        // Verify Alice's balance rolled back to 1000, not 1500
        assert_eq!(state.get_account("Alice").unwrap().balance, 1000);
    }
}