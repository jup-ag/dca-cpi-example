use anchor_lang::prelude::*;
use instructions::*;

declare_id!("5mrhiqFFXyfJMzAJc5vsEQ4cABRhfsP7MgSVgGQjfcrR");

pub mod instructions;

#[program]
pub mod dca_integration {
    use super::*;

    pub fn open_dca(
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
        instructions::setup_dca(
            ctx,
            application_idx,
            in_amount,
            in_amount_per_cycle,
            cycle_frequency,
            min_out_amount,
            max_out_amount,
            start_at,
            close_wsol_in_ata,
        )
    }
}
