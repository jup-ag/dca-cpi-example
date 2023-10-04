use crate::{errors::EscrowErrors, state::Escrow};
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount, Transfer},
};

#[derive(Accounts)]
pub struct Airdrop<'info> {
    #[account(mut)]
    admin: Signer<'info>,

    /// CHECK: Not mutating
    user: UncheckedAccount<'info>,

    #[account(
      mut,
      constraint=escrow.user==user.key(),
    )]
    escrow: Box<Account<'info, Escrow>>,

    #[account(
      address=escrow.output_mint
    )]
    output_mint: Box<Account<'info, Mint>>,

    #[account(
      mut,
      associated_token::authority=user,
      associated_token::mint=output_mint,
    )]
    admin_token_account: Box<Account<'info, TokenAccount>>,

    #[account(
      init_if_needed,
      payer=admin,
      associated_token::authority=user,
      associated_token::mint=output_mint,
    )]
    user_token_account: Box<Account<'info, TokenAccount>>,

    system_program: Program<'info, System>,
    token_program: Program<'info, Token>,
    associated_token_program: Program<'info, AssociatedToken>,
}

// user has to claim (close ix) before airdropping
pub fn airdrop(ctx: Context<Airdrop>) -> Result<()> {
    require_eq!(
        ctx.accounts.escrow.completed,
        true,
        EscrowErrors::DCANotComplete
    );
    require_eq!(
        ctx.accounts.escrow.airdropped,
        false,
        EscrowErrors::Airdropped
    );
    require_gt!(
        ctx.accounts.escrow.airdrop_amount,
        0,
        EscrowErrors::UnexpectedAirdropAmount
    );
    require_gte!(
        ctx.accounts.admin_token_account.amount,
        ctx.accounts.escrow.airdrop_amount,
        EscrowErrors::InsufficientBalance
    );

    msg!("Recording airdrop");
    let escrow = &mut ctx.accounts.escrow;
    escrow.airdropped = true;

    msg!("Transferring airdrop");
    anchor_spl::token::transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.admin_token_account.to_account_info(),
                to: ctx.accounts.user_token_account.to_account_info(),
                authority: ctx.accounts.admin.to_account_info(),
            },
        ),
        ctx.accounts.escrow.airdrop_amount,
    )?;

    Ok(())
}
