use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Escrow {
    pub idx: u64,
    pub user: Pubkey,
    pub dca: Pubkey,
    pub input_mint: Pubkey,
    pub output_mint: Pubkey,
    pub input_amount: u64,
    pub output_amount: u64,
    pub airdrop_amount: u64,
    pub completed: bool,
    pub airdropped: bool,
    pub bump: u8,
}

impl Escrow {
    pub const LEN: usize = 8 + Self::INIT_SPACE;
}

#[macro_export]
macro_rules! escrow_seeds {
    ( $escrow:expr, $idx_bytes:expr ) => {
        &[
            ESCROW_SEED,
            $escrow.user.as_ref(),
            $escrow.input_mint.as_ref(),
            $escrow.output_mint.as_ref(),
            $idx_bytes.as_ref(),
            &[$escrow.bump],
        ]
    };
}
