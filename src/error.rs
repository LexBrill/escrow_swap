// inside error.rs
use thiserror::Error;

use solana_program::program_error::ProgramError;


#[derive(Error, Debug, Copy, Clone)]
pub enum EscrowError {
    /// Invalid instruction
    #[error("Invalid Instruction")]
    InvalidInstruction,
    #[error("Not Enough Remaining")]
    NotEnoughRemaining,
    #[error("Amount Overflow")]
    AmountOverflow,
    #[error("SPL Input Not Accepted")]
    WrongInput,
    #[error("Not the same person who deposited")]
    NotDepositor,
}

impl From<EscrowError> for ProgramError {
    fn from(e: EscrowError) -> Self {
        ProgramError::Custom(e as u32)
    }
}