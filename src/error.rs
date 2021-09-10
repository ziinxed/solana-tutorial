use thiserror::Error;

use solana_program::program_error::ProgramError;


#[derive(Error, Debug, Copy, Clone)]
pub enum EscrowError {
    /// invalid instruction
    /// #[error("Invalid Instructions:)]
    InvalidInstruction
}

impl From<EscrowError> for programError {
    fn from (e: EscrowError) -> Self {
        ProgramError::Custom(e as u32)
    }
}

///entrypoint returns a Result of either nothing or a ProgramError.
