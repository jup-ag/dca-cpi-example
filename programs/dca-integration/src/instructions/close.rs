use crate::constants::ESCROW_SEED;
use crate::{errors::EscrowErrors, escrow_seeds, state::Escrow};
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount, Transfer},
};

#[derive(Accounts)]
pub struct Close<'info> {
    #[account(
      address=escrow.input_mint
    )]
    input_mint: Box<Account<'info, Mint>>,

    #[account(
      address=escrow.output_mint
    )]
    output_mint: Box<Account<'info, Mint>>,

    #[account(mut)]
    user: Signer<'info>,

    #[account(
      init_if_needed,
      payer=user,
      associated_token::authority=user,
      associated_token::mint=output_mint,
    )]
    user_token_account: Box<Account<'info, TokenAccount>>,

    #[account(
      mut,
      constraint=escrow.user==user.key(),
    )]
    escrow: Box<Account<'info, Escrow>>,

    #[account(
      mut,
      associated_token::authority=escrow,
      associated_token::mint=input_mint,
    )]
    escrow_in_ata: Box<Account<'info, TokenAccount>>,

    /// CHECK: Not mutating and checks that it belongs to this user
    #[account(
      address=escrow.dca
    )]
    dca: UncheckedAccount<'info>,

    #[account(
      mut,
      associated_token::authority=escrow,
      associated_token::mint=output_mint,
    )]
    escrow_out_ata: Box<Account<'info, TokenAccount>>,

    system_program: Program<'info, System>,
    token_program: Program<'info, Token>,
    associated_token_program: Program<'info, AssociatedToken>,
}

pub fn close(ctx: Context<Close>) -> Result<()> {
    // Checks that the DCA account is done and closed before closing escrow account
    require_eq!(ctx.accounts.dca.lamports(), 0, EscrowErrors::DCANotClosed);

    require_eq!(
        ctx.accounts.escrow_in_ata.amount,
        0,
        EscrowErrors::UnexpectedBalance
    );

    let escrow = &mut ctx.accounts.escrow;
    escrow.output_amount = ctx.accounts.escrow_out_ata.amount; // will this work for native SOL?
    escrow.completed = true;

    let idx_bytes = ctx.accounts.escrow.idx.to_le_bytes();
    let signer_seeds: &[&[&[u8]]] = &[escrow_seeds!(ctx.accounts.escrow, idx_bytes)];

    // transfer out tokens to user
    // if it's native SOL, it will get returned to user when the account is closed
    if ctx.accounts.escrow_out_ata.amount > 0 {
        anchor_spl::token::transfer(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                Transfer {
                    from: ctx.accounts.escrow_out_ata.to_account_info(),
                    to: ctx.accounts.user_token_account.to_account_info(),
                    authority: ctx.accounts.escrow.to_account_info(),
                },
                signer_seeds,
            ),
            ctx.accounts.escrow_out_ata.amount,
        )?;
    }

    // close ATAs
    anchor_spl::token::close_account(CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        anchor_spl::token::CloseAccount {
            account: ctx.accounts.escrow_in_ata.to_account_info(),
            destination: ctx.accounts.user.to_account_info(),
            authority: ctx.accounts.escrow.to_account_info(),
        },
        signer_seeds,
    ))?;

    anchor_spl::token::close_account(CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        anchor_spl::token::CloseAccount {
            account: ctx.accounts.escrow_out_ata.to_account_info(),
            destination: ctx.accounts.user.to_account_info(),
            authority: ctx.accounts.escrow.to_account_info(),
        },
        signer_seeds,
    ))?;

    Ok(())
}
