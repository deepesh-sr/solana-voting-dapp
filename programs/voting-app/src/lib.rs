use anchor_lang::prelude::*;

declare_id!("8J8nqiGrzSoGGYK8yjQpcZCCXSGhLetfarf4yY8vnDiT");

#[program]
pub mod voting_app {
    use super::*;

    pub fn initialize_poll(
        ctx: Context<InitializePoll>,
        poll_id: u64,
        description: String,
        poll_start: u64,
        poll_end: u64,
    ) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        ctx.accounts.poll.poll_id = poll_id;
        ctx.accounts.poll.poll_start = poll_start;
        ctx.accounts.poll.poll_end = poll_end;
        ctx.accounts.poll.description = description;
        ctx.accounts.poll.candidate_amount = 0;
        Ok(())
    }

    pub fn initialize_candidate(
        ctx: Context<InitializeCandidate>,
        _poll_id: u64,
        candidate_name: String,
    ) -> Result<()> {
        ctx.accounts.candidate_account.set_inner(Candidate {
            candidate_name,
            candidate_vote: 0,
        });
        ctx.accounts.poll.candidate_amount += 1;
        Ok(())
    }

    pub fn vote(ctx: Context<Vote>, poll_id: u64, cadidate_name: String) -> Result<()> {
        let current_time = Clock::get()?.unix_timestamp;
        if current_time > (ctx.accounts.poll.poll_end as i64) {
            return Err(ErrorCode::PollEnded.into());
        }

        if current_time < (ctx.accounts.poll.poll_start as i64) {
            return Err(ErrorCode::PollNotStarted.into());
        }
        ctx.accounts.candidate_account.candidate_vote += 1;
        msg!("Current Time: {}", current_time);
        msg!("Poll Start: {}", ctx.accounts.poll.poll_start);
        msg!("Poll End: {}", ctx.accounts.poll.poll_end);
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(poll_id:u64,candidate_name:String)]
pub struct InitializePoll<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        init,
        payer = signer,
        seeds = [poll_id.to_le_bytes().as_ref()],
        space = 8 + Poll::INIT_SPACE,
        bump,
    )]
    pub poll: Account<'info, Poll>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(poll_id:u64,candidate_name:String)]
pub struct InitializeCandidate<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        mut,
        seeds = [poll_id.to_le_bytes().as_ref()],
        bump,
    )]
    pub poll: Account<'info, Poll>,
    #[account(
        init,
        payer = signer,
        seeds = [poll_id.to_le_bytes().as_ref(),candidate_name.as_ref()],
        space = 8 + Candidate::INIT_SPACE,
        bump,
    )]
    pub candidate_account: Account<'info, Candidate>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(poll_id:u64,candidate_name:String)]
pub struct Vote<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        mut,
        seeds = [poll_id.to_le_bytes().as_ref()],
        bump
    )]
    pub poll: Account<'info, Poll>,
    #[account(
        mut,
        seeds = [poll_id.to_le_bytes().as_ref(),candidate_name.as_ref()],
        bump
    )]
    pub candidate_account: Account<'info, Candidate>,
    pub system_program: Program<'info, System>,
}
#[account]
#[derive(InitSpace)]
pub struct Poll {
    poll_id: u64,
    #[max_len(32)]
    description: String,
    poll_start: u64,
    poll_end: u64,
    candidate_amount: u64,
}

#[account]
#[derive(InitSpace)]
pub struct Candidate {
    #[max_len(16)]
    candidate_name: String,
    candidate_vote: u64,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Poll has not started yet")]
    PollNotStarted,
    #[msg("Poll has ended")]
    PollEnded,
}
