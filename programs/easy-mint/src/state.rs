use anchor_lang::prelude::*;

#[account]
pub struct MintDefinition {
  pub bump: u8,
  pub owner: Pubkey,
  pub pay_to_account: Pubkey,
  pub price_mint: [Pubkey; 5],
  pub price: [u64; 5],
  pub expiration_date: u64,
  pub memorable_word: String,
  pub reserved: [u64; 16],
}

impl MintDefinition {
  pub const SPACE: usize = 1 + 32 + 32 + (32 * 5) + (8 * 5) + 8 + 25 + (8 * 16);
}