use anchor_lang::{Accounts, prelude::*, solana_program::clock::Clock};
use anchor_spl::{token::{Token, TokenAccount, Mint}, associated_token::AssociatedToken};

use crate::state::MintDefinition;

#[derive(Accounts)]
#[instruction(memorable_word: String)]
pub struct CreateMintDefinition<'info> {
    #[account(
      mut,
    )]
    pub owner: Signer<'info>,

    #[account(
      mut,
      seeds = [ "fee".as_bytes() ],
      bump,
    )]
    /// CHECK: just used to send SOL to
    pub program_fee: UncheckedAccount<'info>,

    #[account(
      init,
      seeds = [ memorable_word.as_bytes(), owner.key().as_ref() ],
      bump,
      payer = owner,
      space = MintDefinition::SPACE + 8,
    )]
    pub mint_definition: Box<Account<'info, MintDefinition>>,
    
    #[account(
      init,
      seeds = [ mint_definition.key().as_ref() ],
      bump,
      payer = owner,
      mint::authority = mint,
      mint::decimals = 0,
    )]
    pub mint: Box<Account<'info, Mint>>,

    /// CHECK: checked via the Metadata CPI call
    #[account(mut)]
    pub mint_metadata_account: UncheckedAccount<'info>,

    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,

    /// CHECK: checked via account constraints
    #[account(
        address = mpl_token_metadata::ID
    )]
    pub metadata_program: UncheckedAccount<'info>,
}

#[derive(Accounts)]
pub struct PleaseMintToken<'info> {
    #[account(
      mut,
    )]
    pub payer: Signer<'info>,

    #[account(
      mut,
      seeds = [ "fee".as_bytes() ],
      bump,
    )]
    /// CHECK: just used to send SOL to
    pub program_fee: UncheckedAccount<'info>,

    #[account(
      has_one = pay_to_account,
      constraint = Clock::get().unwrap().unix_timestamp < mint_definition.expiration_date.try_into().unwrap(),
    )]
    pub mint_definition: Box<Account<'info, MintDefinition>>,
    
    #[account(
      constraint = pay_with_mint.key() != Pubkey::default(),
    )]
    pub pay_with_mint: Box<Account<'info, Mint>>,

    #[account(
      mut,
      constraint = pay_from_token_acct.mint == pay_with_mint.key(),
    )]
    pub pay_from_token_acct: Box<Account<'info, TokenAccount>>,

    /// CHECK: just used for ATA below
    pub pay_to_account: UncheckedAccount<'info>,

    #[account(
      init_if_needed,
      payer = payer,
      associated_token::mint = pay_with_mint,
      associated_token::authority = pay_to_account,
    )]
    pub pay_to_token_acct: Box<Account<'info, TokenAccount>>,
    
    /// CHECK: we just use for ATA of delivery
    pub recipient_wallet: UncheckedAccount<'info>,

    #[account(
      mut,
      seeds = [ mint_definition.key().as_ref() ],
      bump,
    )]
    pub mint: Box<Account<'info, Mint>>,

    #[account(
      init_if_needed,
      payer = payer,
      associated_token::mint = mint,
      associated_token::authority = recipient_wallet,
    )]
    pub delivery_token_acct: Box<Account<'info, TokenAccount>>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateMintDefinition<'info> {
    #[account(
      mut,
    )]
    pub owner: Signer<'info>,

    #[account(
      mut,
      has_one = owner,
    )]
    pub mint_definition: Box<Account<'info, MintDefinition>>,
}