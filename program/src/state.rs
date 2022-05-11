use solana_program::{
    program_error::ProgramError,
};

use arrayref::{array_mut_ref, array_ref, array_refs, mut_array_refs};

#[derive(Clone, Debug, PartialEq, Copy)]
pub struct ExchangeState {
    pub input_amount: u64,
    pub exchange_out: u64,
    pub total_profit: u64,
    pub total_lost: u64,
}

impl ExchangeState {
    const LEN: usize = 32;
    pub fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        let src = array_ref![src, 0, ExchangeState::LEN];
        let (
            input_amount_src,
            exchange_out_src,
            total_profit_src,
            total_lost_src,
        ) = array_refs![src, 8, 8, 8, 8];
        Ok(
            ExchangeState{
                input_amount: u64::from_le_bytes(*input_amount_src),
                exchange_out: u64::from_le_bytes(*exchange_out_src),
                total_profit: u64::from_le_bytes(*total_profit_src),
                total_lost: u64::from_le_bytes(*total_lost_src),
            }
        )
    }
    pub fn pack_into_slice(&self, dst: &mut [u8]) {
        let dst = array_mut_ref![dst, 0, ExchangeState::LEN];
        let (
            input_amount_dst,
            exchange_out_dst,
            total_profit_dst,
            total_lost_dst
        ) = mut_array_refs![dst, 8, 8, 8, 8];
        let ExchangeState {
            input_amount,
            exchange_out,
            total_profit,
            total_lost,
        } = self;
        *input_amount_dst = input_amount.to_le_bytes();
        *exchange_out_dst = exchange_out.to_le_bytes();
        *total_profit_dst = total_profit.to_le_bytes();
        *total_lost_dst = total_lost.to_le_bytes();
    }
}