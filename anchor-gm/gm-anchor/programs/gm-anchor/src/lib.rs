/* This code
Defines a new anchor program
Defines a new ‘execute’ function, which takes an account from the context, and a ‘name’ parameter, then stores the name string into the specified account, and prints the name out the program output
Creates an ‘Execute’ struct that defines the accounts passed into the execute instruction, and the deserialization of the gm_account account into a ‘GreetingAccount’ struct
Defines the GreetingAccount struct, that stores the name string */

use anchor_lang::prelude::*;

declare_id!("8WmtfLwXaZV4hpAg2sxLDPLmL9Y73oEGeykfH1AfZDkP");

#[program]
pub mod gm_anchor {
   use super::*;
   pub fn execute(ctx: Context<Execute>, name: String) -> ProgramResult {
       let gm_account = &mut ctx.accounts.gm_account;

       gm_account.name = name;
       msg!("GM {}", gm_account.name);
       Ok(())
   }
}

#[derive(Accounts)]
pub struct Execute<'info> {
   #[account(init, payer = user, space = 8 + 32)]
   pub gm_account: Account<'info, GreetingAccount>,
   #[account(mut)]
   pub user: Signer<'info>,
   pub system_program: Program<'info, System>,
}

#[account]
pub struct GreetingAccount {
   pub name: String,
}