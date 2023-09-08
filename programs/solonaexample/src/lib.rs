use anchor_lang::prelude::*;


declare_id!("9mAuZCKAbF3uLSbM15Fa7gy9xpYzyTpSQgssGVhNvAtG");

#[program]
pub mod solonaexample {
    use super::*;

    pub fn create(ctx: Context<Initialize>) -> Result<()> {
    
        // Placeholder for a function or expression that returns a Result.
        let result = some_function_that_returns_result(ctx);
    
        match result {
            Ok(_) => {
                println!("Hello World");
                Ok(())
            },
            Err(err) => {
                println!("Error: {}", err);
                Err(err)
            }
        }
    }    
    pub fn some_function_that_returns_result(_ctx: Context<Initialize>) -> Result<()> {
        // print context here
        
        // Some logic here that returns Ok or Err.
        Ok(())
    }
    
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

#[derive(Accounts)]
pub struct Create<'info> {
    #[account(init, payer = user, space = 16 + 16)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program <'info, System>,
}

// Transaction instructions
#[derive(Accounts)]
pub struct Increment<'info> {
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>,
}

// An account that goes inside a transaction instruction
#[account]
pub struct BaseAccount {
    pub count: u64,
}