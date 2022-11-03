use anchor_lang::prelude::*;
use anchor_lang::solana_program::program::{invoke, invoke_signed};
use anchor_lang::solana_program::system_instruction;
use anchor_spl::token::{mint_to, transfer, MintTo, Transfer};

use mpl_token_metadata::instruction::create_metadata_accounts_v3;
use mpl_token_metadata::instruction::sign_metadata;
use mpl_token_metadata::state::Creator;

mod error;
mod ix_accounts;
mod state;

use ix_accounts::*;

declare_id!("ezMY4T9fFpdqHTGXn36TA5RBSZRi4Dr7GBEP7AqSWPQ");

const PROGRAM_FEE_CREATE_DEF: u64 = 0;//1000000000;
const PROGRAM_FEE_MINT: u64 = 0;//10000000;

#[program]
pub mod easy_mint {

    use super::*;

    pub fn create_mint_definition(
        ctx: Context<CreateMintDefinition>,
        memorable_word: String,
        pay_to_account: Pubkey,
        price_mint: Pubkey,
        price: u64,
        expiration_date: u64,
        mint_name: String,
        mint_symbol: String,
        mint_uri: String,
        basis_points: u16,
    ) -> Result<()> {
        //set our mint defintion
        let md = &mut ctx.accounts.mint_definition;
        md.bump = ctx.bumps["mint_definition"];
        md.owner = ctx.accounts.owner.key();
        md.pay_to_account = pay_to_account;
        md.price_mint[0] = price_mint;
        md.price[0] = price;
        md.expiration_date = expiration_date;
        md.memorable_word = memorable_word;

        //create the metaplex metadata
        let creators = vec![
            Creator {
                address: ctx.accounts.mint.key(),
                share: 0,
                verified: true,
            },
            Creator {
                address: md.key(),
                share: 0,
                verified: false,
            },
            Creator {
                address: ctx.accounts.owner.key(),
                share: 100,
                verified: false,
            },
        ];

        let ix = create_metadata_accounts_v3(
            ctx.accounts.metadata_program.key(),
            ctx.accounts.mint_metadata_account.key(),
            ctx.accounts.mint.key(),
            ctx.accounts.mint.key(), //mint auth
            ctx.accounts.owner.key(),
            ctx.accounts.mint.key(), //update auth
            mint_name,
            mint_symbol,
            mint_uri,
            Some(creators),
            basis_points,
            true,
            true,
            None,
            None,
            None,
        );
        let accounts = [
            ctx.accounts.mint_metadata_account.to_account_info(),
            ctx.accounts.mint.to_account_info(),
            ctx.accounts.owner.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
            ctx.accounts.rent.to_account_info(),
        ];
        invoke_signed(
            &ix,
            &accounts,
            &[&[md.key().as_ref(), &[ctx.bumps["mint"]]]],
        )?;

        //mint def sign
        let ix = sign_metadata(
            ctx.accounts.metadata_program.key(),
            ctx.accounts.mint_metadata_account.key(),
            md.key(),
        );
        let accounts = [
            ctx.accounts.mint_metadata_account.to_account_info(),
            md.to_account_info(),
        ];
        invoke_signed(
            &ix,
            &accounts,
            &[&[md.memorable_word.as_bytes(), md.owner.as_ref(), &[md.bump]]],
        )?;

        //owner sign
        let ix = sign_metadata(
            ctx.accounts.metadata_program.key(),
            ctx.accounts.mint_metadata_account.key(),
            ctx.accounts.owner.key(),
        );
        let accounts = [
            ctx.accounts.mint_metadata_account.to_account_info(),
            ctx.accounts.owner.to_account_info(),
        ];
        invoke(&ix, &accounts)?;

        //take the fee
        if PROGRAM_FEE_CREATE_DEF > 0 {
            let ix = system_instruction::transfer(
                ctx.accounts.owner.key,
                ctx.accounts.program_fee.key,
                PROGRAM_FEE_CREATE_DEF,
            );
            let accounts = [
                ctx.accounts.owner.to_account_info(),
                ctx.accounts.program_fee.to_account_info(),
            ];
            invoke(&ix, &accounts)?;
        }

        Ok(())
    }

    pub fn update_mint_price(
        ctx: Context<UpdateMintDefinition>,
        price_mint: Pubkey,
        price: u64,
    ) -> Result<()> {
        let md = &mut ctx.accounts.mint_definition;
        if let Some(idx) = md.price_mint.iter().position(|a|a == &price_mint) {
            if price == u64::MAX {
                //max u64 is a flag to delete
                md.price[idx] = 0;
                md.price_mint[idx] = Pubkey::default();
            } else {
                //update existing
                md.price[idx] = price;
            }
            Ok(())
        } else if let Some(idx) = md.price_mint.iter().position(|a|a == &Pubkey::default()) {
            //a new mint
            md.price_mint[idx] = price_mint;
            md.price[idx] = price;
            Ok(())
        } else {
            //no more room for a price
            Err(error!(error::EasyMintErrorCode::MaxMintPrices))
        }
    }

    pub fn update_mint_expiry_date(
        ctx: Context<UpdateMintDefinition>,
        expiration_date: u64,
    ) -> Result<()> {
        let md = &mut ctx.accounts.mint_definition;
        md.expiration_date = expiration_date;

        Ok(())
    }

    pub fn please_mint_token(ctx: Context<PleaseMintToken>) -> Result<()> {
        let md = &ctx.accounts.mint_definition;
        let price_mint = ctx.accounts.pay_from_token_acct.mint;
        
        let price: u64;
        if let Some(idx) = md.price_mint.iter().position(|a|a == &price_mint) {
            price = md.price[idx];
        } else {
            //price doesn't exist for this mint
            return Err(error!(error::EasyMintErrorCode::InvalidPriceMint))
        }

        if price > 0 {
            let cpi = CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                Transfer {
                    from: ctx.accounts.pay_from_token_acct.to_account_info(),
                    to: ctx
                        .accounts
                        .pay_to_token_acct
                        .to_account_info(),
                    authority: ctx.accounts.payer.to_account_info(),
                },
            );
            transfer(cpi, price)?;
        }

        let mint_def_key = md.key();
        let signer_seeds = [&[mint_def_key.as_ref(), &[ctx.bumps["mint"]] as &[u8]] as &[&[u8]]];
        let cpi = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            MintTo {
                mint: ctx.accounts.mint.to_account_info(),
                to: ctx.accounts.delivery_token_acct.to_account_info(),
                authority: ctx.accounts.mint.to_account_info(),
            },
        )
        .with_signer(&signer_seeds);
        mint_to(cpi, 1)?;

        //take the fee
        if PROGRAM_FEE_MINT > 0 {
            let ix = system_instruction::transfer(
                ctx.accounts.payer.key,
                ctx.accounts.program_fee.key,
                PROGRAM_FEE_MINT,
            );
            let accounts = [
                ctx.accounts.payer.to_account_info(),
                ctx.accounts.program_fee.to_account_info(),
            ];
            invoke(&ix, &accounts)?;
        }

        Ok(())
    }
}
