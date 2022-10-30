use anchor_lang::{Accounts, prelude::*};
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

    pub mint_definition: Box<Account<'info, MintDefinition>>,
    
    #[account(
      constraint = pay_with_mint.key() == mint_definition.price_mint
    )]
    pub pay_with_mint: Box<Account<'info, Mint>>,

    #[account(
      mut,
      constraint = pay_from_token_acct.mint == mint_definition.price_mint
    )]
    pub pay_from_token_acct: Box<Account<'info, TokenAccount>>,

    #[account(
      address = mint_definition.owner,
    )]
    /// CHECK: just used for ATA below
    pub mint_definition_owner: UncheckedAccount<'info>,

    #[account(
      init_if_needed,
      payer = payer,
      associated_token::mint = pay_with_mint,
      associated_token::authority = mint_definition_owner,
    )]
    pub payment_mint_definition_owner_token_acct: Box<Account<'info, TokenAccount>>,
    
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