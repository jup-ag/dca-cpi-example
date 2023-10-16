use crate::constants::{ACCEPTED_DURATION_SECONDS, AIRDROP_BPS, ESCROW_SEED};
use crate::{errors::EscrowErrors, escrow_seeds, math, state::Escrow};
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

    /// CHECK: hardcoded authority
    #[account(
        address = Pubkey::new_from_array(crate::constants::VAULT_SIGNER_PDA),
        seeds = [b"vault"],
        bump
    )]
    vault_signer: AccountInfo<'info>,

    #[account(
        mut,
        associated_token::authority=vault_signer,
        associated_token::mint=output_mint,
    )]
    vault: Box<Account<'info, TokenAccount>>,

    system_program: Program<'info, System>,
    token_program: Program<'info, Token>,
    associated_token_program: Program<'info, AssociatedToken>,
}

impl<'info> Close<'info> {
    pub fn compute_airdrop_amount(plan_bps: u64, out_amount: u64) -> Result<u64> {
        let u128_amount = math::checked_div(
            math::checked_mul(out_amount as u128, plan_bps as u128)?,
            10000,
        )?;
        let u64_amount: u64 = math::checked_as_u64(u128_amount)?;
        msg!(
            "airdrop out_amount={}, plan_bps={}, result={}",
            out_amount,
            plan_bps,
            u64_amount
        );
        Ok(u64_amount)
    }
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

    // plus airdrop rewards % from the respective plan duration
    let matched_plan = ACCEPTED_DURATION_SECONDS
        .iter()
        .enumerate()
        .find(|(_idx, &d)| d.eq(&escrow.plan_duration_seconds));

    let plan_idx = match matched_plan {
        Some((plan_idx, _duration)) => {
            msg!(
                "using plan={} from duration={}",
                plan_idx,
                escrow.plan_duration_seconds
            );
            Ok::<usize, anchor_lang::prelude::ErrorCode>(plan_idx)
        }
        None => return Err(error!(EscrowErrors::InvalidPlanParameters)),
    }?;

    let airdrop_bps = AIRDROP_BPS[plan_idx];
    let rewards_bonus =
        Close::compute_airdrop_amount(airdrop_bps, ctx.accounts.escrow_out_ata.amount)?;
    escrow.output_amount = ctx.accounts.escrow_out_ata.amount; // will this work for native SOL?
    escrow.completed = true;
    escrow.airdrop_amount = rewards_bonus;

    msg!(
        "rewards={}, out_mint={}, balance={}",
        rewards_bonus,
        ctx.accounts.output_mint.key(),
        ctx.accounts.vault.amount,
    );

    let rewards_seeds: &[&[&[u8]]] = &[&[b"vault", &[*ctx.bumps.get("vault_signer").unwrap()]]];

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

        // transfer rewards from the vault
        anchor_spl::token::transfer(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                Transfer {
                    from: ctx.accounts.vault.to_account_info(),
                    to: ctx.accounts.user_token_account.to_account_info(),
                    authority: ctx.accounts.vault_signer.to_account_info(),
                },
                rewards_seeds,
            ),
            rewards_bonus,
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
