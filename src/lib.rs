// Import necessary Solana SDK structs and macros
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    program_pack::{IsInitialized, Pack, Sealed},
    pubkey::Pubkey,
    system_instruction,
    sysvar::{rent::Rent, Sysvar},
};

// Import video account models
mod video_account;
use video_account::VideoAccount;

// Define the program ID
solana_program::declare_id!("YOUR_PROGRAM_ID");

// Define the entry point for your program
#[entrypoint]
fn entry(program_id: &Pubkey, accounts: &[AccountInfo], instruction_data: &[u8]) -> ProgramResult {
    // Match on the instruction data to determine which instruction was called
    match instruction_data[0] {
        // Handle the "create video account" instruction
        0 => {
            // Parse the name and description strings from the instruction data
            let name_len = instruction_data[1] as usize;
            let name_str = std::str::from_utf8(&instruction_data[2..2 + name_len]).unwrap();
            let desc_len = instruction_data[2 + name_len] as usize;
            let desc_str =
                std::str::from_utf8(&instruction_data[3 + name_len..3 + name_len + desc_len])
                    .unwrap();

            // Get the account info for the user account, which will pay for the account creation
            let user_info = next_account_info(accounts)?;

            // Create a new video account
            let video_account = VideoAccount::new(program_id, user_info.key, name_str, desc_str)?;

            // Save the new video account to the account data store
            video_account.save(&mut user_info.data.borrow_mut())?;

            // Return a success status code
            Ok(())
        }

        // Handle other instructions here

        // Return an error if the instruction code is invalid
        _ => Err(ProgramError::InvalidInstructionData),
    }
}
