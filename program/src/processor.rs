use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program::{invoke},
    program_pack::{Pack},
    pubkey::Pubkey,
};

use std::ops::DerefMut;

use serum_dex::{
  matching::Side,
  critbit::SlabView,
};

use spl_token::state::Account as TokenAccount;

use crate::{
  instruction::ArbitrageInstruction,
  error::{ArbitrageError},
};

use std::{num::NonZeroU64};

pub struct Processor;

impl Processor {
  pub fn process(
      accounts: &[AccountInfo],
      instruction_data: &[u8],
  ) -> ProgramResult {
    let instruction = ArbitrageInstruction::unpack(instruction_data)?;
    match instruction {
      ArbitrageInstruction::ExchangeV1 () => {
        msg!("Instruction: Exchange v1");
        Self::process_exchange_v1(accounts)
      }
      ArbitrageInstruction::ExchangeV2 () => {
        msg!("Instruction: Exchange v2");
        Self::process_exchange_v1(accounts)
      }   
    } 
  }

  //
  fn process_exchange_v1(accounts: &[AccountInfo]) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    //
    let state_acc = next_account_info(account_info_iter)?;
    //
    let saber_program_acc = next_account_info(account_info_iter)?;
    let saber_market_acc = next_account_info(account_info_iter)?;
    let saber_market_auth = next_account_info(account_info_iter)?;
    let saber_swap_a_acc = next_account_info(account_info_iter)?;
    let saber_swap_b_acc = next_account_info(account_info_iter)?;
    let saber_fee_acc = next_account_info(account_info_iter)?;
    //
    let mercurial_program_acc = next_account_info(account_info_iter)?;
    let mercurial_market_acc = next_account_info(account_info_iter)?;
    let mercurial_market_auth = next_account_info(account_info_iter)?;
    let mercurial_swap_acc1 = next_account_info(account_info_iter)?;
    let mercurial_swap_acc2 = next_account_info(account_info_iter)?;
    let mercurial_swap_acc3 = next_account_info(account_info_iter)?;
    let mercurial_program_acc = next_account_info(account_info_iter)?;
    let mercurial_program_acc = next_account_info(account_info_iter)?;
    //
    let owner_acc = next_account_info(account_info_iter)?;
    let user_usdc_acc = next_account_info(account_info_iter)?;
    let user_other_acc = next_account_info(account_info_iter)?;
    //
    let spl_token_program_acc = next_account_info(account_info_iter)?;
    let sys_clock = next_account_info(account_info_iter)?;

    //
    {
      let mut usdc_balance_before = 0;
      let mut usdc_balance_after = 0;
      {
        // init state
        let usdc_acc_info_before = TokenAccount::unpack(&user_usdc_acc.try_borrow_data()?)?;
        let usdc_acc_balance_before = usdc_acc_info_before.amount;
        usdc_balance_before = usdc_acc_balance_before;

        let other_acc_info_before = TokenAccount::unpack(&user_other_acc.try_borrow_data()?)?;
        let other_acc_balance_before = other_acc_info_before.amount;

        msg!("usdc balance before: {}", usdc_acc_balance_before);

        // saber: buy usdc -> ust, mercurial: sell ust -> usdc
        let usdc_amount_in = 1000000000;
        Self::saber_swap(
          saber_program_acc.key,
          saber_market_acc,
          saber_market_auth,
          owner_acc,
          user_usdc_acc,
          saber_swap_a_acc,
          saber_swap_b_acc,
          user_other_acc,
          saber_fee_acc,
          spl_token_program_acc,
          sys_clock,
          usdc_amount_in,
        )?;

        let other_acc_info_after = TokenAccount::unpack(&user_other_acc.try_borrow_data()?)?;
        let other_acc_balance_after = other_acc_info_after.amount;

        let other_amount_in = other_acc_balance_after - other_acc_balance_before;
        msg!("mercurial sell, amount in: {}", other_amount_in);

        Self::mercurial_swap(
          mercurial_program_acc.key,
          mercurial_market_acc,
          mercurial_market_auth,
          owner_acc,
          mercurial_swap_acc1,
          mercurial_swap_acc2,
          mercurial_swap_acc3,
          user_other_acc,
          user_usdc_acc,
          spl_token_program_acc,
          other_amount_in,
        )?;

        let usdc_acc_info_after = TokenAccount::unpack(&user_usdc_acc.try_borrow_data()?)?;
        let user_acc_balance_after = usdc_acc_info_after.amount;
        usdc_balance_after = user_acc_balance_after;
        msg!("usdc balance after: {}", usdc_balance_after);
      }
    }
    //
    Ok(())
  }

  // saber swap
  fn saber_swap<'a>(
    program_id: &Pubkey,
    market_acc: &AccountInfo<'a>,
    market_auth: &AccountInfo<'a>,
    owner_acc: &AccountInfo<'a>,
    user_src_acc: &AccountInfo<'a>,
    swap_src_acc: &AccountInfo<'a>,
    swap_dst_acc: &AccountInfo<'a>,
    user_dst_acc: &AccountInfo<'a>,
    fee_acc: &AccountInfo<'a>,
    spl_token_program_acc: &AccountInfo<'a>,
    sys_clock: &AccountInfo<'a>,
    amount_in: u64,
  ) -> ProgramResult {
    let saber_swap_accounts = [
      market_acc.clone(),
      market_auth.clone(),
      owner_acc.clone(),
      user_src_acc.clone(),
      swap_src_acc.clone(),
      swap_dst_acc.clone(),
      user_dst_acc.clone(),
      fee_acc.clone(),
      sys_clock.clone(),
      spl_token_program_acc.clone(),
    ];

    let saber_swap_instruction = stable_swap_client::instruction::swap(
      spl_token_program_acc.key,
      market_acc.key,
      market_auth.key,
      owner_acc.key,
      user_src_acc.key,
      swap_src_acc.key,
      swap_dst_acc.key,
      user_dst_acc.key,
      fee_acc.key,
      amount_in,
      1,
    ).map_err(|_| ArbitrageError::InvalidCall)?;

    msg!("invoke saber swap");
    invoke(&saber_swap_instruction, &saber_swap_accounts[..])?;

    Ok(())
  }

  fn mercurial_swap<'a>(
    program_id: &Pubkey,
    market_acc: &AccountInfo<'a>,
    market_auth: &AccountInfo<'a>,
    owner_acc: &AccountInfo<'a>,
    swap_acc1: &AccountInfo<'a>,
    swap_acc2: &AccountInfo<'a>,
    swap_acc3: &AccountInfo<'a>,
    user_src_acc: &AccountInfo<'a>,
    user_dst_acc: &AccountInfo<'a>,
    spl_token_program_acc: &AccountInfo<'a>,
    amount_in: u64,
  ) -> ProgramResult {

    let swap_accs = [
      swap_acc1.key,
      swap_acc2.key,
      swap_acc3.key,
    ];
    let mercurial_exchange_accounts = [
      market_acc.clone(),
      market_auth.clone(),
      swap_acc1.clone(),
      swap_acc2.clone(),
      swap_acc3.clone(),
      owner_acc.clone(),
      user_src_acc.clone(),
      user_dst_acc.clone(),
      spl_token_program_acc.clone(),
    ];

    let mercurial_exchange_instruction = mercurial_stable_swap_n_pool_instructions::instruction::exchange(
      program_id,
      market_acc.key,
      spl_token_program_acc.key,
      market_auth.key,
      owner_acc.key,
      swap_accs.to_vec(),
      user_src_acc.key,
      user_dst_acc.key,
      amount_in,
      1,
    ).map_err(|_| ArbitrageError::InvalidCall)?;

    msg!("invoke mercurial swap");
    invoke(&mercurial_exchange_instruction, &mercurial_exchange_accounts[..])?;

    Ok(())   
  }
}
