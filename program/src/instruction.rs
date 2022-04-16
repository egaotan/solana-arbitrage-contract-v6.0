use solana_program::program_error::ProgramError;
use crate::error::ProtocolError;

pub enum ArbitrageInstruction {
    ///
    ///
    /// 
    ExchangeV1(),
    ///
    /// 
    /// 
    ExchangeV2(),
}

impl ArbitrageInstruction {
    /// Unpacks a byte buffer into a [EscrowInstruction](enum.EscrowInstruction.html).
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (tag, rest) = input.split_first().ok_or(ArbitrageError::InvalidInstruction)?;
        Ok(match tag {
            1..=100 => Self::ExchangeV1(),
            101..=200 => Self::ExchangeV2(),
            _ => return Err(ArbitrageError::InvalidInstruction.into()),
        })
    }
}

