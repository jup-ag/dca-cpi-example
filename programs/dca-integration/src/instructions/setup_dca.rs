use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};
use jupiter_dca::cpi::{self};

#[derive(Accounts)]
#[instruction(application_idx: u64)]
pub struct SetupDca<'info> {
    dca: UncheckedAccount<'info>,
    input_mint: Account<'info, Mint>,
    output_mint: Account<'info, Mint>,
    dca_pda: UncheckedAccount<'info>,
    dca_in_ata: UncheckedAccount<'info>,
    dca_out_ata: UncheckedAccount<'info>,
    dca_event_authority: UncheckedAccount<'info>,

    #[account(mut)]
    user: Signer<'info>,

    #[account(
        mut,
        token::authority=user,
        token::mint=input_mint,
    )]
    user_token_account: Account<'info, TokenAccount>,

    system_program: Program<'info, System>,
    token_program: Program<'info, Token>,
    associated_token_program: Program<'info, AssociatedToken>,
}

pub fn setup_dca(
    ctx: Context<SetupDca>,
    application_idx: u64,
    in_amount: u64,
    in_amount_per_cycle: u64,
    cycle_frequency: i64,
    min_out_amount: Option<u64>,
    max_out_amount: Option<u64>,
    start_at: Option<i64>,
    close_wsol_in_ata: Option<bool>,
) -> Result<()> {
    let open_dca_accounts = cpi::accounts::OpenDca {
        dca: ctx.accounts.dca_pda.to_account_info(),
        user: ctx.accounts.user.to_account_info(),
        input_mint: ctx.accounts.input_mint.to_account_info(),
        output_mint: ctx.accounts.output_mint.to_account_info(),
        user_ata: ctx.accounts.user_token_account.to_account_info(),
        in_ata: ctx.accounts.dca_in_ata.to_account_info(),
        out_ata: ctx.accounts.dca_out_ata.to_account_info(),
        event_authority: ctx.accounts.dca_event_authority.to_account_info(),
        program: ctx.accounts.dca.to_account_info(),
        system_program: ctx.accounts.system_program.to_account_info(),
        token_program: ctx.accounts.token_program.to_account_info(),
        associated_token_program: ctx.accounts.associated_token_program.to_account_info(),
    };

    let cpi_ctx = CpiContext::new(ctx.accounts.dca.to_account_info(), open_dca_accounts);

    cpi::open_dca(
        cpi_ctx,
        application_idx,
        in_amount,
        in_amount_per_cycle,
        cycle_frequency,
        min_out_amount,
        max_out_amount,
        start_at,
        close_wsol_in_ata,
    )?;

    Ok(())
}
