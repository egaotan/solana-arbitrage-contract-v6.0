use solana_program::program_error::ProgramError;
use crate::error::ArbitrageError;
use arrayref::{array_ref, array_refs};
use serum_dex::matching::Side;

/// market
#[derive(Clone, Debug, PartialEq, Copy)]
pub enum Market {
    Orca,
    Saber,
    Serum,
    Raydium,
    Mercurial,
}

impl Market {
    pub fn from(value: u8) -> Option<Self> {
        match value {
            0 => Some(Market::Orca),
            1 => Some(Market::Saber),
            2 => Some(Market::Serum),
            3 => Some(Market::Raydium),
            4 => Some(Market::Mercurial),
            _ => None,
        }
    }
}

fn find_side(value: u8) -> Option<Side> {
    match value {
        0 => Some(Side::Bid),
        1 => Some(Side::Ask),
        _ => None,
    }
}

pub struct ExchangeWithPathInstruction {
    pub flag: u8,
    pub amount: u64,
    pub market: Market,
    pub side: Side,
}

impl ExchangeWithPathInstruction {
    const DATA_LEN: usize = 11;
    fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        if input.len() < ExchangeWithPathInstruction::DATA_LEN {
            return Err(ArbitrageError::InvalidInstruction.into());
        }
        let arr_data = array_ref![input, 0, ExchangeWithPathInstruction::DATA_LEN];
        let (
            &[flag],
            &amount_arr,
            &[market],
            &[side],
        ) = array_refs![arr_data, 1, 8, 1, 1];
        Ok(
            ExchangeWithPathInstruction {
                flag: flag,
                amount: u64::from_le_bytes(amount_arr),
                market: Market::from(market).ok_or(ProgramError::InvalidInstructionData)?,
                side: find_side(side).ok_or(ProgramError::InvalidInstructionData)?,
            }
        )
    }
}

pub struct ExchangeStableInstruction {
    pub flag: u8,
}

impl ExchangeStableInstruction {
    const DATA_LEN: usize = 1;
    fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        if input.len() < ExchangeStableInstruction::DATA_LEN {
            return Err(ArbitrageError::InvalidInstruction.into());
        }
        let arr_data = array_ref![input, 0, ExchangeStableInstruction::DATA_LEN];
        /*
        let (
            &[flag],
        ) = array_refs![arr_data, 1];
        */
        Ok(
            ExchangeStableInstruction {
                flag: arr_data[0],
            }
        )
    }
}

pub enum ArbitrageInstruction {
    ///
    ///
    /// 
    Exchange_NonStable_All(ExchangeStableInstruction),
    ///
    /// 
    /// 
    Exchange_NonStable_Serum(),
    ///
    /// 
    /// 
    Exchange_WithPath(ExchangeWithPathInstruction),
    ///
    /// 
    ///
    Exchange_Stable1(ExchangeStableInstruction),
    ///
    ///
    /// 
    Exchange_Stable2(ExchangeStableInstruction),
    ///
    /// 
    ///
    Exchange_Stable3(ExchangeStableInstruction),
    ///
    ///
    /// 
    Exchange_Stable4(ExchangeStableInstruction),
    ///
    /// 
    ///
    Exchange_Stable5(ExchangeStableInstruction),
    ///
    ///
    /// 
    Exchange_Stable6(ExchangeStableInstruction),
    ///
    /// 
    ///
    Exchange_Stable7(ExchangeStableInstruction),
    ///
    ///
    /// 
    Exchange_Stable8(ExchangeStableInstruction),  
        ///
    /// 
    ///
    Exchange_Stable9(ExchangeStableInstruction),
    ///
    ///
    /// 
    Exchange_Stable10(ExchangeStableInstruction), 
        ///
    /// 
    ///
    Exchange_Stable11(ExchangeStableInstruction),
    ///
    ///
    /// 
    Exchange_Stable12(ExchangeStableInstruction),       
}

impl ArbitrageInstruction {
    /// Unpacks a byte buffer into a [EscrowInstruction](enum.EscrowInstruction.html).
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (tag, rest) = input.split_first().ok_or(ArbitrageError::InvalidInstruction)?;
        Ok(match tag {
            0..=10 => Self::Exchange_NonStable_All(ExchangeStableInstruction::unpack(rest)?),
            10..=20 => Self::Exchange_NonStable_Serum(),
            20..=30 => Self::Exchange_WithPath(ExchangeWithPathInstruction::unpack(rest)?),
            30..=40 => Self::Exchange_Stable1(ExchangeStableInstruction::unpack(rest)?),
            40..=50 => Self::Exchange_Stable2(ExchangeStableInstruction::unpack(rest)?),
            50..=60 => Self::Exchange_Stable3(ExchangeStableInstruction::unpack(rest)?),
            60..=70 => Self::Exchange_Stable4(ExchangeStableInstruction::unpack(rest)?),
            70..=80 => Self::Exchange_Stable5(ExchangeStableInstruction::unpack(rest)?),
            80..=90 => Self::Exchange_Stable6(ExchangeStableInstruction::unpack(rest)?),
            90..=100 => Self::Exchange_Stable7(ExchangeStableInstruction::unpack(rest)?),
            100..=110 => Self::Exchange_Stable8(ExchangeStableInstruction::unpack(rest)?),
            110..=120 => Self::Exchange_Stable9(ExchangeStableInstruction::unpack(rest)?),
            120..=130 => Self::Exchange_Stable10(ExchangeStableInstruction::unpack(rest)?),
            130..=140 => Self::Exchange_Stable11(ExchangeStableInstruction::unpack(rest)?),
            140..=150 => Self::Exchange_Stable12(ExchangeStableInstruction::unpack(rest)?),
            _ => return Err(ArbitrageError::InvalidInstruction.into()),
        })
    }
}

