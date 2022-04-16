use thiserror::Error;
use solana_program::{
    msg,
    program_error::{ProgramError},
};

#[derive(Error, Debug, Clone)]
pub enum ArbitrageError {
  #[error(transparent)]
  ProgramError(#[from] ProgramError),
  /// Invalid instruction
  #[error("Invalid Instruction")]
  InvalidInstruction,
  /// Not Rent Exempt
  #[error("Invalid Call")]
  InvalidCall,
  /// Not Rent Exempt
  #[error("Invalid Onwer")]
  InvalidOwner,
  /// Expected Amount Mismatch
  #[error("swap out amount is smaller than in amount")]
  OutAmountSmallerThanInAmount,
}

impl From<ArbitrageError> for ProgramError {
  fn from(e: ArbitrageError) -> ProgramError {
    match e {
      ArbitrageError::ProgramError(pe) => pe,
      ArbitrageError::InvalidInstruction => {
        ProgramError::Custom(10000)
      }
      ArbitrageError::InvalidCall => {
        ProgramError::Custom(10001)
      }
      ArbitrageError::InvalidOwner => {
        ProgramError::Custom(10002)
      }
      ArbitrageError::OutAmountSmallerThanInAmount => {
        ProgramError::Custom(10003)
      }
    }
  }
}

pub type ArbitrageResult<T = ()> = Result<T, ArbitrageError>;
