// mod error;
// inside error.rs
use thiserror::Error;

use solana_program::program_error::ProgramError;


// #[error_code]
#[derive(Error, Debug, Copy, Clone)]
pub enum InstructionError {
    /// Invalid instruction
    #[error("Invalid Instruction")]
    InvalidInstruction,
    #[error("NotRentExempt")]
    NotRentExempt,
}

impl From<InstructionError> for ProgramError {
    fn from(e: InstructionError) -> Self {
        ProgramError::Custom(e as u32)
    }
}