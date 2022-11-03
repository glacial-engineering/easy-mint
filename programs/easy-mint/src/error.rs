use anchor_lang::prelude::*;

#[error_code]
pub enum EasyMintErrorCode {
  #[msg("Invalid price mint")]
  InvalidPriceMint,
  #[msg("Maximum number of mint pricing reached")]
  MaxMintPrices,
}
