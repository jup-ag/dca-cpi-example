use anchor_lang::prelude::*;
use instructions::*;
pub mod math;

declare_id!("BoDCAjKTzVkunw5xx5r3EPWqe3uyNABJJjSRCJNoRmZa");

pub mod constants;
pub mod errors;
pub mod instructions;
pub mod state;

#[program]
pub mod dca_integration {
    use super::*;

    pub fn setup_dca(
        ctx: Context<SetupDca>,
        application_idx: u64,
        in_amount: u64,
        in_amount_per_cycle: u64,
        cycle_frequency: i64,
        plan_duration_seconds: u32,
    ) -> Result<()> {
        instructions::setup_dca(
            ctx,
            application_idx,
            in_amount,
            in_amount_per_cycle,
            cycle_frequency,
            plan_duration_seconds,
        )
    }

    pub fn close(ctx: Context<Close>) -> Result<()> {
        instructions::close(ctx)
    }

    pub fn airdrop(ctx: Context<Airdrop>) -> Result<()> {
        instructions::airdrop(ctx)
    }

    pub fn create_vault(ctx: Context<CreateVault>) -> Result<()> {
        instructions::create_vault(ctx)
    }
}
