use anchor_lang::prelude::*;

#[error_code]
pub enum EscrowErrors {
    #[msg("DCA Account not yet closed")]
    DCANotClosed,

    #[msg("DCA Not Complete")]
    DCANotComplete,

    #[msg("Already airdropped")]
    Airdropped,

    #[msg("Unexpected Balance")]
    UnexpectedBalance,

    #[msg("Insufficient Balance")]
    InsufficientBalance,
}
