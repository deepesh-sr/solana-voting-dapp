#![allow(clippy::result_large_err)]

use std::convert::Infallible;

use anchor_lang::prelude::*;

declare_id!("FqzkXZdwYjurnUKetJCAvaUw5WAqbwzU6gZEwydeEfqS");

#[program]
pub mod vote {
    use super::*;

    pub fn initialise_poll(_ctx:Context<InitializePoll>, _poll_id : u64 )-> Result<()>{
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(poll_id:u64)]
pub struct InitializePoll<'info>{
    #[account(mut)]
    pub payer : Signer<'info>,
    #[account(
        init_if_needed,
        payer = payer ,
        space = 8 + Poll::INIT_SPACE,
        seeds = [poll_id.to_le_bytes().as_ref()],
        bump
    )]
    pub poll : Account<'info,Poll>,
    pub system_program : Program<'info,System>
}


#[account]
#[derive(InitSpace)]
pub struct Poll {
    
    poll_id:u64,
    #[max_len(64)]
    poll_description:String,
    poll_start:u64,
    poll_end:u64,
    candidate_amount: u8,
}
