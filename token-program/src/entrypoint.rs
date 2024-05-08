// the entrypoint.rs file. The code here performs the following:
// Defines the program as a Solana program
// Defines the entrypoint for the program as the ‘process_instruction’ function
// Creates the process_instrunction function, which simply calls the process_instruction instruction from the Processor implementation, passing in all the input parameters received

use crate::processor::Processor;

use solana_program::{
   account_info::{AccountInfo},
   entrypoint,
   entrypoint::ProgramResult,
   msg,
   pubkey::Pubkey,
};

// Declare and export the program's entrypoint
entrypoint!(process_instruction);


fn process_instruction(
   program_id: &Pubkey,
   accounts: &[AccountInfo],
   instruction_data: &[u8],
) -> ProgramResult {
   msg!(
       "process_instruction: {}: {} accounts, data={:?}",
       program_id,
       accounts.len(),
       instruction_data
   );

   Processor::process_instruction(program_id, accounts, instruction_data)
}