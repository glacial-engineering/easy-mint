use anchor_lang::prelude::*;
use anchor_lang::solana_program::program::{invoke_signed, invoke};
use anchor_spl::token::{mint_to, transfer, MintTo, Transfer};

use mpl_token_metadata::instruction::create_metadata_accounts_v3;
use mpl_token_metadata::state::Creator;

mod error;
mod ix_accounts;
mod state;

use ix_accounts::*;

declare_id!("ezMY4T9fFpdqHTGXn36TA5RBSZRi4Dr7GBEP7AqSWPQ");

#[program]
pub mod easy_mint {
    use mpl_token_metadata::instruction::sign_metadata;

    use super::*;

    pub fn create_mint_definition(
        ctx: Context<CreateMintDefinition>,
        memorable_word: String,
        price_mint: Pubkey,
        price: u64,
        mint_name: String,
        mint_symbol: String,
        mint_uri: String,
        basis_points: u16,
    ) -> Result<()> {
        //set our mint defintion
        let md = &mut ctx.accounts.mint_definition;
        md.bump = ctx.bumps["mint_definition"];
        md.owner = ctx.accounts.owner.key();
        md.price_mint = price_mint;
        md.price = price;
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
        invoke(
            &ix,
            &accounts,
        )?;

        Ok(())
    }

    pub fn update_mint_price(
        ctx: Context<UpdateMintDefinition>,
        price_mint: Pubkey,
        price: u64,
    ) -> Result<()> {
        let md = &mut ctx.accounts.mint_definition;
        md.price_mint = price_mint;
        md.price = price;

        Ok(())
    }

    pub fn please_mint_token(ctx: Context<PleaseMintToken>) -> Result<()> {
        if ctx.accounts.mint_definition.price > 0 {
            let cpi = CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                Transfer {
                    from: ctx.accounts.pay_from_token_acct.to_account_info(),
                    to: ctx
                        .accounts
                        .payment_mint_definition_owner_token_acct
                        .to_account_info(),
                    authority: ctx.accounts.payer.to_account_info(),
                },
            );
            transfer(cpi, ctx.accounts.mint_definition.price)?;
        }

        let mint_def_key = ctx.accounts.mint_definition.key();
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

        Ok(())
    }
}
