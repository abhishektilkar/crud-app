#![allow(clippy::result_large_err)]

use anchor_lang::prelude::*;

declare_id!("35rmpzLZ2YwxFuPygL1ULcWJKLzj3ZXbraNvJYj4fPXy");

#[program]
pub mod crudapp {
    use super::*;

    pub fn create_journal_entry(
      ctx: Context<CreateEntry>,
      // owner: Pubkey,
      title: String,
      message: String,
    ) -> Result<()> {

      //  &mut ctx.accounts.owner;
      // const owner = *ctx.accounts.owner.key;
      ctx.accounts.journal_entry.set_inner(JournalEntryState {
        owner: *ctx.accounts.owner.key,
        title,
        message,
      });

      Ok(())
    }

    pub fn update_journal_entry(
      ctx: Context<UpdateEntry>,
      _title: String,
      message: String,
    ) -> Result<()> {

      let journal_entry = &mut ctx.accounts.journal_entry;
      // journal_entry.title = title;
      journal_entry.message = message;

      Ok(())
    }

    pub fn delete_journal_entry(
      _ctx: Context<DeleteEntry>,
      _title: String,
    ) -> Result<()> {

      Ok(())
    }

}

#[derive(Accounts)]
#[instruction(title: String)]
pub struct DeleteEntry<'info> {
  #[account(
    mut,
    seeds = [title.as_bytes(), owner.key().as_ref()],
    bump,
    close = owner,
  )]
  pub journal_entry: Account<'info, JournalEntryState>,
  
  #[account(
    mut
  )]
  pub owner: Signer<'info>,

  pub system_program: Program<'info, System>,
}


#[derive(Accounts)]
#[instruction(title: String, message: String)]
pub struct CreateEntry<'info> {
  #[account(
    init,
    payer = owner,
    space = 8 + JournalEntryState::INIT_SPACE,
    seeds = [title.as_bytes(), owner.key().as_ref()],
    bump
  )]
  pub journal_entry: Account<'info, JournalEntryState>,

  #[account(
    mut
  )]
  pub owner: Signer<'info>,

  pub system_program: Program<'info, System>,
}


#[derive(Accounts)]
#[instruction(title: String, message: String)]
pub struct UpdateEntry<'info> {
  #[account(
    mut,
    seeds = [title.as_bytes(), owner.key().as_ref()],
    bump,
    realloc = 8 + JournalEntryState::INIT_SPACE,
    realloc::payer = owner,
    realloc::zero = true
  )]
  pub journal_entry: Account<'info, JournalEntryState>,

  #[account(
    mut
  )]
  pub owner: Signer<'info>,

  pub system_program: Program<'info, System>,
}


#[account]
#[derive(InitSpace)]
pub struct JournalEntryState {
  pub owner: Pubkey,
  #[max_len(56)]
  pub title: String,

  #[max_len(1180)]
  pub message: String

}