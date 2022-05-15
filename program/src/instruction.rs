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

pub struct ExchangeWithTryInstruction {
    pub flag: u8,
}

impl ExchangeWithTryInstruction {
    const DATA_LEN: usize = 1;
    fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        if input.len() < ExchangeWithTryInstruction::DATA_LEN {
            return Err(ArbitrageError::InvalidInstruction.into());
        }
        let arr_data = array_ref![input, 0, ExchangeWithTryInstruction::DATA_LEN];
        /*
        let (
            &[flag],
        ) = array_refs![arr_data, 1];
        */
        Ok(
            ExchangeWithTryInstruction {
                flag: arr_data[0],
            }
        )
    }
}

pub enum ArbitrageInstruction {
    ///
    ///
    /// 
    Exchange_NonStable_All(ExchangeWithTryInstruction),
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
    Exchange_Stable1(ExchangeWithTryInstruction),
    ///
    ///
    /// 
    Exchange_Stable2(ExchangeWithTryInstruction),
    ///
    /// 
    ///
    Exchange_Stable3(ExchangeWithTryInstruction),
    ///
    ///
    /// 
    Exchange_Stable4(ExchangeWithTryInstruction),
    ///
    /// 
    ///
    Exchange_Stable5(ExchangeWithTryInstruction),
    ///
    ///
    /// 
    Exchange_Stable6(ExchangeWithTryInstruction),
    ///
    /// 
    ///
    Exchange_Stable7(ExchangeWithTryInstruction),
    ///
    ///
    /// 
    Exchange_Stable8(ExchangeWithTryInstruction),  
        ///
    /// 
    ///
    Exchange_Stable9(ExchangeWithTryInstruction),
    ///
    ///
    /// 
    Exchange_Stable10(ExchangeWithTryInstruction), 
    ///
    /// 
    ///
    Exchange_Stable11(ExchangeWithTryInstruction),
    ///
    ///
    /// 
    Exchange_Stable12(ExchangeWithTryInstruction),
        ///
    /// 
    ///
    Exchange_Stable13(ExchangeWithTryInstruction),
    ///
    ///
    /// 
    Exchange_Stable14(ExchangeWithTryInstruction),  
        ///
    /// 
    ///
    Exchange_Stable15(ExchangeWithTryInstruction),
    ///
    ///
    /// 
    Exchange_Stable16(ExchangeWithTryInstruction),  
        ///
    /// 
    ///
    Exchange_Stable17(ExchangeWithTryInstruction),
    ///
    ///
    /// 
    Exchange_Stable18(ExchangeWithTryInstruction),  
        ///
    /// 
    ///
    Exchange_Stable19(ExchangeWithTryInstruction),
    ///
    ///
    /// 
    Exchange_Stable20(ExchangeWithTryInstruction),  
        ///
    /// 
    ///
    Exchange_Stable21(ExchangeWithTryInstruction),
    ///
    ///
    /// 
    Exchange_Stable22(ExchangeWithTryInstruction),  
        ///
    /// 
    ///
    Exchange_Stable23(ExchangeWithTryInstruction),
    ///
    ///
    /// 
    Exchange_Stable24(ExchangeWithTryInstruction),  
        ///
    /// 
    ///
    Exchange_Stable25(ExchangeWithTryInstruction),
    ///
    ///
    /// 
    Exchange_Stable26(ExchangeWithTryInstruction),  
        ///
    /// 
    ///
    Exchange_Stable27(ExchangeWithTryInstruction),
    ///
    ///
    /// 
    Exchange_Stable28(ExchangeWithTryInstruction),  
            ///
    /// 
    ///
    Exchange_Stable29(ExchangeWithTryInstruction),
    ///
    ///
    /// 
    Exchange_Stable30(ExchangeWithTryInstruction),
            ///
    /// 
    ///
    Exchange_Stable31(ExchangeWithTryInstruction),
    ///
    ///
    /// 
    Exchange_Stable32(ExchangeWithTryInstruction),   
}

impl ArbitrageInstruction {
    /// Unpacks a byte buffer into a [EscrowInstruction](enum.EscrowInstruction.html).
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (tag, rest) = input.split_first().ok_or(ArbitrageError::InvalidInstruction)?;
        Ok(match tag {
            0..=10 => Self::Exchange_NonStable_All(ExchangeWithTryInstruction::unpack(rest)?),
            10..=20 => Self::Exchange_NonStable_Serum(),
            20..=30 => Self::Exchange_WithPath(ExchangeWithPathInstruction::unpack(rest)?),
            30..=40 => Self::Exchange_Stable1(ExchangeWithTryInstruction::unpack(rest)?),
            40..=50 => Self::Exchange_Stable2(ExchangeWithTryInstruction::unpack(rest)?),
            50..=60 => Self::Exchange_Stable3(ExchangeWithTryInstruction::unpack(rest)?),
            60..=70 => Self::Exchange_Stable4(ExchangeWithTryInstruction::unpack(rest)?),
            70..=80 => Self::Exchange_Stable5(ExchangeWithTryInstruction::unpack(rest)?),
            80..=90 => Self::Exchange_Stable6(ExchangeWithTryInstruction::unpack(rest)?),
            90..=100 => Self::Exchange_Stable7(ExchangeWithTryInstruction::unpack(rest)?),
            100..=110 => Self::Exchange_Stable8(ExchangeWithTryInstruction::unpack(rest)?),
            110..=120 => Self::Exchange_Stable9(ExchangeWithTryInstruction::unpack(rest)?),
            120..=130 => Self::Exchange_Stable10(ExchangeWithTryInstruction::unpack(rest)?),
            130..=140 => Self::Exchange_Stable11(ExchangeWithTryInstruction::unpack(rest)?),
            140..=150 => Self::Exchange_Stable12(ExchangeWithTryInstruction::unpack(rest)?),
            151 => Self::Exchange_Stable13(ExchangeWithTryInstruction::unpack(rest)?),
            152 => Self::Exchange_Stable14(ExchangeWithTryInstruction::unpack(rest)?),
            153 => Self::Exchange_Stable15(ExchangeWithTryInstruction::unpack(rest)?),
            154 => Self::Exchange_Stable16(ExchangeWithTryInstruction::unpack(rest)?),
            155 => Self::Exchange_Stable17(ExchangeWithTryInstruction::unpack(rest)?),
            156 => Self::Exchange_Stable18(ExchangeWithTryInstruction::unpack(rest)?),
            157 => Self::Exchange_Stable19(ExchangeWithTryInstruction::unpack(rest)?),
            158 => Self::Exchange_Stable20(ExchangeWithTryInstruction::unpack(rest)?),
            159 => Self::Exchange_Stable21(ExchangeWithTryInstruction::unpack(rest)?),
            160 => Self::Exchange_Stable22(ExchangeWithTryInstruction::unpack(rest)?),
            161 => Self::Exchange_Stable23(ExchangeWithTryInstruction::unpack(rest)?),
            162 => Self::Exchange_Stable24(ExchangeWithTryInstruction::unpack(rest)?),
            163 => Self::Exchange_Stable25(ExchangeWithTryInstruction::unpack(rest)?),
            164 => Self::Exchange_Stable26(ExchangeWithTryInstruction::unpack(rest)?),
            165 => Self::Exchange_Stable27(ExchangeWithTryInstruction::unpack(rest)?),
            166 => Self::Exchange_Stable28(ExchangeWithTryInstruction::unpack(rest)?),
            167 => Self::Exchange_Stable29(ExchangeWithTryInstruction::unpack(rest)?),
            168 => Self::Exchange_Stable30(ExchangeWithTryInstruction::unpack(rest)?),
            169 => Self::Exchange_Stable31(ExchangeWithTryInstruction::unpack(rest)?),
            170 => Self::Exchange_Stable32(ExchangeWithTryInstruction::unpack(rest)?),
            _ => return Err(ArbitrageError::InvalidInstruction.into()),
        })
    }
}

