//! # Solana Counter Contract
//! 
//! This is a simple counter contract on Solana that allows incrementing and decrementing a counter.
//! The contract demonstrates basic Solana program concepts including:
//! - Account data serialization/deserialization using Borsh
//! - Instruction handling
//! - Program entrypoint implementation
//! - Account data modification

use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{AccountInfo, next_account_info},
    entrypoint::ProgramResult,
    entrypoint,
    pubkey::Pubkey,
    msg
};

/// Represents the different types of instructions that can be processed by the contract
/// 
/// # Variants
/// * `Increment(u32)` - Increment the counter by the specified amount
/// * `Decrement(u32)` - Decrement the counter by the specified amount
#[derive(BorshDeserialize, BorshSerialize, Debug)]
enum InstructionType {
    Increment(u32),
    Decrement(u32)
}

/// The main counter data structure that will be stored in the account
/// 
/// # Fields
/// * `count` - The current value of the counter
#[derive(BorshDeserialize, BorshSerialize, Debug)]
struct Counter {
    count: u32
}

// Define the program's entrypoint
entrypoint!(counter_contract);

/// The main entrypoint of the program
/// 
/// # Arguments
/// * `program_id` - The public key of the program
/// * `accounts` - The accounts involved in the transaction
/// * `instruction_data` - The serialized instruction data
/// 
/// # Returns
/// * `ProgramResult` - Result indicating success or failure of the program execution
/// 
/// # Errors
/// * Returns error if account data cannot be deserialized
/// * Returns error if instruction data cannot be deserialized
/// * Returns error if account data cannot be serialized
pub fn counter_contract(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8]
) -> ProgramResult {
    // Get the first account from the accounts array
    let acc = next_account_info(&mut accounts.iter())?;

    // Deserialize the instruction data to determine the operation
    let instruction_type = InstructionType::try_from_slice(instruction_data)?;
    
    // Deserialize the current counter data from the account
    let mut counter_data = Counter::try_from_slice(&acc.data.borrow())?; 

    // Process the instruction
    match instruction_type {
        InstructionType::Increment(amount) => {
            msg!("incrementing by {}", amount);
            counter_data.count += amount;
        },
        InstructionType::Decrement(amount) => {
            msg!("decrementing by {}", amount);
            counter_data.count -= amount;
        }
    }

    // Serialize the updated counter data back to the account
    counter_data.serialize(&mut *acc.data.borrow_mut())?;

    msg!("contract succeeded");

    Ok(())
}