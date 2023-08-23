use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Pda {
    pub idx: u64,
    pub user: Pubkey,
    pub input_mint: Pubkey,
    pub output_mint: Pubkey,
    pub bump: u8,
}

impl Pda {
    pub const LEN: usize = 8 + Self::INIT_SPACE;
}

#[macro_export]
macro_rules! pda_seeds {
    ( $pda:expr, $idx_bytes:expr ) => {
        &[
          PDA_SEED,
          $pda.user.as_ref(),
          $pda.input_mint.as_ref(),
          $pda.output_mint.as_ref(),
          $idx_bytes.as_ref(),
          &[$pda.bump]
          ]
    };
}
