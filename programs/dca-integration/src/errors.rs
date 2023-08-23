use anchor_lang::prelude::*;

#[error_code]
pub enum EscrowErrors {
    #[msg("DCA Account not yet closed")]
    DCANotClosed,

    #[msg("Unexpected Balance")]
    UnexpectedBalance,
}
