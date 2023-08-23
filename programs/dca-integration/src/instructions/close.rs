use crate::{pda_seeds, state::Pda};
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount, Transfer},
};
use crate::constants::PDA_SEED;


#[derive(Accounts)]
#[instruction(application_idx: u64)]
pub struct Close<'info> {
    #[account(
      address=pda.input_mint
    )]
    input_mint: Account<'info, Mint>,

    #[account(
      address=pda.output_mint
    )]
    output_mint: Account<'info, Mint>,

    #[account(mut)]
    user: Signer<'info>,

    #[account(
        init_if_needed,
        payer=user,
        token::authority=user,
        token::mint=output_mint,
    )]
    user_token_account: Account<'info, TokenAccount>,

    #[account(
      mut,
      constraint=pda.user==user.key(),
      close=user
    )]
    pda: Account<'info, Pda>,

    #[account(
      mut,
      associated_token::authority=pda,
      associated_token::mint=input_mint
    )]
    pda_in_ata: Account<'info, TokenAccount>,

    #[account(
      mut,
      associated_token::authority=pda,
      associated_token::mint=output_mint
    )]
    pda_out_ata: Account<'info, TokenAccount>,

    system_program: Program<'info, System>,
    token_program: Program<'info, Token>,
    associated_token_program: Program<'info, AssociatedToken>,
}

pub fn close(ctx: Context<Close>) -> Result<()> {
    // WARNING! Do not close before DCA is done; There's no way to retrieve funds.
    // WARNING! Be careful not to use this if DCA-ing into SOL. Example does not handle native SOL transfer if PDA has SOL

    require_eq!(ctx.accounts.pda_in_ata.amount, 0);

    let idx_bytes = ctx.accounts.pda.idx.to_le_bytes();
    let signer_seeds: &[&[&[u8]]] = &[pda_seeds!(ctx.accounts.pda, idx_bytes)];

    // transfer out tokens to user
    if ctx.accounts.pda_out_ata.amount > 0 {
        anchor_spl::token::transfer(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                Transfer {
                    from: ctx.accounts.pda_out_ata.to_account_info(),
                    to: ctx.accounts.user_token_account.to_account_info(),
                    authority: ctx.accounts.pda.to_account_info(),
                },
                signer_seeds,
            ),
            ctx.accounts.pda_out_ata.amount,
        )?;
    }

    // close ATAs
    let cpi_accounts = anchor_spl::token::CloseAccount {
        account: ctx.accounts.pda_in_ata.to_account_info(),
        destination: ctx.accounts.user.to_account_info(),
        authority: ctx.accounts.pda.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);
    anchor_spl::token::close_account(cpi_ctx)?;

    let cpi_accounts = anchor_spl::token::CloseAccount {
        account: ctx.accounts.pda_out_ata.to_account_info(),
        destination: ctx.accounts.user.to_account_info(),
        authority: ctx.accounts.pda.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);
    anchor_spl::token::close_account(cpi_ctx)?;

    Ok(())
}
