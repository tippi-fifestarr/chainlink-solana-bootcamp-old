/* instruction.rs file. The code here defines an enum that defines all the possible instructions that can be sent to the program:
0 - Create a new token
1 - Create a new token account
2 - Mint some tokens to a token account
3 - Transfer tokens between token accounts

Take note that the Mint and Transfer values can contain an additional piece of data called ‘amount’ */


use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub enum TokenInstruction {
   CreateToken,
   CreateTokenAccount,
   Mint { amount: u64 },
   Transfer { amount: u64 },
}