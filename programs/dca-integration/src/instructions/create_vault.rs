use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};

use crate::constants::*;

#[derive(Accounts)]
pub struct CreateVault<'info> {
    #[account(mut)]
    signer: Signer<'info>,

    /// CHECK: hardcoded authority
    #[account(
        seeds = [b"vault"],
        bump
    )]
    vault_signer: AccountInfo<'info>,

    #[account()]
    mint: Box<Account<'info, Mint>>,

    #[account(
        init,
        payer=signer,
        associated_token::authority=vault_signer,
        associated_token::mint=mint,
    )]
    vault: Box<Account<'info, TokenAccount>>,

    system_program: Program<'info, System>,
    token_program: Program<'info, Token>,
    associated_token_program: Program<'info, AssociatedToken>,
}

pub fn create_vault(ctx: Context<CreateVault>) -> Result<()> {
    // authority can only call this instruction
    require_keys_eq!(
        ctx.accounts.signer.key(),
        Pubkey::new_from_array(AUTHORITY_PK)
    );

    // BONK only
    require_keys_eq!(ctx.accounts.mint.key(), Pubkey::new_from_array(BONK_MINT));

    // make sure we've calculated the right address
    require_keys_eq!(
        ctx.accounts.vault_signer.key(),
        Pubkey::new_from_array(VAULT_SIGNER_PDA)
    );

    Ok(())
}
