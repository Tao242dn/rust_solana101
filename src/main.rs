// Rust Basic and Stateless Instruction Exercise
// Module declarations
mod instruction;
mod processor;
mod state;
mod error;

use instruction::Instruction;
use processor::process_instruction;
use state::AppState;

fn main() {
    println!("=== RUST BASIC AND STATELESS INSTRUCTION ===\n");

    // Initialize initial state
    let mut app_state = AppState::new();

    println!("Initial state:");
    app_state.display();

    // List of instructions to execute
    let instructions = vec![
        Instruction::CreateAccount {
            name: String::from("Alice"),
            balance: 1000,
        },
        Instruction::CreateAccount {
            name: String::from("Bob"),
            balance: 500,
        },
        Instruction::Transfer {
            from: String::from("Alice"),
            to: String::from("Bob"),
            amount: 200,
        },
        Instruction::UpdateBalance {
            name: String::from("Alice"),
            amount: 300,
        },
        Instruction::GetBalance {
            name: String::from("Bob"),
        },
        Instruction::DeleteAccount {
            name: String::from("Charlie"), // Will cause error
        },
    ];

    // Process each instruction
    println!("\n=== PROCESSING INSTRUCTIONS ===\n");
    for (index, instruction) in instructions.iter().enumerate() {
        println!("Instruction #{}: {:?}", index + 1, instruction);

        match process_instruction(instruction, &mut app_state) {
            Ok(message) => println!("✓ Success: {}", message),
            Err(e) => println!("✗ Error: {}", e),
        }

        println!();
    }

    // Display final state
    println!("=== FINAL STATE ===");
    app_state.display();
}
