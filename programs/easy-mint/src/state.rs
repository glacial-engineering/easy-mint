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

  //new fields
  pub supply_limit: Option<u32>, //5 bytes

  //was originaly [u64; 16]
  pub reserved: [u64; 15], //8 * 15 bytes
  pub reserved2: u16, //2 bytes
  pub reserved3: u8, //1 byte
}

impl MintDefinition {
  pub const SPACE: usize = 1 + 32 + 32 + (32 * 5) + (8 * 5) + 8 + 25 + (8 * 16);
}