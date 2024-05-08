/*  the processor.rs file. The top section defines it as a Solana program, 
and tells the compiler we want to use the Borsh libraries,
as well as the instruction and state modules that we created earlier. 
In addition to this, it defines a process_instruction function, which takes in the standard Solana Rust parameters,
it retrieves the passed in instruction via the instruction_data parameter,
then depending on what value it has (0,1,2,3), it calls different logic to perform the specified function. */

use borsh::{BorshDeserialize};
use solana_program::{
   account_info::{next_account_info, AccountInfo},
   entrypoint::ProgramResult,
   msg,
   program_error::ProgramError,
   pubkey::Pubkey,
};

use crate::instruction::TokenInstruction;
use crate::state::{Token, TokenAccount};

pub struct Processor {}

impl Processor {
   pub fn process_instruction(
       _program_id: &Pubkey,
       accounts: &[AccountInfo],
       instruction_data: &[u8],
   ) -> ProgramResult {
       let instruction = TokenInstruction::try_from_slice(instruction_data)
           .map_err(|_| ProgramError::InvalidInstructionData)?;
       let accounts_iter = &mut accounts.iter();
       msg!("Instruction: {:?}",instruction);
       match instruction {
           // Create a new token
           /* in this section we simply read in from the accounts array a new account to create a token for, 
           as well as a ‘token authority’ account that acts as an owner of the token, and has authority to do things like mint new tokens etc.
           Then we simply set some default values for the token such as the supply, set the authority/owner to the passed in token authority account, 
           then we save the token data into the passed in token master account. */
           TokenInstruction::CreateToken => {
               msg!("Instruction: Create Token");
               //get account info for master token account
               let token_master_account = next_account_info(accounts_iter)?;
               let token_authority = next_account_info(accounts_iter)?;
               let mut token = Token::load_unchecked(token_master_account)?;

               //set default values and save master token account
               token.authority = *token_authority.key;
               token.supply = 0;
               token.save(token_master_account)?

           }
            /* The code takes in three accounts from the accounts array:
                a New account to create a token account for
                b The master token account that we want to create a token account under
                c The owner of the new token account that we’re creating
            The program then sets the relevant owner and master token values, 
        as well as sets the initial balance to 0, then saves the data in the passed in new token account */
           TokenInstruction::CreateTokenAccount => {
               msg!("Instruction: Create Token Account");
                //get account info for master token account and token account to be created
                let token_account_acct = next_account_info(accounts_iter)?;
                let token_master_account = next_account_info(accounts_iter)?;
                let owner = next_account_info(accounts_iter)?;
                let mut token_account = TokenAccount::load_unchecked(token_account_acct)?;

                //set default values and save token account
                token_account.owner = *owner.key;
                token_account.token = *token_master_account.key;
                token_account.amount = 0;
                token_account.save(token_account_acct)?
           }
            /*  The Mint branch: 
            // In this part, the program looks for three accounts in the accounts array:
            //    The token account that wants to receive the minted tokens
            //    The master token account of the tokens that we want to mint
            //    The token authority account, that has access to mint new tokens
            // The logic then does some basic validation to ensure that the passed in token authority account 
            // is the one that signed the transaction, otherwise it returns an error. After this check passes,
            // it simply increases the total supply in the master token account, and then increases the balance 
            // of the token in the specified token account by the passed in value, and saves the state of the accounts */
           TokenInstruction::Mint { amount } => {
               msg!("Instruction: Mint");
                //get account info for master token account and token account to mint to
                let token_account_acct = next_account_info(accounts_iter)?;
                let token_master_account = next_account_info(accounts_iter)?;
                let mut token_account = TokenAccount::load(token_account_acct)?;
                let mut token = Token::load(token_master_account)?;

                //basic validation, ensure its the master token authority trying to mint
                let token_authority = next_account_info(accounts_iter)?;
                if !token_authority.is_signer {
                    msg!("Only the token owner can mint tokens");
                    return Err(ProgramError::MissingRequiredSignature);
                }

                //update total supply of the master token, and update balance of token account that received the mint
                token.supply += amount;
                token_account.amount += amount;

                //save updated contents of both accounts
                token_account.save(token_account_acct)?;
                token.save(token_master_account)?;
           }
           /* The Transfer branch:
              In this part, the program looks for three accounts in the accounts array:
                The token account that wants to send the tokens
                The token account that wants to receive the tokens
                The token authority account, that has access to transfer tokens
            The logic then does some basic validation to ensure that the passed in token authority account
            is the one that signed the transaction, otherwise it returns an error. After this check passes,
            it simply decreases the balance of the token in the specified token account by the passed in value,
            and then increases the balance of the token in the specified token account by the passed in value,
            and saves the state of the accounts */
           TokenInstruction::Transfer { amount } => {
               msg!("Instruction: Transfer");
               //get account info for from and to token accounts, as well as master token account
               let from_token_acct = next_account_info(accounts_iter)?;
               let to_token_acct = next_account_info(accounts_iter)?;
               let owner = next_account_info(accounts_iter)?;
               let mut src_token_account = TokenAccount::load(from_token_acct)?;
               let mut dst_token_account = TokenAccount::load(to_token_acct)?;

               //basic validation, ensure sender has enough funds
               if src_token_account.amount <= amount {
                   msg!("Not enough tokens to transfer");
                   return Err(ProgramError::InsufficientFunds);
               }

               //ensure the owner of the from account is the one signing the transaction
               if !owner.is_signer {
                   msg!("Not the token owner signing the transaction");
                   return Err(ProgramError::MissingRequiredSignature);
               }

               //ensure the owner passed in is the actual owner of the token account
               if !(src_token_account.owner == *owner.key) {
                   msg!("Not the token account owner signing the transaction");
                   return Err(ProgramError::MissingRequiredSignature);
               }

               //update values in from and to accounts, then save new contents of both accounts
               src_token_account.amount -= amount;
               dst_token_account.amount += amount;
               src_token_account.save(from_token_acct)?;
               dst_token_account.save(to_token_acct)?;
           }
       }
       Ok(())
   }
}