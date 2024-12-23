#![allow(clippy::result_large_err)]

use anchor_lang::prelude::*;

declare_id!("coUnmi3oBUtwtd9fjeAvSsJssXh5A5xyPbhpewyzRVF");

#[program]
pub mod voting_dapp {
   
    use super::*;

    pub fn initialize_poll(ctx:Context<InitializePoll>, poll_id:u64, description:String, poll_start:u64, poll_end:u64)->Result<()>{
        let poll = &mut ctx.accounts.poll;
        poll.poll_id = poll_id;
        poll.description = description;
        poll.poll_start = poll_start;
        poll.poll_end = poll_end;
        poll.candidate_amount = 0;
        Ok(())
    }
   
    pub fn initialize_candidate(ctx:Context<InitializeCandidate>,canditate_name:String,_poll_id:u64)->Result<()>{
        let candidate = &mut ctx.accounts.candidate;
        candidate.candidate_name = canditate_name;
        candidate.candidate_votes = 0; 
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(candidate_name:String,poll_id:u64)]
pub struct InitializeCandidate<'info>{
    #[account(mut)] 
    pub signer:Signer<'info>,
    #[account(
        seeds=[b"Poll",poll_id.to_le_bytes().as_ref()],
        bump
    )]
    pub poll:Account<'info,PollAccount>,

    #[account(
        init,
        payer=signer,
        space = 8+CandidateAccount::INIT_SPACE,
        seeds=[b"Candidate",poll_id.to_le_bytes().as_ref(),candidate_name.as_bytes().as_ref()],
        bump
    )]
    pub candidate:Account<'info,CandidateAccount>,

    pub system_program:Program<'info,System>,
}

#[derive(Accounts)]
#[instruction(poll_id:u64)]
pub struct InitializePoll<'info>{
    #[account(mut)]
    pub signer:Signer<'info>,
    #[account(
        init,
        payer=signer,
        space = 8+PollAccount::INIT_SPACE,
        seeds=[b"Poll",poll_id.to_le_bytes().as_ref()],
        bump
    )]
    pub poll:Account<'info,PollAccount>,
    pub system_program:Program<'info,System>,
   
}

#[account]
#[derive(InitSpace)]
pub struct PollAccount{
    pub poll_id:u64,
    #[max_len(280)]
    pub description:String,
    pub poll_start:u64,
    pub poll_end:u64,
    pub candidate_amount:u64,
}

#[account]
#[derive(InitSpace)]
pub struct CandidateAccount{
    #[max_len(32)]
    pub candidate_name:String,
    pub candidate_votes:u64,
}