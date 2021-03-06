use solana_program::program_error::ProgramError;
use thiserror::Error;

#[derive(Debug, Error, Copy, Clone)]
pub enum EscrowError {
    /// Invalid Instruction
    #[error("Invalid Instruction")]
    InvalidInstruction,

    #[error("Not Rent Exempt")]
    NotRentExempt,
}

impl From<EscrowError> for ProgramError {
    fn from(e: EscrowError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
