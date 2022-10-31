use anchor_lang::prelude::*;

#[account]
pub struct MintDefinition {
  pub bump: u8,
  pub owner: Pubkey,
  pub price_mint: Pubkey,
  pub price: u64,
  pub expiration_date: u64,
  pub memorable_word: String,
  pub reserved: [u64; 16],
}

impl MintDefinition {
  pub const SPACE: usize = 1 + 32 + 32 + 8 + 8 + 25 + (8*16);
}