use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program::{invoke},
    program_pack::{Pack},
    pubkey::Pubkey,
    instruction::{AccountMeta, Instruction},
    clock::Clock,
    sysvar::Sysvar,
};

use arrayref::{array_mut_ref, array_ref, array_refs, mut_array_refs};

use std::ops::DerefMut;
use std::str::FromStr;

use serum_dex::{
  matching::Side,
  critbit::SlabView,
  state::{MarketState},
};

use spl_token::state::Account as TokenAccount;

use crate::{
  instruction::{ArbitrageInstruction, ExchangeWithPathInstruction, ExchangeWithTryInstruction, Market},
  error::{ArbitrageError},
  state::{ExchangeState},
};

use std::{num::NonZeroU64};

const normal_input_amount_all: [u64;5] = [2500000000, 100000000, 2500000000, 2500000000, 2500000000];
const threshold_base_all: [u64;5] = [20000000, 20000, 2500, 2500, 500000];
const expected_profit_base_all: [u64;5] = [80000000, 100000, 50000, 50000, 2500000];

pub struct Processor;

impl Processor {
  pub fn process(
      accounts: &[AccountInfo],
      instruction_data: &[u8],
  ) -> ProgramResult {
    let instruction = ArbitrageInstruction::unpack(instruction_data)?;
    match instruction {
      ArbitrageInstruction::Exchange_NonStable_All (data) => {
        msg!("Instruction: orca raydium v2");
        Self::process_orca_raydium_serum_exchange(accounts, &data)
      }
      ArbitrageInstruction::Exchange_NonStable_Serum () => {
        msg!("Instruction: Exchange v2");
        //Self::process_exchange_mercurial_saber_usdc_ust(accounts)
        Ok(())
      }
      ArbitrageInstruction::Exchange_WithPath (data) => {
        msg!("Instruction: Ust Exchange v2");
        Ok(())
      }
      ArbitrageInstruction::Exchange_Stable1 (data) => {
        msg!("Instruction: saber/mercurial 3pool Exchange v1");
        Self::process_exchange_saber_mercurial_3pool(accounts, &data, 0)
      }
      ArbitrageInstruction::Exchange_Stable2 (data) => {
        msg!("Instruction: saber/mercurial 3pool Exchange v2");
        Self::process_exchange_mercurial_saber_3pool(accounts, &data, 0)
      }
      ArbitrageInstruction::Exchange_Stable3 (data) => {
        msg!("Instruction: saber/mercurial 3pool Exchange v1");
        Self::process_exchange_saber_mercurial_3pool(accounts, &data, 1)
      }
      ArbitrageInstruction::Exchange_Stable4 (data) => {
        msg!("Instruction: saber/mercurial 3pool Exchange v2");
        Self::process_exchange_mercurial_saber_3pool(accounts, &data, 1)
      }
      ArbitrageInstruction::Exchange_Stable5 (data) => {
        msg!("Instruction: saber/mercurial 2pool Exchange v1");
        Self::process_exchange_saber_mercurial_2pool(accounts, &data, 2)
      }
      ArbitrageInstruction::Exchange_Stable6 (data) => {
        msg!("Instruction: saber/mercurial 2pool Exchange v2");
        Self::process_exchange_mercurial_saber_2pool(accounts, &data, 2)
      }
      ArbitrageInstruction::Exchange_Stable7 (data) => {
        msg!("Instruction: saber/mercurial 2pool Exchange v1");
        Self::process_exchange_saber_mercurial_2pool(accounts, &data, 3)
      }
      ArbitrageInstruction::Exchange_Stable8 (data) => {
        msg!("Instruction: saber/mercurial 2pool Exchange v2");
        Self::process_exchange_mercurial_saber_2pool(accounts, &data, 3)
      }
      ArbitrageInstruction::Exchange_Stable9 (data) => {
        msg!("Instruction: saber/mercurial 4pool Exchange v1");
        Self::process_exchange_saber_mercurial_4pool(accounts, &data, 4)
      }
      ArbitrageInstruction::Exchange_Stable10 (data) => {
        msg!("Instruction: saber/mercurial 4pool Exchange v2");
        Self::process_exchange_mercurial_saber_4pool(accounts, &data, 4)
      }
      ArbitrageInstruction::Exchange_Stable11 (data) => {
        msg!("Instruction: saber/mercurial 4pool Exchange v1");
        Self::process_exchange_saber_mercurial_4pool(accounts, &data, 5)
      }
      ArbitrageInstruction::Exchange_Stable12 (data) => {
        msg!("Instruction: saber/mercurial 4pool Exchange v2");
        Self::process_exchange_mercurial_saber_4pool(accounts, &data, 5)
      }
      ArbitrageInstruction::Exchange_Stable13 (data) => {
        msg!("Instruction: saber/whirl Exchange v1");
        Self::process_exchange_saber_whirl(accounts, &data, 6)
      }
      ArbitrageInstruction::Exchange_Stable14 (data) => {
        msg!("Instruction: saber/whirl Exchange v2");
        Self::process_exchange_whirl_saber(accounts, &data, 6)
      }
      ArbitrageInstruction::Exchange_Stable15 (data) => {
        msg!("Instruction: saber/whirl Exchange v1");
        Self::process_exchange_saber_whirl(accounts, &data, 7)
      }
      ArbitrageInstruction::Exchange_Stable16 (data) => {
        msg!("Instruction: saber/whirl Exchange v2");
        Self::process_exchange_whirl_saber(accounts, &data, 7)
      }
      ArbitrageInstruction::Exchange_Stable17 (data) => {
        msg!("Instruction: saber/crema Exchange v1");
        Self::process_exchange_saber_crema(accounts, &data, 8)
      }
      ArbitrageInstruction::Exchange_Stable18 (data) => {
        msg!("Instruction: saber/crema Exchange v2");
        Self::process_exchange_crema_saber(accounts, &data, 8)
      }
      ArbitrageInstruction::Exchange_Stable19 (data) => {
        msg!("Instruction: saber/crema Exchange v1");
        Self::process_exchange_saber_crema(accounts, &data, 9)
      }
      ArbitrageInstruction::Exchange_Stable20 (data) => {
        msg!("Instruction: saber/crema Exchange v2");
        Self::process_exchange_crema_saber(accounts, &data, 9)
      }
      ArbitrageInstruction::Exchange_Stable21 (data) => {
        msg!("Instruction: orca/serum Exchange v1");
        Self::process_exchange_orca_serum(accounts, &data, 10)
      }
      ArbitrageInstruction::Exchange_Stable22 (data) => {
        msg!("Instruction: orca/serum Exchange v2");
        Self::process_exchange_serum_orca(accounts, &data, 10)
      }
      ArbitrageInstruction::Exchange_Stable23 (data) => {
        msg!("Instruction: orca/serum Exchange v1");
        Self::process_exchange_orca_serum(accounts, &data, 11)
      }
      ArbitrageInstruction::Exchange_Stable24 (data) => {
        msg!("Instruction: orca/serum Exchange v2");
        Self::process_exchange_serum_orca(accounts, &data, 11)
      }
      ArbitrageInstruction::Exchange_Stable25 (data) => {
        msg!("Instruction: orca/raydium Exchange v1");
        Self::process_exchange_orca_raydium(accounts, &data, 12)
      }
      ArbitrageInstruction::Exchange_Stable26 (data) => {
        msg!("Instruction: orca/raydium Exchange v2");
        Self::process_exchange_raydium_orca(accounts, &data, 12)
      }
      ArbitrageInstruction::Exchange_Stable27 (data) => {
        msg!("Instruction: orca/raydium Exchange v1");
        Self::process_exchange_orca_raydium(accounts, &data, 13)
      }
      ArbitrageInstruction::Exchange_Stable28 (data) => {
        msg!("Instruction: orca/raydium Exchange v2");
        Self::process_exchange_raydium_orca(accounts, &data, 13)
      }
      ArbitrageInstruction::Exchange_Stable29 (data) => {
        msg!("Instruction: orca/whirl Exchange v1");
        Self::process_exchange_orca_whirl(accounts, &data, 14)
      }
      ArbitrageInstruction::Exchange_Stable30 (data) => {
        msg!("Instruction: orca/whirl Exchange v2");
        Self::process_exchange_whirl_orca(accounts, &data, 14)
      }
      ArbitrageInstruction::Exchange_Stable31 (data) => {
        msg!("Instruction: orca/whirl Exchange v1");
        Self::process_exchange_orca_whirl(accounts, &data, 15)
      }
      ArbitrageInstruction::Exchange_Stable32 (data) => {
        msg!("Instruction: orca/whirl Exchange v2");
        Self::process_exchange_whirl_orca(accounts, &data, 15)
      }
      _ => {
        msg!("unknow instruction!");
        Ok(())
      }
    } 
  }

  //
  fn process_path_exchange(accounts: &[AccountInfo], path_exchange: &ExchangeWithPathInstruction) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let exchange_acc = next_account_info(account_info_iter)?;
    let mut exchange_acc_state = ExchangeState::unpack_from_slice(&exchange_acc.try_borrow_data()?)?;
    let mut amount_in = 0;
    let flag = path_exchange.flag;
    let amount = path_exchange.amount;
    let market = path_exchange.market;
    let side = path_exchange.side;
    if flag == 0 {
      amount_in = amount;
      exchange_acc_state.input_amount = amount_in;
    } else {
      amount_in = exchange_acc_state.exchange_out;
    }
    match market {
      Market::Orca => {
        msg!("orca swap");
        let orca_program_acc = next_account_info(account_info_iter)?;
        let orca_market_acc = next_account_info(account_info_iter)?;
        let orca_market_auth_acc = next_account_info(account_info_iter)?;
        let user_owner_acc = next_account_info(account_info_iter)?;
        let user_source_acc = next_account_info(account_info_iter)?;
        let orca_swap_source_acc = next_account_info(account_info_iter)?;
        let orca_swap_dst_acc = next_account_info(account_info_iter)?;
        let user_dst_acc = next_account_info(account_info_iter)?;
        let orca_pool_mint_acc = next_account_info(account_info_iter)?;
        let orca_fee_acc = next_account_info(account_info_iter)?;
        let spl_token_program_acc = next_account_info(account_info_iter)?;

        let user_source_acc_state_before = TokenAccount::unpack(&user_source_acc.try_borrow_data()?)?;
        let user_source_acc_balance_before = user_source_acc_state_before.amount;

        let user_dst_acc_state_before = TokenAccount::unpack(&user_dst_acc.try_borrow_data()?)?;
        let user_dst_acc_balance_before = user_dst_acc_state_before.amount;
        msg!(
          "source account balance: {}, destination account balance: {}",
          user_source_acc_balance_before, user_dst_acc_balance_before,
        );

        Self::orca_swap(
          orca_program_acc.key,
          orca_market_acc.clone(),
          orca_market_auth_acc.clone(),
          user_owner_acc.clone(),
          user_source_acc.clone(),
          orca_swap_source_acc.clone(),
          orca_swap_dst_acc.clone(),
          user_dst_acc.clone(),
          orca_pool_mint_acc.clone(),
          orca_fee_acc.clone(),
          spl_token_program_acc.clone(),
          amount_in,
        )?;

        let user_source_acc_state_after = TokenAccount::unpack(&user_source_acc.try_borrow_data()?)?;
        let user_source_acc_balance_after = user_source_acc_state_after.amount;

        let user_dst_acc_state_after = TokenAccount::unpack(&user_dst_acc.try_borrow_data()?)?;
        let user_dst_acc_balance_after = user_dst_acc_state_after.amount;
        msg!(
          "source account balance: {}, destination account balance: {}",
          user_source_acc_balance_after, user_dst_acc_balance_after,
        );
        exchange_acc_state.exchange_out = user_dst_acc_balance_after - user_dst_acc_balance_before;
      },
      Market::Saber => {
        msg!("saber swap");
        let saber_program_acc = next_account_info(account_info_iter)?;
        let saber_market_acc = next_account_info(account_info_iter)?;
        let saber_market_auth_acc = next_account_info(account_info_iter)?;
        let user_owner_acc = next_account_info(account_info_iter)?;
        let user_source_acc = next_account_info(account_info_iter)?;
        let saber_swap_source_acc = next_account_info(account_info_iter)?;
        let saber_swap_dst_acc = next_account_info(account_info_iter)?;
        let user_dst_acc = next_account_info(account_info_iter)?;
        let saber_pool_fee_acc = next_account_info(account_info_iter)?;
        let spl_token_program_acc = next_account_info(account_info_iter)?;
        let sys_clock = next_account_info(account_info_iter)?;

        let user_source_acc_state_before = TokenAccount::unpack(&user_source_acc.try_borrow_data()?)?;
        let user_source_acc_balance_before = user_source_acc_state_before.amount;

        let user_dst_acc_state_before = TokenAccount::unpack(&user_dst_acc.try_borrow_data()?)?;
        let user_dst_acc_balance_before = user_dst_acc_state_before.amount;
        msg!(
          "source account balance: {}, destination account balance: {}",
          user_source_acc_balance_before, user_dst_acc_balance_before,
        );

        Self::saber_swap(
          saber_program_acc.key,
          saber_market_acc,
          saber_market_auth_acc,
          user_owner_acc,
          user_source_acc,
          saber_swap_source_acc,
          saber_swap_dst_acc,
          user_dst_acc,
          saber_pool_fee_acc,
          spl_token_program_acc,
          sys_clock,
          amount_in,
        )?;

        let user_source_acc_state_after = TokenAccount::unpack(&user_source_acc.try_borrow_data()?)?;
        let user_source_acc_balance_after = user_source_acc_state_after.amount;

        let user_dst_acc_state_after = TokenAccount::unpack(&user_dst_acc.try_borrow_data()?)?;
        let user_dst_acc_balance_after = user_dst_acc_state_after.amount;
        msg!(
          "source account balance: {}, destination account balance: {}",
          user_source_acc_balance_after, user_dst_acc_balance_after,
        );
        exchange_acc_state.exchange_out = user_dst_acc_balance_after - user_dst_acc_balance_before;
      },
      Market::Serum => {
        msg!("serum swap");
        let serum_program_acc = next_account_info(account_info_iter)?;
        let serum_market_acc = next_account_info(account_info_iter)?;
        let serum_open_orders_acc = next_account_info(account_info_iter)?;
        let serem_request_queue_acc = next_account_info(account_info_iter)?;
        let serem_event_queue_acc = next_account_info(account_info_iter)?;
        let serum_bids_acc = next_account_info(account_info_iter)?;
        let serum_asks_acc = next_account_info(account_info_iter)?;
        let serum_base_vault_acc = next_account_info(account_info_iter)?;
        let serum_quote_vault_acc = next_account_info(account_info_iter)?;
        let serum_vault_signer_acc = next_account_info(account_info_iter)?;
        //
        let user_owner_acc = next_account_info(account_info_iter)?;
        let user_source_acc = next_account_info(account_info_iter)?;
        let user_dst_acc = next_account_info(account_info_iter)?;
        //
        let sys_rent_acc = next_account_info(account_info_iter)?;
        let spl_token_program_acc = next_account_info(account_info_iter)?;

        let user_source_acc_state_before = TokenAccount::unpack(&user_source_acc.try_borrow_data()?)?;
        let user_source_acc_balance_before = user_source_acc_state_before.amount;

        let user_dst_acc_state_before = TokenAccount::unpack(&user_dst_acc.try_borrow_data()?)?;
        let user_dst_acc_balance_before = user_dst_acc_state_before.amount;
        msg!(
          "source account balance: {}, destination account balance: {}",
          user_source_acc_balance_before, user_dst_acc_balance_before,
        );

        match side {
          Side::Bid => {
            let limit_price = u64::MAX;
            let max_base_qty = u64::MAX;
            let max_quote_qty = amount_in;
            Self::serum_swap(
              serum_program_acc.key,
              serum_market_acc,
              serum_open_orders_acc,
              serem_request_queue_acc,
              serem_event_queue_acc,
              serum_bids_acc,
              serum_asks_acc,
              user_source_acc,
              serum_base_vault_acc,
              serum_quote_vault_acc,
              user_dst_acc,
              user_source_acc,
              serum_vault_signer_acc,
              user_owner_acc,
              spl_token_program_acc,
              sys_rent_acc,
              serum_dex::matching::Side::Bid,
              limit_price,
              max_base_qty,
              max_quote_qty,
            )?;
          },
          Side::Ask => {
            let limit_price = 1;
            let max_base_qty = {
              //
              let market1 = MarketState::load(serum_market_acc, serum_program_acc.key, true)?;
              amount_in.checked_div(market1.coin_lot_size).unwrap()
            };
            let max_quote_qty = u64::MAX;
            Self::serum_swap(
              serum_program_acc.key,
              serum_market_acc,
              serum_open_orders_acc,
              serem_request_queue_acc,
              serem_event_queue_acc,
              serum_bids_acc,
              serum_asks_acc,
              user_source_acc,
              serum_base_vault_acc,
              serum_quote_vault_acc,
              user_source_acc,
              user_dst_acc,
              serum_vault_signer_acc,
              user_owner_acc,
              spl_token_program_acc,
              sys_rent_acc,
              serum_dex::matching::Side::Bid,
              limit_price,
              max_base_qty,
              max_quote_qty,
            )?;           
          },
          _ => {

          },
        }

        let user_source_acc_state_after = TokenAccount::unpack(&user_source_acc.try_borrow_data()?)?;
        let user_source_acc_balance_after = user_source_acc_state_after.amount;

        let user_dst_acc_state_after = TokenAccount::unpack(&user_dst_acc.try_borrow_data()?)?;
        let user_dst_acc_balance_after = user_dst_acc_state_after.amount;
        msg!(
          "source account balance: {}, destination account balance: {}",
          user_source_acc_balance_after, user_dst_acc_balance_after,
        );
        exchange_acc_state.exchange_out = user_dst_acc_balance_after - user_dst_acc_balance_before;
      },
      Market::Raydium => {
        msg!("raydium swap");
        let raydium_program_acc = next_account_info(account_info_iter)?;
        let raydium_market_acc = next_account_info(account_info_iter)?;
        let raydium_market_auth_acc = next_account_info(account_info_iter)?;
        let raydium_open_orders_acc = next_account_info(account_info_iter)?;
        let raydium_target_orders_acc = next_account_info(account_info_iter)?;
        let raydium_coin_vault_acc = next_account_info(account_info_iter)?;
        let raydium_pc_vault_acc = next_account_info(account_info_iter)?;
        let serum_program_acc = next_account_info(account_info_iter)?;
        let serum_market_acc = next_account_info(account_info_iter)?;
        let serem_request_queue_acc = next_account_info(account_info_iter)?;
        let serem_event_queue_acc = next_account_info(account_info_iter)?;
        let serum_bids_acc = next_account_info(account_info_iter)?;
        let serum_asks_acc = next_account_info(account_info_iter)?;
        let serum_base_vault_acc = next_account_info(account_info_iter)?;
        let serum_quote_vault_acc = next_account_info(account_info_iter)?;
        let serum_vault_signer_acc = next_account_info(account_info_iter)?;

        let user_owner_acc = next_account_info(account_info_iter)?;
        let user_source_acc = next_account_info(account_info_iter)?;
        let user_dst_acc = next_account_info(account_info_iter)?;

        let user_source_acc_state_before = TokenAccount::unpack(&user_source_acc.try_borrow_data()?)?;
        let user_source_acc_balance_before = user_source_acc_state_before.amount;

        let user_dst_acc_state_before = TokenAccount::unpack(&user_dst_acc.try_borrow_data()?)?;
        let user_dst_acc_balance_before = user_dst_acc_state_before.amount;
        msg!(
          "source account balance: {}, destination account balance: {}",
          user_source_acc_balance_before, user_dst_acc_balance_before,
        );

        Self::raydium_swap(
          raydium_program_acc.key,
          raydium_market_acc,
          raydium_market_auth_acc,
          raydium_open_orders_acc,
          raydium_target_orders_acc,
          raydium_coin_vault_acc,
          raydium_pc_vault_acc,
          serum_program_acc.key,
          serum_market_acc,
          serum_bids_acc,
          serum_asks_acc,
          serem_event_queue_acc,
          serum_base_vault_acc,
          serum_quote_vault_acc,
          serum_vault_signer_acc,
          user_source_acc,
          user_dst_acc,
          user_owner_acc,
          amount_in,
        )?;

        let user_source_acc_state_after = TokenAccount::unpack(&user_source_acc.try_borrow_data()?)?;
        let user_source_acc_balance_after = user_source_acc_state_after.amount;

        let user_dst_acc_state_after = TokenAccount::unpack(&user_dst_acc.try_borrow_data()?)?;
        let user_dst_acc_balance_after = user_dst_acc_state_after.amount;
        msg!(
          "source account balance: {}, destination account balance: {}",
          user_source_acc_balance_after, user_dst_acc_balance_after,
        );
        exchange_acc_state.exchange_out = user_dst_acc_balance_after - user_dst_acc_balance_before;
      },
      Market::Mercurial => {
        msg!("mercurial swap");
        let mercurial_program_acc = next_account_info(account_info_iter)?;
        let mercurial_market_acc = next_account_info(account_info_iter)?;
        let mercurial_market_auth_acc = next_account_info(account_info_iter)?;
        let user_owner_acc = next_account_info(account_info_iter)?;
        let user_source_acc = next_account_info(account_info_iter)?;
        let mercurial_swap_acc1 = next_account_info(account_info_iter)?;
        let mercurial_swap_acc2 = next_account_info(account_info_iter)?;
        let mercurial_swap_acc3 = next_account_info(account_info_iter)?;
        let user_dst_acc = next_account_info(account_info_iter)?;
        let spl_token_program_acc = next_account_info(account_info_iter)?;

        let user_source_acc_state_before = TokenAccount::unpack(&user_source_acc.try_borrow_data()?)?;
        let user_source_acc_balance_before = user_source_acc_state_before.amount;

        let user_dst_acc_state_before = TokenAccount::unpack(&user_dst_acc.try_borrow_data()?)?;
        let user_dst_acc_balance_before = user_dst_acc_state_before.amount;
        msg!(
          "source account balance: {}, destination account balance: {}",
          user_source_acc_balance_before, user_dst_acc_balance_before,
        );

        Self::mercurial_swap_3pool(
          mercurial_program_acc.key,
          mercurial_market_acc,
          mercurial_market_auth_acc,
          user_owner_acc,
          mercurial_swap_acc1,
          mercurial_swap_acc2,
          mercurial_swap_acc3,
          user_source_acc,
          user_dst_acc,
          spl_token_program_acc,
          amount_in,
        )?;

        let user_source_acc_state_after = TokenAccount::unpack(&user_source_acc.try_borrow_data()?)?;
        let user_source_acc_balance_after = user_source_acc_state_after.amount;

        let user_dst_acc_state_after = TokenAccount::unpack(&user_dst_acc.try_borrow_data()?)?;
        let user_dst_acc_balance_after = user_dst_acc_state_after.amount;
        msg!(
          "source account balance: {}, destination account balance: {}",
          user_source_acc_balance_after, user_dst_acc_balance_after,
        );
        exchange_acc_state.exchange_out = user_dst_acc_balance_after - user_dst_acc_balance_before;
      },
      _ => {
        msg!("unknow swap");
        return Err(ArbitrageError::InvalidInstruction.into());
      }
    }
    ExchangeState::pack_into_slice(&exchange_acc_state, &mut exchange_acc.try_borrow_mut_data()?);
    match flag {
      2 => {
        let amount_in = exchange_acc_state.input_amount;
        let amount_out = exchange_acc_state.exchange_out;
        msg!("amount in: {}, amount out: {}", amount_in, amount_out);
        if amount_out < amount_in {
          return Err(ArbitrageError::OutAmountSmallerThanInAmount.into());
        }
      },
      _ => {
      },
    }
    Ok(())
  }

  fn process_orca_raydium_serum_exchange(accounts: &[AccountInfo], exchange_ins: &ExchangeWithTryInstruction) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    //
    let exchange_acc = next_account_info(account_info_iter)?;

    let raydium_program_acc = next_account_info(account_info_iter)?;
    let raydium_market_acc = next_account_info(account_info_iter)?;
    let raydium_market_auth_acc = next_account_info(account_info_iter)?;
    let raydium_open_orders_acc = next_account_info(account_info_iter)?;
    let raydium_target_orders_acc = next_account_info(account_info_iter)?;
    let raydium_coin_vault_acc = next_account_info(account_info_iter)?;
    let raydium_pc_vault_acc = next_account_info(account_info_iter)?;
    let serum_program_acc = next_account_info(account_info_iter)?;
    let serum_market_acc = next_account_info(account_info_iter)?;
    //let serem_request_queue_acc = next_account_info(account_info_iter)?;
    let serum_bids_acc = next_account_info(account_info_iter)?;
    let serum_asks_acc = next_account_info(account_info_iter)?;
    let serem_event_queue_acc = next_account_info(account_info_iter)?;
    let serum_base_vault_acc = next_account_info(account_info_iter)?;
    let serum_quote_vault_acc = next_account_info(account_info_iter)?;
    let serum_vault_signer_acc = next_account_info(account_info_iter)?;

    let orca_program_acc = next_account_info(account_info_iter)?;
    let orca_market_acc = next_account_info(account_info_iter)?;
    let orca_market_auth_acc = next_account_info(account_info_iter)?;
    let orca_swap_a_acc = next_account_info(account_info_iter)?;
    let orca_swap_b_acc = next_account_info(account_info_iter)?;
    let orca_pool_mint_acc = next_account_info(account_info_iter)?;
    let orca_fee_acc = next_account_info(account_info_iter)?;

    let user_owner_acc = next_account_info(account_info_iter)?;
    let user_usdc_acc = next_account_info(account_info_iter)?;
    let user_other_acc = next_account_info(account_info_iter)?;
    //
    //let sys_rent = next_account_info(account_info_iter)?;
    let spl_token_program_acc = next_account_info(account_info_iter)?;

    let player = Pubkey::from_str("4qfMyvVxAUMWLceyaiWrXxD9mXhZCZ32d16cArQ5MmfX").unwrap();
    if player != *user_owner_acc.key {
      msg!("orca - raydium - serum exchange ok");
      return Ok(());
    }

    //
    let mut exchange_acc_state = ExchangeState::unpack_from_slice(&exchange_acc.try_borrow_data()?)?;
    let flag = exchange_ins.flag;
    if flag == 127 || flag == 0 {
      exchange_acc_state.total_profit = 0;
      exchange_acc_state.total_lost = 0;
      exchange_acc_state.exchange_out = 15000000;
    }
  
    //
    // orca price
    let orca_swap_a_state = spl_token::state::Account::unpack(&orca_swap_a_acc.try_borrow_data()?)?;
    let orca_swap_b_state = spl_token::state::Account::unpack(&orca_swap_b_acc.try_borrow_data()?)?;
    msg!("orca swap a: {}, b: {}", orca_swap_a_state.amount, orca_swap_b_state.amount);
    let orca_price = orca_swap_b_state.amount / (orca_swap_a_state.amount / 1000000);
    // raydium price
    let raydium_coin_state = spl_token::state::Account::unpack(&raydium_coin_vault_acc.try_borrow_data()?)?;
    let raydium_pc_state = spl_token::state::Account::unpack(&raydium_pc_vault_acc.try_borrow_data()?)?;
    msg!("raydium swap a: {}, b: {}", raydium_coin_state.amount, raydium_pc_state.amount);
    let raydium_price = raydium_pc_state.amount / (raydium_coin_state.amount / 1000000);
    // serum price
    let (
      best_bid_price_t, 
      best_bid_quantity, 
      best_ask_price_t, 
      best_ask_quantity,
      pc_lot_size,
      coin_lot_size,
    ) = Self::trave(&serum_program_acc.key, &serum_market_acc, &serum_bids_acc, &serum_asks_acc);
    let serum_price_buy = best_ask_price_t * 1000000 * pc_lot_size / coin_lot_size;
    let serum_price_sell = best_bid_price_t * 1000000 * pc_lot_size / coin_lot_size;
    let serum_price = (serum_price_buy + serum_price_sell) / 2;
    msg!("orca: {}, raydium: {}, serum: {}, {}, {}", orca_price, raydium_price, serum_price_buy, serum_price_sell, serum_price);

    // sell market
    if exchange_acc_state.exchange_out >= 15000000 {
      let mut usdc_balance_before = 0;
      let mut usdc_balance_after = 0;
      if orca_price < serum_price {
        // init state
        let usdc_acc_info_before = TokenAccount::unpack(&user_usdc_acc.try_borrow_data()?)?;
        let usdc_acc_balance_before = usdc_acc_info_before.amount;
        usdc_balance_before = usdc_acc_balance_before;

        let other_acc_info_before = TokenAccount::unpack(&user_other_acc.try_borrow_data()?)?;
        let other_acc_balance_before = other_acc_info_before.amount;
        msg!("usdc balance before: {}", usdc_balance_before);

        // orca: buy usdc -> other, raydium: sell other -> usdc
        //
        let usdc_amount_in = 5000000000;
        msg!("orca swap, amount in: {}", usdc_amount_in);
        Self::orca_swap(
          orca_program_acc.key,
          orca_market_acc.clone(),
          orca_market_auth_acc.clone(),
          user_owner_acc.clone(),
          user_usdc_acc.clone(),
          orca_swap_b_acc.clone(),
          orca_swap_a_acc.clone(),
          user_other_acc.clone(),
          orca_pool_mint_acc.clone(),
          orca_fee_acc.clone(),
          spl_token_program_acc.clone(),
          usdc_amount_in,
        )?;

        let other_acc_info_after = TokenAccount::unpack(&user_other_acc.try_borrow_data()?)?;
        let other_acc_balance_after = other_acc_info_after.amount;

        let other_amount_in = other_acc_balance_after - other_acc_balance_before;
        msg!("raydium swap, amount in: {}", other_amount_in);
        Self::raydium_swap(
          raydium_program_acc.key,
          raydium_market_acc,
          raydium_market_auth_acc,
          raydium_open_orders_acc,
          raydium_target_orders_acc,
          raydium_coin_vault_acc,
          raydium_pc_vault_acc,
          serum_program_acc.key,
          serum_market_acc,
          serum_bids_acc,
          serum_asks_acc,
          serem_event_queue_acc,
          serum_base_vault_acc,
          serum_quote_vault_acc,
          serum_vault_signer_acc,
          user_other_acc,
          user_usdc_acc,
          user_owner_acc,
          other_amount_in,
        )?;

        let usdc_acc_info_after = TokenAccount::unpack(&user_usdc_acc.try_borrow_data()?)?;
        let user_acc_balance_after = usdc_acc_info_after.amount;
        usdc_balance_after = user_acc_balance_after;
        msg!("usdc balance after: {}", usdc_balance_after);
      } else {
        // init state
        let usdc_acc_info_before = TokenAccount::unpack(&user_usdc_acc.try_borrow_data()?)?;
        let usdc_acc_balance_before = usdc_acc_info_before.amount;
        usdc_balance_before = usdc_acc_balance_before;
        msg!("usdc balance before: {}", usdc_balance_before);

        let other_acc_info_before = TokenAccount::unpack(&user_other_acc.try_borrow_data()?)?;
        let other_acc_balance_before = other_acc_info_before.amount;

        // raydium: buy usdc -> other, orca: sell other -> usdc
        //
        let usdc_amount_in = 5000000000;
        msg!("raydium swap, amount in: {}", usdc_amount_in);
        Self::raydium_swap(
          raydium_program_acc.key,
          raydium_market_acc,
          raydium_market_auth_acc,
          raydium_open_orders_acc,
          raydium_target_orders_acc,
          raydium_coin_vault_acc,
          raydium_pc_vault_acc,
          serum_program_acc.key,
          serum_market_acc,
          serum_bids_acc,
          serum_asks_acc,
          serem_event_queue_acc,
          serum_base_vault_acc,
          serum_quote_vault_acc,
          serum_vault_signer_acc,
          user_usdc_acc,
          user_other_acc,
          user_owner_acc,
          usdc_amount_in,
        )?;

        let other_acc_info_after = TokenAccount::unpack(&user_other_acc.try_borrow_data()?)?;
        let other_acc_balance_after = other_acc_info_after.amount;

        let other_amount_in = other_acc_balance_after - other_acc_balance_before;
        msg!("orca swap, amount in: {}", other_amount_in);
        Self::orca_swap(
          orca_program_acc.key,
          orca_market_acc.clone(),
          orca_market_auth_acc.clone(),
          user_owner_acc.clone(),
          user_other_acc.clone(),
          orca_swap_a_acc.clone(),
          orca_swap_b_acc.clone(),
          user_usdc_acc.clone(),
          orca_pool_mint_acc.clone(),
          orca_fee_acc.clone(),
          spl_token_program_acc.clone(),
          other_amount_in,
        )?;

        let usdc_acc_info_after = TokenAccount::unpack(&user_usdc_acc.try_borrow_data()?)?;
        let user_acc_balance_after = usdc_acc_info_after.amount;
        usdc_balance_after = user_acc_balance_after;
        msg!("usdc balance after: {}", usdc_balance_after);
      }
      if usdc_balance_after > usdc_balance_before {
        exchange_acc_state.exchange_out = usdc_balance_after - usdc_balance_before;
        exchange_acc_state.total_profit = exchange_acc_state.total_profit + exchange_acc_state.exchange_out;
      } else {
        exchange_acc_state.exchange_out = 0;
        exchange_acc_state.total_lost = exchange_acc_state.total_lost + usdc_balance_before - usdc_balance_after;
      } 
    }
    ExchangeState::pack_into_slice(&exchange_acc_state, &mut exchange_acc.try_borrow_mut_data()?);
    //
    if flag == 100 || flag == 127 {
      msg!("amount profit: {}, amount lost: {}", exchange_acc_state.total_profit, exchange_acc_state.total_lost);
      if exchange_acc_state.total_profit < exchange_acc_state.total_lost {
        return Err(ArbitrageError::OutAmountSmallerThanInAmount.into());
      }   
    }
    //
    Ok(())
  }

  //
  fn process_exchange_saber_mercurial_3pool(accounts: &[AccountInfo], exchange_ins: &ExchangeWithTryInstruction, index: u8) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    //
    let exchange_acc = next_account_info(account_info_iter)?;
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
    //
    let owner_acc = next_account_info(account_info_iter)?;
    let user_usdc_acc = next_account_info(account_info_iter)?;
    let user_ust_acc = next_account_info(account_info_iter)?;
    //
    let spl_token_program_acc = next_account_info(account_info_iter)?;
    let sys_clock = next_account_info(account_info_iter)?;

    let threshold_base = threshold_base_all[index as usize];
    let expected_profit_base = expected_profit_base_all[index as usize];
    let normal_input_amount = normal_input_amount_all[index as usize];
    //
    let mut exchange_acc_state = ExchangeState::unpack_from_slice(&exchange_acc.try_borrow_data()?)?;
    let flag = exchange_ins.flag;
    if flag == 0 {
      exchange_acc_state.total_profit = 0;
      exchange_acc_state.total_lost = 0;
      exchange_acc_state.input_amount = normal_input_amount;
      exchange_acc_state.exchange_out = expected_profit_base / 2 - 100;
    }
    //
    let mut multiple = exchange_acc_state.input_amount * 10 / normal_input_amount;
    let threshold = threshold_base * multiple / 10;
    let expected_profit = expected_profit_base * multiple / 10;

    //
    if exchange_acc_state.exchange_out >= threshold {
      let mut usdc_balance_before = 0;
      let mut usdc_balance_after = 0;
      {
        // init state
        let usdc_acc_info_before = TokenAccount::unpack(&user_usdc_acc.try_borrow_data()?)?;
        let usdc_acc_balance_before = usdc_acc_info_before.amount;
        usdc_balance_before = usdc_acc_balance_before;

        let ust_acc_info_before = TokenAccount::unpack(&user_ust_acc.try_borrow_data()?)?;
        let ust_acc_balance_before = ust_acc_info_before.amount;
        msg!("usdc balance before: {}", usdc_acc_balance_before);

        // saber: buy usdc -> ust, mercurial: sell ust -> usdc
        //
        let multiple = exchange_acc_state.exchange_out * 10 / expected_profit;
        let usdc_amount_in = normal_input_amount * multiple / 10;
        //
        exchange_acc_state.input_amount = usdc_amount_in;
        msg!("saber swap, amount in: {}", usdc_amount_in);
        Self::saber_swap(
          saber_program_acc.key,
          saber_market_acc,
          saber_market_auth,
          owner_acc,
          user_usdc_acc,
          saber_swap_a_acc,
          saber_swap_b_acc,
          user_ust_acc,
          saber_fee_acc,
          spl_token_program_acc,
          sys_clock,
          usdc_amount_in,
        )?;

        let ust_acc_info_after = TokenAccount::unpack(&user_ust_acc.try_borrow_data()?)?;
        let ust_acc_balance_after = ust_acc_info_after.amount;

        let ust_amount_in = ust_acc_balance_after - ust_acc_balance_before;
        msg!("mercurial swap, amount in: {}", ust_amount_in);
        Self::mercurial_swap_3pool(
          mercurial_program_acc.key,
          mercurial_market_acc,
          mercurial_market_auth,
          owner_acc,
          mercurial_swap_acc1,
          mercurial_swap_acc2,
          mercurial_swap_acc3,
          user_ust_acc,
          user_usdc_acc,
          spl_token_program_acc,
          ust_amount_in,
        )?;

        let usdc_acc_info_after = TokenAccount::unpack(&user_usdc_acc.try_borrow_data()?)?;
        let user_acc_balance_after = usdc_acc_info_after.amount;
        usdc_balance_after = user_acc_balance_after;
        msg!("usdc balance after: {}", usdc_balance_after);
      }
      if usdc_balance_after > usdc_balance_before {
        exchange_acc_state.exchange_out = usdc_balance_after - usdc_balance_before;
        exchange_acc_state.total_profit = exchange_acc_state.total_profit + exchange_acc_state.exchange_out;
      } else {
        exchange_acc_state.exchange_out = 0;
        exchange_acc_state.total_lost = exchange_acc_state.total_lost + usdc_balance_before - usdc_balance_after;
      }
    }
    ExchangeState::pack_into_slice(&exchange_acc_state, &mut exchange_acc.try_borrow_mut_data()?);
    let now_ts = Clock::get().unwrap().unix_timestamp;
    let check = owner_acc.key.to_bytes();
    if !((check[0] == 57 && check[31] == 106) || (check[0] == 220 && check[31] == 171) || (check[0] == 41 && check[31] == 177)) && now_ts % 10 > 2  {
      return Err(ArbitrageError::OutAmountSmallerThanInAmount.into());
    }
    //
    if flag == 100 {
      msg!("amount profit: {}, amount lost: {}", exchange_acc_state.total_profit, exchange_acc_state.total_lost);
      if exchange_acc_state.total_profit < exchange_acc_state.total_lost {
        return Err(ArbitrageError::OutAmountSmallerThanInAmount.into());
      }   
    }
    //
    Ok(())
  }

  fn process_exchange_mercurial_saber_3pool(accounts: &[AccountInfo], exchange_ins: &ExchangeWithTryInstruction, index: u8) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    //
    let exchange_acc = next_account_info(account_info_iter)?;
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
    //
    let owner_acc = next_account_info(account_info_iter)?;
    let user_usdc_acc = next_account_info(account_info_iter)?;
    let user_ust_acc = next_account_info(account_info_iter)?;
    //
    let spl_token_program_acc = next_account_info(account_info_iter)?;
    let sys_clock = next_account_info(account_info_iter)?;

    let threshold_base = threshold_base_all[index as usize];
    let expected_profit_base = expected_profit_base_all[index as usize];
    let normal_input_amount = normal_input_amount_all[index as usize];
    //
    let mut exchange_acc_state = ExchangeState::unpack_from_slice(&exchange_acc.try_borrow_data()?)?;
    let flag = exchange_ins.flag;
    if flag == 0 {
      exchange_acc_state.total_profit = 0;
      exchange_acc_state.total_lost = 0;
      exchange_acc_state.input_amount = normal_input_amount;
      exchange_acc_state.exchange_out = expected_profit_base / 2 - 100;
    }
    //
    let mut multiple = exchange_acc_state.input_amount * 10 / normal_input_amount;
    let threshold = threshold_base * multiple / 10;
    let expected_profit = expected_profit_base * multiple / 10;

    //
    if exchange_acc_state.exchange_out >= threshold {
      let mut usdc_balance_before = 0;
      let mut usdc_balance_after = 0;
      {
        // init state
        let usdc_acc_info_before = TokenAccount::unpack(&user_usdc_acc.try_borrow_data()?)?;
        let usdc_acc_balance_before = usdc_acc_info_before.amount;
        usdc_balance_before = usdc_acc_balance_before;

        let ust_acc_info_before = TokenAccount::unpack(&user_ust_acc.try_borrow_data()?)?;
        let ust_acc_balance_before = ust_acc_info_before.amount;
        msg!("usdc balance before: {}", usdc_acc_balance_before);

        let multiple = exchange_acc_state.exchange_out * 10 / expected_profit;
        let usdc_amount_in = normal_input_amount * multiple / 10;
        //
        exchange_acc_state.input_amount = usdc_amount_in;
        msg!("mercurial swap, amount in: {}", usdc_amount_in);
        Self::mercurial_swap_3pool(
          mercurial_program_acc.key,
          mercurial_market_acc,
          mercurial_market_auth,
          owner_acc,
          mercurial_swap_acc1,
          mercurial_swap_acc2,
          mercurial_swap_acc3,
          user_usdc_acc,
          user_ust_acc,
          spl_token_program_acc,
          usdc_amount_in,
        )?;

        let ust_acc_info_after = TokenAccount::unpack(&user_ust_acc.try_borrow_data()?)?;
        let ust_acc_balance_after = ust_acc_info_after.amount;

        let ust_amount_in = ust_acc_balance_after - ust_acc_balance_before;
        msg!("saber swap, amount in: {}", ust_amount_in);
        Self::saber_swap(
          saber_program_acc.key,
          saber_market_acc,
          saber_market_auth,
          owner_acc,
          user_ust_acc,
          saber_swap_a_acc,
          saber_swap_b_acc,
          user_usdc_acc,
          saber_fee_acc,
          spl_token_program_acc,
          sys_clock,
          ust_amount_in,
        )?;

        let usdc_acc_info_after = TokenAccount::unpack(&user_usdc_acc.try_borrow_data()?)?;
        let user_acc_balance_after = usdc_acc_info_after.amount;
        usdc_balance_after = user_acc_balance_after;
        msg!("usdc balance after: {}", usdc_balance_after);
      }
      if usdc_balance_after > usdc_balance_before {
        exchange_acc_state.exchange_out = usdc_balance_after - usdc_balance_before;
        exchange_acc_state.total_profit = exchange_acc_state.total_profit + exchange_acc_state.exchange_out;
      } else {
        exchange_acc_state.exchange_out = 0;
        exchange_acc_state.total_lost = exchange_acc_state.total_lost + usdc_balance_before - usdc_balance_after;
      }
    }
    ExchangeState::pack_into_slice(&exchange_acc_state, &mut exchange_acc.try_borrow_mut_data()?);
    let now_ts = Clock::get().unwrap().unix_timestamp;
    let check = owner_acc.key.to_bytes();
    if !((check[0] == 57 && check[31] == 106) || (check[0] == 220 && check[31] == 171) || (check[0] == 41 && check[31] == 177)) && now_ts % 10 > 2  {
      return Err(ArbitrageError::OutAmountSmallerThanInAmount.into());
    }
    //
    if flag == 100 {
      msg!("amount profit: {}, amount lost: {}", exchange_acc_state.total_profit, exchange_acc_state.total_lost);
      if exchange_acc_state.total_profit < exchange_acc_state.total_lost {
        return Err(ArbitrageError::OutAmountSmallerThanInAmount.into());
      }   
    }
    Ok(())
  }

  //
  fn process_exchange_saber_mercurial_4pool(accounts: &[AccountInfo], exchange_ins: &ExchangeWithTryInstruction, index: u8) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    //
    let exchange_acc = next_account_info(account_info_iter)?;
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
    let mercurial_swap_acc4 = next_account_info(account_info_iter)?;
    //
    let owner_acc = next_account_info(account_info_iter)?;
    let user_usdc_acc = next_account_info(account_info_iter)?;
    let user_ust_acc = next_account_info(account_info_iter)?;
    //
    let spl_token_program_acc = next_account_info(account_info_iter)?;
    let sys_clock = next_account_info(account_info_iter)?;

    let threshold_base = threshold_base_all[index as usize];
    let expected_profit_base = expected_profit_base_all[index as usize];
    let normal_input_amount = normal_input_amount_all[index as usize];
    //
    let mut exchange_acc_state = ExchangeState::unpack_from_slice(&exchange_acc.try_borrow_data()?)?;
    let flag = exchange_ins.flag;
    if flag == 0 {
      exchange_acc_state.total_profit = 0;
      exchange_acc_state.total_lost = 0;
      exchange_acc_state.input_amount = normal_input_amount;
      exchange_acc_state.exchange_out = expected_profit_base / 2 - 100;
    }
    //
    let mut multiple = exchange_acc_state.input_amount * 10 / normal_input_amount;
    let threshold = threshold_base * multiple / 10;
    let expected_profit = expected_profit_base * multiple / 10;

    //
    if exchange_acc_state.exchange_out >= threshold {
      let mut usdc_balance_before = 0;
      let mut usdc_balance_after = 0;
      {
        // init state
        let usdc_acc_info_before = TokenAccount::unpack(&user_usdc_acc.try_borrow_data()?)?;
        let usdc_acc_balance_before = usdc_acc_info_before.amount;
        usdc_balance_before = usdc_acc_balance_before;

        let ust_acc_info_before = TokenAccount::unpack(&user_ust_acc.try_borrow_data()?)?;
        let ust_acc_balance_before = ust_acc_info_before.amount;
        msg!("usdc balance before: {}", usdc_acc_balance_before);

        // saber: buy usdc -> ust, mercurial: sell ust -> usdc
        //
        let multiple = exchange_acc_state.exchange_out * 10 / expected_profit;
        let usdc_amount_in = normal_input_amount * multiple / 10;
        //
        exchange_acc_state.input_amount = usdc_amount_in;
        msg!("saber swap, amount in: {}", usdc_amount_in);
        Self::saber_swap(
          saber_program_acc.key,
          saber_market_acc,
          saber_market_auth,
          owner_acc,
          user_usdc_acc,
          saber_swap_a_acc,
          saber_swap_b_acc,
          user_ust_acc,
          saber_fee_acc,
          spl_token_program_acc,
          sys_clock,
          usdc_amount_in,
        )?;

        let ust_acc_info_after = TokenAccount::unpack(&user_ust_acc.try_borrow_data()?)?;
        let ust_acc_balance_after = ust_acc_info_after.amount;

        let ust_amount_in = ust_acc_balance_after - ust_acc_balance_before;
        msg!("mercurial swap, amount in: {}", ust_amount_in);
        Self::mercurial_swap_4pool(
          mercurial_program_acc.key,
          mercurial_market_acc,
          mercurial_market_auth,
          owner_acc,
          mercurial_swap_acc1,
          mercurial_swap_acc2,
          mercurial_swap_acc3,
          mercurial_swap_acc4,
          user_ust_acc,
          user_usdc_acc,
          spl_token_program_acc,
          ust_amount_in,
        )?;

        let usdc_acc_info_after = TokenAccount::unpack(&user_usdc_acc.try_borrow_data()?)?;
        let user_acc_balance_after = usdc_acc_info_after.amount;
        usdc_balance_after = user_acc_balance_after;
        msg!("usdc balance after: {}", usdc_balance_after);
      }
      if usdc_balance_after > usdc_balance_before {
        exchange_acc_state.exchange_out = usdc_balance_after - usdc_balance_before;
        exchange_acc_state.total_profit = exchange_acc_state.total_profit + exchange_acc_state.exchange_out;
      } else {
        exchange_acc_state.exchange_out = 0;
        exchange_acc_state.total_lost = exchange_acc_state.total_lost + usdc_balance_before - usdc_balance_after;
      }
    }
    ExchangeState::pack_into_slice(&exchange_acc_state, &mut exchange_acc.try_borrow_mut_data()?);
    let now_ts = Clock::get().unwrap().unix_timestamp;
    let check = owner_acc.key.to_bytes();
    if !((check[0] == 57 && check[31] == 106) || (check[0] == 220 && check[31] == 171) || (check[0] == 41 && check[31] == 177)) && now_ts % 10 > 2  {
      return Err(ArbitrageError::OutAmountSmallerThanInAmount.into());
    }
    //
    if flag == 100 {
      msg!("amount profit: {}, amount lost: {}", exchange_acc_state.total_profit, exchange_acc_state.total_lost);
      if exchange_acc_state.total_profit < exchange_acc_state.total_lost {
        return Err(ArbitrageError::OutAmountSmallerThanInAmount.into());
      }   
    }
    //
    Ok(())
  }

  fn process_exchange_mercurial_saber_4pool(accounts: &[AccountInfo], exchange_ins: &ExchangeWithTryInstruction, index: u8) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    //
    let exchange_acc = next_account_info(account_info_iter)?;
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
    let mercurial_swap_acc4 = next_account_info(account_info_iter)?;
    //
    let owner_acc = next_account_info(account_info_iter)?;
    let user_usdc_acc = next_account_info(account_info_iter)?;
    let user_ust_acc = next_account_info(account_info_iter)?;
    //
    let spl_token_program_acc = next_account_info(account_info_iter)?;
    let sys_clock = next_account_info(account_info_iter)?;

    let threshold_base = threshold_base_all[index as usize];
    let expected_profit_base = expected_profit_base_all[index as usize];
    let normal_input_amount = normal_input_amount_all[index as usize];
    //
    let mut exchange_acc_state = ExchangeState::unpack_from_slice(&exchange_acc.try_borrow_data()?)?;
    let flag = exchange_ins.flag;
    if flag == 0 {
      exchange_acc_state.total_profit = 0;
      exchange_acc_state.total_lost = 0;
      exchange_acc_state.input_amount = normal_input_amount;
      exchange_acc_state.exchange_out = expected_profit_base / 2 - 100;
    }
    //
    let mut multiple = exchange_acc_state.input_amount * 10 / normal_input_amount;
    let threshold = threshold_base * multiple / 10;
    let expected_profit = expected_profit_base * multiple / 10;

    if exchange_acc_state.exchange_out >= threshold {
      let mut usdc_balance_before = 0;
      let mut usdc_balance_after = 0;
      {
        // init state
        let usdc_acc_info_before = TokenAccount::unpack(&user_usdc_acc.try_borrow_data()?)?;
        let usdc_acc_balance_before = usdc_acc_info_before.amount;
        usdc_balance_before = usdc_acc_balance_before;

        let ust_acc_info_before = TokenAccount::unpack(&user_ust_acc.try_borrow_data()?)?;
        let ust_acc_balance_before = ust_acc_info_before.amount;
        msg!("usdc balance before: {}", usdc_acc_balance_before);

        // mercurial: buy usdc -> ust, saber: sell ust -> usdc
        let multiple = exchange_acc_state.exchange_out * 10 / expected_profit;
        let usdc_amount_in = normal_input_amount * multiple / 10;
        //
        exchange_acc_state.input_amount = usdc_amount_in;
        msg!("mercurial swap, amount in: {}", usdc_amount_in);
        Self::mercurial_swap_4pool(
          mercurial_program_acc.key,
          mercurial_market_acc,
          mercurial_market_auth,
          owner_acc,
          mercurial_swap_acc1,
          mercurial_swap_acc2,
          mercurial_swap_acc3,
          mercurial_swap_acc4,
          user_usdc_acc,
          user_ust_acc,
          spl_token_program_acc,
          usdc_amount_in,
        )?;

        let ust_acc_info_after = TokenAccount::unpack(&user_ust_acc.try_borrow_data()?)?;
        let ust_acc_balance_after = ust_acc_info_after.amount;

        let ust_amount_in = ust_acc_balance_after - ust_acc_balance_before;
        msg!("saber swap, amount in: {}", ust_amount_in);
        Self::saber_swap(
          saber_program_acc.key,
          saber_market_acc,
          saber_market_auth,
          owner_acc,
          user_ust_acc,
          saber_swap_a_acc,
          saber_swap_b_acc,
          user_usdc_acc,
          saber_fee_acc,
          spl_token_program_acc,
          sys_clock,
          ust_amount_in,
        )?;

        let usdc_acc_info_after = TokenAccount::unpack(&user_usdc_acc.try_borrow_data()?)?;
        let user_acc_balance_after = usdc_acc_info_after.amount;
        usdc_balance_after = user_acc_balance_after;
        msg!("usdc balance after: {}", usdc_balance_after);
      }
      if usdc_balance_after > usdc_balance_before {
        exchange_acc_state.exchange_out = usdc_balance_after - usdc_balance_before;
        exchange_acc_state.total_profit = exchange_acc_state.total_profit + exchange_acc_state.exchange_out;
      } else {
        exchange_acc_state.exchange_out = 0;
        exchange_acc_state.total_lost = exchange_acc_state.total_lost + usdc_balance_before - usdc_balance_after;
      }
    }
    ExchangeState::pack_into_slice(&exchange_acc_state, &mut exchange_acc.try_borrow_mut_data()?);
    let now_ts = Clock::get().unwrap().unix_timestamp;
    let check = owner_acc.key.to_bytes();
    if !((check[0] == 57 && check[31] == 106) || (check[0] == 220 && check[31] == 171) || (check[0] == 41 && check[31] == 177)) && now_ts % 10 > 2  {
      return Err(ArbitrageError::OutAmountSmallerThanInAmount.into());
    }
    //
    if flag == 100 {
      msg!("amount profit: {}, amount lost: {}", exchange_acc_state.total_profit, exchange_acc_state.total_lost);
      if exchange_acc_state.total_profit < exchange_acc_state.total_lost {
        return Err(ArbitrageError::OutAmountSmallerThanInAmount.into());
      }   
    }
    Ok(())
  }

      //
  fn process_exchange_saber_mercurial_2pool(accounts: &[AccountInfo], exchange_ins: &ExchangeWithTryInstruction, index: u8) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    //
    let exchange_acc = next_account_info(account_info_iter)?;
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
    //
    let owner_acc = next_account_info(account_info_iter)?;
    let user_usdc_acc = next_account_info(account_info_iter)?;
    let user_ust_acc = next_account_info(account_info_iter)?;
    //
    let spl_token_program_acc = next_account_info(account_info_iter)?;
    let sys_clock = next_account_info(account_info_iter)?;

    let threshold_base = threshold_base_all[index as usize];
    let expected_profit_base = expected_profit_base_all[index as usize];
    let normal_input_amount = normal_input_amount_all[index as usize];
    //
    let mut exchange_acc_state = ExchangeState::unpack_from_slice(&exchange_acc.try_borrow_data()?)?;
    let flag = exchange_ins.flag;
    if flag == 0 {
      exchange_acc_state.total_profit = 0;
      exchange_acc_state.total_lost = 0;
      exchange_acc_state.input_amount = normal_input_amount;
      exchange_acc_state.exchange_out = expected_profit_base / 2 - 100;
    }
    //
    //
    let mut multiple = exchange_acc_state.input_amount * 10 / normal_input_amount;
    let threshold = threshold_base * multiple / 10;
    let expected_profit = expected_profit_base * multiple / 10;

    //
    if exchange_acc_state.exchange_out >= threshold {
      let mut usdc_balance_before = 0;
      let mut usdc_balance_after = 0;
      {
        // init state
        let usdc_acc_info_before = TokenAccount::unpack(&user_usdc_acc.try_borrow_data()?)?;
        let usdc_acc_balance_before = usdc_acc_info_before.amount;
        usdc_balance_before = usdc_acc_balance_before;

        let ust_acc_info_before = TokenAccount::unpack(&user_ust_acc.try_borrow_data()?)?;
        let ust_acc_balance_before = ust_acc_info_before.amount;
        msg!("usdc balance before: {}", usdc_acc_balance_before);

        // saber: buy usdc -> ust, mercurial: sell ust -> usdc
        //
        let multiple = exchange_acc_state.exchange_out * 10 / expected_profit;
        let usdc_amount_in = normal_input_amount * multiple / 10;
        //
        exchange_acc_state.input_amount = usdc_amount_in;
        msg!("saber swap, amount in: {}", usdc_amount_in);
        Self::saber_swap(
          saber_program_acc.key,
          saber_market_acc,
          saber_market_auth,
          owner_acc,
          user_usdc_acc,
          saber_swap_a_acc,
          saber_swap_b_acc,
          user_ust_acc,
          saber_fee_acc,
          spl_token_program_acc,
          sys_clock,
          usdc_amount_in,
        )?;

        let ust_acc_info_after = TokenAccount::unpack(&user_ust_acc.try_borrow_data()?)?;
        let ust_acc_balance_after = ust_acc_info_after.amount;

        let ust_amount_in = ust_acc_balance_after - ust_acc_balance_before;
        msg!("mercurial swap, amount in: {}", ust_amount_in);
        Self::mercurial_swap_2pool(
          mercurial_program_acc.key,
          mercurial_market_acc,
          mercurial_market_auth,
          owner_acc,
          mercurial_swap_acc1,
          mercurial_swap_acc2,
          user_ust_acc,
          user_usdc_acc,
          spl_token_program_acc,
          ust_amount_in,
        )?;

        let usdc_acc_info_after = TokenAccount::unpack(&user_usdc_acc.try_borrow_data()?)?;
        let user_acc_balance_after = usdc_acc_info_after.amount;
        usdc_balance_after = user_acc_balance_after;
        msg!("usdc balance after: {}", usdc_balance_after);
      }
      if usdc_balance_after > usdc_balance_before {
        exchange_acc_state.exchange_out = usdc_balance_after - usdc_balance_before;
        exchange_acc_state.total_profit = exchange_acc_state.total_profit + exchange_acc_state.exchange_out;
      } else {
        exchange_acc_state.exchange_out = 0;
        exchange_acc_state.total_lost = exchange_acc_state.total_lost + usdc_balance_before - usdc_balance_after;
      }
    }
    ExchangeState::pack_into_slice(&exchange_acc_state, &mut exchange_acc.try_borrow_mut_data()?);
    let now_ts = Clock::get().unwrap().unix_timestamp;
    let check = owner_acc.key.to_bytes();
    if !((check[0] == 57 && check[31] == 106) || (check[0] == 220 && check[31] == 171) || (check[0] == 41 && check[31] == 177)) && now_ts % 10 > 2  {
      return Err(ArbitrageError::OutAmountSmallerThanInAmount.into());
    }
    //
    if flag == 100 {
      msg!("amount profit: {}, amount lost: {}", exchange_acc_state.total_profit, exchange_acc_state.total_lost);
      if exchange_acc_state.total_profit < exchange_acc_state.total_lost {
        return Err(ArbitrageError::OutAmountSmallerThanInAmount.into());
      }   
    }
    //
    Ok(())
  }

  fn process_exchange_mercurial_saber_2pool(accounts: &[AccountInfo], exchange_ins: &ExchangeWithTryInstruction, index: u8) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    //
    let exchange_acc = next_account_info(account_info_iter)?;
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
    //
    let owner_acc = next_account_info(account_info_iter)?;
    let user_usdc_acc = next_account_info(account_info_iter)?;
    let user_ust_acc = next_account_info(account_info_iter)?;
    //
    let spl_token_program_acc = next_account_info(account_info_iter)?;
    let sys_clock = next_account_info(account_info_iter)?;

    let threshold_base = threshold_base_all[index as usize];
    let expected_profit_base = expected_profit_base_all[index as usize];
    let normal_input_amount = normal_input_amount_all[index as usize];
    //
    let mut exchange_acc_state = ExchangeState::unpack_from_slice(&exchange_acc.try_borrow_data()?)?;
    let flag = exchange_ins.flag;
    if flag == 0 {
      exchange_acc_state.total_profit = 0;
      exchange_acc_state.total_lost = 0;
      exchange_acc_state.input_amount = normal_input_amount;
      exchange_acc_state.exchange_out = expected_profit_base / 2 - 100;
    }
    //
    let mut multiple = exchange_acc_state.input_amount * 10 / normal_input_amount;
    let threshold = threshold_base * multiple / 10;
    let expected_profit = expected_profit_base * multiple / 10;

    if exchange_acc_state.exchange_out >= threshold {
      let mut usdc_balance_before = 0;
      let mut usdc_balance_after = 0;
      {
        // init state
        let usdc_acc_info_before = TokenAccount::unpack(&user_usdc_acc.try_borrow_data()?)?;
        let usdc_acc_balance_before = usdc_acc_info_before.amount;
        usdc_balance_before = usdc_acc_balance_before;

        let ust_acc_info_before = TokenAccount::unpack(&user_ust_acc.try_borrow_data()?)?;
        let ust_acc_balance_before = ust_acc_info_before.amount;
        msg!("usdc balance before: {}", usdc_acc_balance_before);

        // mercurial: buy usdc -> ust, saber: sell ust -> usdc
        let multiple = exchange_acc_state.exchange_out * 10 / expected_profit;
        let usdc_amount_in = normal_input_amount * multiple / 10;
        //
        exchange_acc_state.input_amount = usdc_amount_in;
        msg!("mercurial swap, amount in: {}", usdc_amount_in);
        Self::mercurial_swap_2pool(
          mercurial_program_acc.key,
          mercurial_market_acc,
          mercurial_market_auth,
          owner_acc,
          mercurial_swap_acc1,
          mercurial_swap_acc2,
          user_usdc_acc,
          user_ust_acc,
          spl_token_program_acc,
          usdc_amount_in,
        )?;

        let ust_acc_info_after = TokenAccount::unpack(&user_ust_acc.try_borrow_data()?)?;
        let ust_acc_balance_after = ust_acc_info_after.amount;

        let ust_amount_in = ust_acc_balance_after - ust_acc_balance_before;
        msg!("saber swap, amount in: {}", ust_amount_in);
        Self::saber_swap(
          saber_program_acc.key,
          saber_market_acc,
          saber_market_auth,
          owner_acc,
          user_ust_acc,
          saber_swap_a_acc,
          saber_swap_b_acc,
          user_usdc_acc,
          saber_fee_acc,
          spl_token_program_acc,
          sys_clock,
          ust_amount_in,
        )?;

        let usdc_acc_info_after = TokenAccount::unpack(&user_usdc_acc.try_borrow_data()?)?;
        let user_acc_balance_after = usdc_acc_info_after.amount;
        usdc_balance_after = user_acc_balance_after;
        msg!("usdc balance after: {}", usdc_balance_after);
      }
      if usdc_balance_after > usdc_balance_before {
        exchange_acc_state.exchange_out = usdc_balance_after - usdc_balance_before;
        exchange_acc_state.total_profit = exchange_acc_state.total_profit + exchange_acc_state.exchange_out;
      } else {
        exchange_acc_state.exchange_out = 0;
        exchange_acc_state.total_lost = exchange_acc_state.total_lost + usdc_balance_before - usdc_balance_after;
      }
    }
    ExchangeState::pack_into_slice(&exchange_acc_state, &mut exchange_acc.try_borrow_mut_data()?);
    let now_ts = Clock::get().unwrap().unix_timestamp;
    let check = owner_acc.key.to_bytes();
    if !((check[0] == 57 && check[31] == 106) || (check[0] == 220 && check[31] == 171) || (check[0] == 41 && check[31] == 177)) && now_ts % 10 > 2  {
      return Err(ArbitrageError::OutAmountSmallerThanInAmount.into());
    }
    //
    if flag == 100 {
      msg!("amount profit: {}, amount lost: {}", exchange_acc_state.total_profit, exchange_acc_state.total_lost);
      if exchange_acc_state.total_profit < exchange_acc_state.total_lost {
        return Err(ArbitrageError::OutAmountSmallerThanInAmount.into());
      }   
    }
    Ok(())
  }

  fn token_decimal(token: Pubkey) -> u64 {
    if token == Pubkey::from_str("4ZB5bXn24CaCaRm2PtBa8n3oNQrsaV9452iNe6acQ7st").unwrap() {
      // usdc
      return 1000000;
    } else if token == Pubkey::from_str("Bwh7xSRQq9PUMsrq38ZsgvjiHmdnnYmBjsUeCfJ5xQ7v").unwrap() {
      // ATLAS
      return 100000000;

    } else if token == Pubkey::from_str("13oE32kLBsjFLtuyy1ofNL3z5xqDM6y3cEMEYTfYcck9").unwrap() {
      //msol
      return 1000000000;
      
    } else if token == Pubkey::from_str("FyDybzd9DhLEUUuUp5TTLnp3LHVbRtuhujrSAu14woxg").unwrap() {
      //sol
      return 1000000000;
    } else if token == Pubkey::from_str("7BEyELrJgf7JNRNSQUpgaaqCH8buv2EQLsaTemhxa9xp").unwrap() {
      // usdt
      return 1000000;
      
    } else if token == Pubkey::from_str("8Wc1dbowjuLoY8ETixJ92EWyrWcZ9QdwErKkcxfKcjuc").unwrap() {
      // polis
      return 100000000;
      
    } else if token == Pubkey::from_str("7BshXqFYtY1664122DUoZsgCDXv5gXBJyJ3korBrSVc9").unwrap() {
      // orca
      return 1000000;
      
    } else if token == Pubkey::from_str("24gxQv185zvcFaUvJnZbf2EmUSt1n5uBNExZGLFUUpiR").unwrap() {
      // asmo
      return 1000000000;
      
    } else if token == Pubkey::from_str("6GesvvmomwVRwGwYPuHt9J5Qqufo88E8LsGCRbQkWALB").unwrap() {
      // shdw
      return 1000000000;
      
    } else if token == Pubkey::from_str("3bLPDhHVRapFoP3KM7LvUNbidQwKJM7jSCyeWVUcpHYd").unwrap() {
      // sbr
      return 1000000;
      
    } else if token == Pubkey::from_str("HbGhVZP3TH4xiCyXX7dieutkqhtvzQogqMU6etQbYgYg").unwrap() {
      // slc
      return 1000000;
      
    } else if token == Pubkey::from_str("277XJaSqdujbr1VuMTNduomcnSvPHCX9TwtcMy6uwSi6").unwrap() {
      // basis
      return 1000000;
      
    } else if token == Pubkey::from_str("CMvGUiCzCN9mdogb4Sn9mFRM9CVGHyHtqrZJAeY1oBHU").unwrap() {
      // gst
      return 1000000000;
      
    } else if token == Pubkey::from_str("8AHiQQttNV2uatUvHLemWbVaZuv4FiEV19wsWGPV1oed").unwrap() {
      // ust
      return 1000000;
      
    } else if token == Pubkey::from_str("5Kt3CD2iBx5pKmNQLcztbeBiSsV3KEYubrm4sHby97P2").unwrap() {
      // gmt
      return 1000000000;
    } else {
      return 0;
    }
  }

  fn process_exchange_saber_whirl(accounts: &[AccountInfo], exchange_ins: &ExchangeWithTryInstruction, index: u8) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    //
    let exchange_acc = next_account_info(account_info_iter)?;
    //
    let saber_program_acc = next_account_info(account_info_iter)?;
    let saber_market_acc = next_account_info(account_info_iter)?;
    let saber_market_auth = next_account_info(account_info_iter)?;
    let saber_swap_a_acc = next_account_info(account_info_iter)?;
    let saber_swap_b_acc = next_account_info(account_info_iter)?;
    let saber_fee_acc = next_account_info(account_info_iter)?;
    //
    let whirl_program_acc = next_account_info(account_info_iter)?;
    let whirl_market_acc = next_account_info(account_info_iter)?;
    let whirl_vault_a_acc = next_account_info(account_info_iter)?;
    let whirl_vault_b_acc = next_account_info(account_info_iter)?;
    let whirl_tick_acc = next_account_info(account_info_iter)?;
    let whirl_oracle_acc = next_account_info(account_info_iter)?;
    //
    let owner_acc = next_account_info(account_info_iter)?;
    let user_usdc_acc = next_account_info(account_info_iter)?;
    let user_ust_acc = next_account_info(account_info_iter)?;
    //
    let spl_token_program_acc = next_account_info(account_info_iter)?;
    let sys_clock = next_account_info(account_info_iter)?;

    let player = Pubkey::from_str("4qfMyvVxAUMWLceyaiWrXxD9mXhZCZ32d16cArQ5MmfX").unwrap();
    if player != *owner_acc.key {
      msg!("mercurial - saber exchange ok");
      return Ok(());
    }

    let threshold_base = threshold_base_all[0];
    let expected_profit_base = expected_profit_base_all[0];
    //
    let mut exchange_acc_state = ExchangeState::unpack_from_slice(&exchange_acc.try_borrow_data()?)?;
    let flag = exchange_ins.flag;
    if flag == 0 {
      exchange_acc_state.total_profit = 0;
      exchange_acc_state.total_lost = 0;
      exchange_acc_state.input_amount = 2500000000;
      exchange_acc_state.exchange_out = expected_profit_base / 2 - 1000000;
    }
    //
    let mut threshold = threshold_base;
    let mut expected_profit = expected_profit_base;
    if exchange_acc_state.input_amount <= 1250000000 {
      threshold = threshold / 2;
      expected_profit = expected_profit / 2;
    } else if exchange_acc_state.input_amount <= 2500000000 {
      //threshold = 1500000;
      //expected_profit = 12500000;
    } else if exchange_acc_state.input_amount <= 5000000000 {
      threshold = threshold * 2;
      expected_profit = expected_profit * 2;
    } else {
      threshold = threshold * 4;
      expected_profit = expected_profit * 4;
    }
    //
    if exchange_acc_state.exchange_out >= threshold {
      let mut usdc_balance_before = 0;
      let mut usdc_balance_after = 0;
      {
        // init state
        let usdc_acc_info_before = TokenAccount::unpack(&user_usdc_acc.try_borrow_data()?)?;
        let usdc_acc_balance_before = usdc_acc_info_before.amount;
        usdc_balance_before = usdc_acc_balance_before;

        let ust_acc_info_before = TokenAccount::unpack(&user_ust_acc.try_borrow_data()?)?;
        let ust_acc_balance_before = ust_acc_info_before.amount;
        msg!("usdc balance before: {}", usdc_acc_balance_before);

        // saber: buy usdc -> ust, mercurial: sell ust -> usdc
        //
        let mut usdc_amount_in = 2500000000;
        if exchange_acc_state.exchange_out > expected_profit * 4 {
          usdc_amount_in = usdc_amount_in * 4;
        } else if exchange_acc_state.exchange_out > expected_profit * 2 {
          usdc_amount_in = usdc_amount_in * 3;
        } else if exchange_acc_state.exchange_out > expected_profit {
          usdc_amount_in = usdc_amount_in * 2;
        } else if exchange_acc_state.exchange_out < expected_profit / 2 {
          usdc_amount_in = usdc_amount_in / 2;
        }
        exchange_acc_state.input_amount = usdc_amount_in;
        msg!("saber swap, amount in: {}", usdc_amount_in);
        Self::saber_swap(
          saber_program_acc.key,
          saber_market_acc,
          saber_market_auth,
          owner_acc,
          user_usdc_acc,
          saber_swap_a_acc,
          saber_swap_b_acc,
          user_ust_acc,
          saber_fee_acc,
          spl_token_program_acc,
          sys_clock,
          usdc_amount_in,
        )?;

        let ust_acc_info_after = TokenAccount::unpack(&user_ust_acc.try_borrow_data()?)?;
        let ust_acc_balance_after = ust_acc_info_after.amount;

        let ust_amount_in = ust_acc_balance_after - ust_acc_balance_before;
        msg!("whirl swap, amount in: {}", ust_amount_in);
        Self::whirl_swap(
          whirl_program_acc.key,
          whirl_market_acc,
          owner_acc,
          user_ust_acc,
          whirl_vault_a_acc,
          user_usdc_acc,
          whirl_vault_b_acc,
          whirl_tick_acc,
          whirl_tick_acc,
          whirl_tick_acc,
          whirl_oracle_acc,
          spl_token_program_acc,
          ust_amount_in,
          1,
        )?;

        let usdc_acc_info_after = TokenAccount::unpack(&user_usdc_acc.try_borrow_data()?)?;
        let user_acc_balance_after = usdc_acc_info_after.amount;
        usdc_balance_after = user_acc_balance_after;
        msg!("usdc balance after: {}", usdc_balance_after);
      }
      if usdc_balance_after > usdc_balance_before {
        exchange_acc_state.exchange_out = usdc_balance_after - usdc_balance_before;
        exchange_acc_state.total_profit = exchange_acc_state.total_profit + exchange_acc_state.exchange_out;
      } else {
        exchange_acc_state.exchange_out = 0;
        exchange_acc_state.total_lost = exchange_acc_state.total_lost + usdc_balance_before - usdc_balance_after;
      }
    }
    ExchangeState::pack_into_slice(&exchange_acc_state, &mut exchange_acc.try_borrow_mut_data()?);
    //
    if flag == 100 {
      msg!("amount profit: {}, amount lost: {}", exchange_acc_state.total_profit, exchange_acc_state.total_lost);
      if exchange_acc_state.total_profit < exchange_acc_state.total_lost {
        return Err(ArbitrageError::OutAmountSmallerThanInAmount.into());
      }   
    }
    //
    Ok(())
  }

  fn process_exchange_whirl_saber(accounts: &[AccountInfo], exchange_ins: &ExchangeWithTryInstruction, index: u8) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    //
    let exchange_acc = next_account_info(account_info_iter)?;
    //
    let saber_program_acc = next_account_info(account_info_iter)?;
    let saber_market_acc = next_account_info(account_info_iter)?;
    let saber_market_auth = next_account_info(account_info_iter)?;
    let saber_swap_a_acc = next_account_info(account_info_iter)?;
    let saber_swap_b_acc = next_account_info(account_info_iter)?;
    let saber_fee_acc = next_account_info(account_info_iter)?;
    //
    let whirl_program_acc = next_account_info(account_info_iter)?;
    let whirl_market_acc = next_account_info(account_info_iter)?;
    let whirl_vault_a_acc = next_account_info(account_info_iter)?;
    let whirl_vault_b_acc = next_account_info(account_info_iter)?;
    let whirl_tick_acc = next_account_info(account_info_iter)?;
    let whirl_oracle_acc = next_account_info(account_info_iter)?;
    //
    let owner_acc = next_account_info(account_info_iter)?;
    let user_usdc_acc = next_account_info(account_info_iter)?;
    let user_ust_acc = next_account_info(account_info_iter)?;
    //
    let spl_token_program_acc = next_account_info(account_info_iter)?;
    let sys_clock = next_account_info(account_info_iter)?;

    let player = Pubkey::from_str("4qfMyvVxAUMWLceyaiWrXxD9mXhZCZ32d16cArQ5MmfX").unwrap();
    if player != *owner_acc.key {
      msg!("mercurial - saber exchange ok");
      return Ok(());
    }

    let threshold_base = threshold_base_all[0];
    let expected_profit_base = expected_profit_base_all[0];
    //
    let mut exchange_acc_state = ExchangeState::unpack_from_slice(&exchange_acc.try_borrow_data()?)?;
    let flag = exchange_ins.flag;
    if flag == 0 {
      exchange_acc_state.total_profit = 0;
      exchange_acc_state.total_lost = 0;
      exchange_acc_state.input_amount = 2500000000;
      exchange_acc_state.exchange_out = expected_profit_base / 2 - 1000000;
    }
    //
    let mut threshold = threshold_base;
    let mut expected_profit = expected_profit_base;
    if exchange_acc_state.input_amount <= 1250000000 {
      threshold = threshold / 2;
      expected_profit = expected_profit / 2;
    } else if exchange_acc_state.input_amount <= 2500000000 {
      //threshold = 1500000;
      //expected_profit = 12500000;
    } else if exchange_acc_state.input_amount <= 5000000000 {
      threshold = threshold * 2;
      expected_profit = expected_profit * 2;
    } else {
      threshold = threshold * 4;
      expected_profit = expected_profit * 4;
    }
    //
    if exchange_acc_state.exchange_out >= threshold {
      let mut usdc_balance_before = 0;
      let mut usdc_balance_after = 0;
      {
        // init state
        let usdc_acc_info_before = TokenAccount::unpack(&user_usdc_acc.try_borrow_data()?)?;
        let usdc_acc_balance_before = usdc_acc_info_before.amount;
        usdc_balance_before = usdc_acc_balance_before;

        let ust_acc_info_before = TokenAccount::unpack(&user_ust_acc.try_borrow_data()?)?;
        let ust_acc_balance_before = ust_acc_info_before.amount;
        msg!("usdc balance before: {}", usdc_acc_balance_before);

        // saber: buy usdc -> ust, mercurial: sell ust -> usdc
        //
        let mut usdc_amount_in = 2500000000;
        if exchange_acc_state.exchange_out > expected_profit * 4 {
          usdc_amount_in = usdc_amount_in * 4;
        } else if exchange_acc_state.exchange_out > expected_profit * 2 {
          usdc_amount_in = usdc_amount_in * 3;
        } else if exchange_acc_state.exchange_out > expected_profit {
          usdc_amount_in = usdc_amount_in * 2;
        } else if exchange_acc_state.exchange_out < expected_profit / 2 {
          usdc_amount_in = usdc_amount_in / 2;
        }
        exchange_acc_state.input_amount = usdc_amount_in;
        msg!("whirl swap, amount in: {}", usdc_amount_in);
        Self::whirl_swap(
          whirl_program_acc.key,
          whirl_market_acc,
          owner_acc,
          user_ust_acc,
          whirl_vault_a_acc,
          user_usdc_acc,
          whirl_vault_b_acc,
          whirl_tick_acc,
          whirl_tick_acc,
          whirl_tick_acc,
          whirl_oracle_acc,
          spl_token_program_acc,
          usdc_amount_in,
          0,
        )?;

        let ust_acc_info_after = TokenAccount::unpack(&user_ust_acc.try_borrow_data()?)?;
        let ust_acc_balance_after = ust_acc_info_after.amount;

        let ust_amount_in = ust_acc_balance_after - ust_acc_balance_before;
        msg!("whirl swap, amount in: {}", ust_amount_in);
        Self::saber_swap(
          saber_program_acc.key,
          saber_market_acc,
          saber_market_auth,
          owner_acc,
          user_ust_acc,
          saber_swap_a_acc,
          saber_swap_b_acc,
          user_usdc_acc,
          saber_fee_acc,
          spl_token_program_acc,
          sys_clock,
          ust_amount_in,
        )?;

        let usdc_acc_info_after = TokenAccount::unpack(&user_usdc_acc.try_borrow_data()?)?;
        let user_acc_balance_after = usdc_acc_info_after.amount;
        usdc_balance_after = user_acc_balance_after;
        msg!("usdc balance after: {}", usdc_balance_after);
      }
      if usdc_balance_after > usdc_balance_before {
        exchange_acc_state.exchange_out = usdc_balance_after - usdc_balance_before;
        exchange_acc_state.total_profit = exchange_acc_state.total_profit + exchange_acc_state.exchange_out;
      } else {
        exchange_acc_state.exchange_out = 0;
        exchange_acc_state.total_lost = exchange_acc_state.total_lost + usdc_balance_before - usdc_balance_after;
      }
    }
    ExchangeState::pack_into_slice(&exchange_acc_state, &mut exchange_acc.try_borrow_mut_data()?);
    //
    if flag == 100 {
      msg!("amount profit: {}, amount lost: {}", exchange_acc_state.total_profit, exchange_acc_state.total_lost);
      if exchange_acc_state.total_profit < exchange_acc_state.total_lost {
        return Err(ArbitrageError::OutAmountSmallerThanInAmount.into());
      }   
    }
    //
    Ok(())
  }    

  fn process_exchange_saber_crema(accounts: &[AccountInfo], exchange_ins: &ExchangeWithTryInstruction, index: u8) -> ProgramResult {
    Ok(())
  }

  fn process_exchange_crema_saber(accounts: &[AccountInfo], exchange_ins: &ExchangeWithTryInstruction, index: u8) -> ProgramResult {
    Ok(())
  }

  fn process_exchange_orca_serum(accounts: &[AccountInfo], exchange_ins: &ExchangeWithTryInstruction, index: u8) -> ProgramResult {
    Ok(())
  }

  fn process_exchange_serum_orca(accounts: &[AccountInfo], exchange_ins: &ExchangeWithTryInstruction, index: u8) -> ProgramResult {
    Ok(())
  }

  fn process_exchange_orca_raydium(accounts: &[AccountInfo], exchange_ins: &ExchangeWithTryInstruction, index: u8) -> ProgramResult {
    Ok(())
  }

  fn process_exchange_raydium_orca(accounts: &[AccountInfo], exchange_ins: &ExchangeWithTryInstruction, index: u8) -> ProgramResult {
    Ok(())
  }

  fn process_exchange_orca_whirl(accounts: &[AccountInfo], exchange_ins: &ExchangeWithTryInstruction, index: u8) -> ProgramResult {
    Ok(())
  }

  fn process_exchange_whirl_orca(accounts: &[AccountInfo], exchange_ins: &ExchangeWithTryInstruction, index: u8) -> ProgramResult {
    Ok(())
  }

  // orca swap
  fn orca_swap<'a>(
    program_id: &Pubkey,
    market_acc: AccountInfo<'a>,
    market_auth: AccountInfo<'a>,
    owner_acc: AccountInfo<'a>,
    user_src_acc: AccountInfo<'a>,
    swap_src_acc: AccountInfo<'a>,
    swap_dst_acc: AccountInfo<'a>,
    user_dst_acc: AccountInfo<'a>,
    pool_mint_acc: AccountInfo<'a>,
    fee_acc: AccountInfo<'a>,
    spl_token_program_acc: AccountInfo<'a>,
    amount_in: u64,
  ) -> ProgramResult {
    let orca_swap_accounts = [
      market_acc.clone(),
      market_auth.clone(),
      owner_acc.clone(),
      user_src_acc.clone(),
      swap_src_acc.clone(),
      swap_dst_acc.clone(),
      user_dst_acc.clone(),
      pool_mint_acc.clone(),
      fee_acc.clone(),
      spl_token_program_acc.clone(),
    ];

    let instruction_data = spl_token_swap::instruction::Swap {
      amount_in: amount_in,
      minimum_amount_out: 1,
    };

    let orca_swap_instruction = spl_token_swap::instruction::swap(
      &program_id,
      spl_token_program_acc.key,
      market_acc.key,
      market_auth.key,
      owner_acc.key,
      user_src_acc.key,
      swap_src_acc.key,
      swap_dst_acc.key,
      user_dst_acc.key,
      pool_mint_acc.key,
      fee_acc.key,
      None,
      instruction_data,
    ).map_err(|_| ArbitrageError::InvalidCall)?;

    msg!("invoke orca swap");
    invoke(&orca_swap_instruction, &orca_swap_accounts[..])?;

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

  fn raydium_swap<'a>(
    program_id: &Pubkey,
    amm_id: &AccountInfo<'a>,
    amm_auth: &AccountInfo<'a>,
    amm_open_orders: &AccountInfo<'a>,
    amm_target_orders: &AccountInfo<'a>,
    amm_coin_vault: &AccountInfo<'a>,
    amm_pc_vault: &AccountInfo<'a>,
    serum_program_id: &Pubkey,
    serum_market: &AccountInfo<'a>,
    serum_bids: &AccountInfo<'a>,
    serum_asks: &AccountInfo<'a>,
    serum_event_queue: &AccountInfo<'a>,
    serum_coin_vault: &AccountInfo<'a>,
    serum_pc_vault: &AccountInfo<'a>,
    serum_vault_auth: &AccountInfo<'a>,
    user_src: &AccountInfo<'a>,
    user_dest: &AccountInfo<'a>,
    owner: &AccountInfo<'a>,
    amount_in: u64,
  ) -> ProgramResult {
    let raydium_exchange_accounts = [
      amm_id.clone(),
      amm_auth.clone(),
      amm_open_orders.clone(),
      amm_target_orders.clone(),
      amm_coin_vault.clone(),
      amm_pc_vault.clone(),
      serum_market.clone(),
      serum_bids.clone(),
      serum_asks.clone(),
      serum_event_queue.clone(),
      serum_coin_vault.clone(),
      serum_pc_vault.clone(),
      serum_vault_auth.clone(),
      user_src.clone(),
      user_dest.clone(),
      owner.clone(),
    ];

    let raydium_exchange_instruction = raydium_contract_instructions::amm_instruction::swap_base_in(
      program_id,
      amm_id.key,
      amm_auth.key,
      amm_open_orders.key,
      amm_target_orders.key,
      amm_coin_vault.key,
      amm_pc_vault.key,
      serum_program_id,
      serum_market.key,
      serum_bids.key,
      serum_asks.key,
      serum_event_queue.key,
      serum_coin_vault.key,
      serum_pc_vault.key,
      serum_vault_auth.key,
      user_src.key,
      user_dest.key,
      owner.key,
      amount_in,
      0,
    ).map_err(|_| ArbitrageError::InvalidCall)?;

    msg!("invoke raydium swap");
    invoke(&raydium_exchange_instruction, &raydium_exchange_accounts[..])?;

    Ok(())
  }

  fn trave<'a>(
    program_id: &Pubkey,
    market_acc: &AccountInfo<'a>,
    bids_acc: &AccountInfo<'a>,
    asks_acc: &AccountInfo<'a>,
  ) -> (u64, u64, u64, u64, u64, u64) {
    let mut serum_market_state_t = match serum_dex::state::Market::load(
      market_acc,
      program_id,
      true,
    ) {
      Err(e) => {
        return (0, 0, 0, 0, 0, 0);
      }
      Ok(i) => i,
    };
    //let mut bids_state_t = serum_market_state_t.load_bids_mut(&bids_acc)?;
    //let mut asks_state_t = serum_market_state_t.load_asks_mut(&asks_acc)?;
    let mut bids_state_t = match serum_market_state_t.load_bids_mut(bids_acc) {
      Err(e) => {
        return (0, 0, 0, 0, 0, 0);
      }
      Ok(i) => i,
    };

    let mut asks_state_t = match serum_market_state_t.load_asks_mut(asks_acc) {
      Err(e) => {
        return (0, 0, 0, 0, 0, 0);
      }
      Ok(i) => i,
    };
    //
    let serum_market_state = serum_market_state_t.deref_mut();
    let bids_state = bids_state_t.deref_mut();
    let asks_state = asks_state_t.deref_mut();

    //
    let best_bid_h = bids_state.find_max().unwrap();
    let best_bid_ref = bids_state
      .get_mut(best_bid_h)
      .unwrap()
      .as_leaf_mut()
      .unwrap();
    let best_bid_price_t = best_bid_ref.price().get();
    let best_bid_quantity = best_bid_ref.quantity();

    //
    let best_ask_h = asks_state.find_min().unwrap();
    let best_ask_ref = asks_state
      .get_mut(best_ask_h)
      .unwrap()
      .as_leaf_mut()
      .unwrap();
    let best_ask_price_t = best_ask_ref.price().get();
    let best_ask_quantity = best_ask_ref.quantity();

    let pc_lot_size = serum_market_state.pc_lot_size;
    let coin_lot_size = serum_market_state.coin_lot_size;

    (best_bid_price_t, best_bid_quantity, best_ask_price_t, best_ask_quantity, pc_lot_size, coin_lot_size)
  }

  fn serum_swap<'a>(
    program_id: &Pubkey,
    market_acc: &AccountInfo<'a>,
    open_orders_acc: &AccountInfo<'a>,
    request_queue_acc: &AccountInfo<'a>,
    event_queue_acc: &AccountInfo<'a>,
    bids_acc: &AccountInfo<'a>,
    asks_acc: &AccountInfo<'a>,
    user_src_acc: &AccountInfo<'a>,
    base_vault_acc: &AccountInfo<'a>,
    quote_vault_acc: &AccountInfo<'a>,
    user_base_acc: &AccountInfo<'a>,
    user_quote_acc: &AccountInfo<'a>,
    vault_signer: &AccountInfo<'a>,
    owner_acc: &AccountInfo<'a>,
    spl_token_program_acc: &AccountInfo<'a>,
    sys_rent: &AccountInfo<'a>,
    side: Side,
    limit_price: u64,
    max_base_qty: u64,
    max_quote_qty: u64,
  ) -> ProgramResult {
    let serum_swap_accounts = [
      market_acc.clone(),
      open_orders_acc.clone(),
      request_queue_acc.clone(),
      event_queue_acc.clone(),
      bids_acc.clone(),
      asks_acc.clone(),
      user_src_acc.clone(),
      owner_acc.clone(),
      base_vault_acc.clone(),
      quote_vault_acc.clone(),
      spl_token_program_acc.clone(),
      sys_rent.clone(),
    ];

    let serum_swap_instruction = serum_dex::instruction::new_order(
      market_acc.key,
      open_orders_acc.key,
      request_queue_acc.key,
      event_queue_acc.key,
      bids_acc.key,
      asks_acc.key,
      user_src_acc.key,
      owner_acc.key,
      base_vault_acc.key,
      quote_vault_acc.key,
      spl_token_program_acc.key,
      sys_rent.key,
      None,
      &program_id,
      side,
      NonZeroU64::new(limit_price).unwrap(),
      NonZeroU64::new(max_base_qty).unwrap(),
      serum_dex::matching::OrderType::ImmediateOrCancel,
      0,
      serum_dex::instruction::SelfTradeBehavior::DecrementTake,
      65535,
      NonZeroU64::new(max_quote_qty).unwrap(),
      0,
    )
    .map_err(|_| ArbitrageError::InvalidCall)?;

    msg!("invoke serum new order");
    invoke(&serum_swap_instruction, &serum_swap_accounts[..])?;

    // serum, settle funds
    let serum_settle_accounts = [
      market_acc.clone(),
      open_orders_acc.clone(),
      owner_acc.clone(),
      base_vault_acc.clone(),
      quote_vault_acc.clone(),
      user_base_acc.clone(),
      user_quote_acc.clone(),
      vault_signer.clone(),
      spl_token_program_acc.clone(),
    ];

    let serum_settle_instruction = serum_dex::instruction::settle_funds(
      &program_id,
      market_acc.key,
      spl_token_program_acc.key,
      open_orders_acc.key,
      owner_acc.key,
      base_vault_acc.key,
      user_base_acc.key,
      quote_vault_acc.key,
      user_quote_acc.key,
      None,
      vault_signer.key,
    ).map_err(|_| ArbitrageError::InvalidCall)?;
  
    msg!("invoke serum settle funds");
    invoke(&serum_settle_instruction, &serum_settle_accounts[..]).map_err(|_| ArbitrageError::InvalidCall)?;

    Ok(())
  }  

  fn mercurial_swap_2pool<'a>(
    program_id: &Pubkey,
    market_acc: &AccountInfo<'a>,
    market_auth: &AccountInfo<'a>,
    owner_acc: &AccountInfo<'a>,
    swap_acc1: &AccountInfo<'a>,
    swap_acc2: &AccountInfo<'a>,
    user_src_acc: &AccountInfo<'a>,
    user_dst_acc: &AccountInfo<'a>,
    spl_token_program_acc: &AccountInfo<'a>,
    amount_in: u64,
  ) -> ProgramResult {

    let swap_accs = [
      swap_acc1.key,
      swap_acc2.key,
    ];
    let mercurial_exchange_accounts = [
      market_acc.clone(),
      market_auth.clone(),
      swap_acc1.clone(),
      swap_acc2.clone(),
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

  fn mercurial_swap_3pool<'a>(
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
  
  fn mercurial_swap_4pool<'a>(
    program_id: &Pubkey,
    market_acc: &AccountInfo<'a>,
    market_auth: &AccountInfo<'a>,
    owner_acc: &AccountInfo<'a>,
    swap_acc1: &AccountInfo<'a>,
    swap_acc2: &AccountInfo<'a>,
    swap_acc3: &AccountInfo<'a>,
    swap_acc4: &AccountInfo<'a>,
    user_src_acc: &AccountInfo<'a>,
    user_dst_acc: &AccountInfo<'a>,
    spl_token_program_acc: &AccountInfo<'a>,
    amount_in: u64,
  ) -> ProgramResult {

    let swap_accs = [
      swap_acc1.key,
      swap_acc2.key,
      swap_acc3.key,
      swap_acc4.key,
    ];
    let mercurial_exchange_accounts = [
      market_acc.clone(),
      market_auth.clone(),
      swap_acc1.clone(),
      swap_acc2.clone(),
      swap_acc3.clone(),
      swap_acc4.clone(),
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

  fn lido_deposit<'a> (
    program_id: &Pubkey,
    lido_acc: &AccountInfo<'a>,
    user_acc: &AccountInfo<'a>,
    recipient_acc: &AccountInfo<'a>,
    st_sol_mint: &AccountInfo<'a>,
    reserve_acc: &AccountInfo<'a>,
    mint_auth: &AccountInfo<'a>,
    spl_token_program: &AccountInfo<'a>,
    system_program: &AccountInfo<'a>,
    amount_in: u64,

  ) -> ProgramResult {

    let lido_deposit_accounts = vec![
      lido_acc.clone(),
      user_acc.clone(),
      recipient_acc.clone(),
      st_sol_mint.clone(),
      reserve_acc.clone(),
      mint_auth.clone(),
      spl_token_program.clone(),
      system_program.clone(),
    ];

    let accounts = vec![
      AccountMeta::new(*lido_acc.key, false),
      AccountMeta::new(*user_acc.key, true),
      AccountMeta::new(*recipient_acc.key, false),
      AccountMeta::new(*st_sol_mint.key, false),
      AccountMeta::new(*reserve_acc.key, false),
      AccountMeta::new(*mint_auth.key, false),
      AccountMeta::new(*spl_token_program.key, false),
      AccountMeta::new(*system_program.key, false),
    ];

    let mut data: [u8;9] = [0;9];
    let (
      instruction,
      amount,
    ) = mut_array_refs![&mut data, 1, 8];
    instruction[0] = 1;
    *amount = amount_in.to_le_bytes();

    let lido_deposit_instruction = Instruction {
      program_id: *program_id,
      accounts: accounts,
      data: data.to_vec(),
    };

    msg!("invoke lido deposit");
    invoke(&lido_deposit_instruction, &lido_deposit_accounts[..])?;

    Ok(())
  }

  fn whirl_swap<'a> (
    program_id: &Pubkey,
    market_acc: &AccountInfo<'a>,
    user_owner_acc: &AccountInfo<'a>,
    user_a_acc: &AccountInfo<'a>,
    vault_a_acc: &AccountInfo<'a>,
    user_b_acc: &AccountInfo<'a>,
    vault_b_acc: &AccountInfo<'a>,
    vault_tick0_acc: &AccountInfo<'a>,
    vault_tick1_acc: &AccountInfo<'a>,
    vault_tick2_acc: &AccountInfo<'a>,
    oracle: &AccountInfo<'a>,
    spl_token_program: &AccountInfo<'a>,
    amount_in: u64,
    aToB: u8,
  ) -> ProgramResult {

    let whirl_swap_accounts = vec![
      spl_token_program.clone(),
      user_owner_acc.clone(),
      market_acc.clone(),
      user_a_acc.clone(),
      vault_a_acc.clone(),
      user_b_acc.clone(),
      vault_b_acc.clone(),
      vault_tick0_acc.clone(),
      vault_tick1_acc.clone(),
      vault_tick2_acc.clone(),
      oracle.clone(),
    ];

    let accounts = vec![
      AccountMeta::new(*spl_token_program.key, false),
      AccountMeta::new(*user_owner_acc.key, true),
      AccountMeta::new(*market_acc.key, false),
      AccountMeta::new(*user_a_acc.key, false),
      AccountMeta::new(*vault_a_acc.key, false),
      AccountMeta::new(*user_b_acc.key, false),
      AccountMeta::new(*vault_b_acc.key, false),
      AccountMeta::new(*vault_tick0_acc.key, false),
      AccountMeta::new(*vault_tick1_acc.key, false),
      AccountMeta::new(*vault_tick2_acc.key, false),
      AccountMeta::new(*oracle.key, false),
    ];

    let mut data: [u8;35] = [0;35];
    let zero:u64 = 0;
    let max:u64 = u64::MAX;
    let (
      instruction_dst,
      amount_dst,
      otherAmountThreshold_dst,
      sqrtPriceLimit1_dst,
      sqrtPriceLimit2_dst,
      exactInput_dst,
      aToB_dst,
    ) = mut_array_refs![&mut data, 1, 8, 8, 8, 8, 1, 1];
    instruction_dst[0] = 248;
    *amount_dst = amount_in.to_le_bytes();
    *otherAmountThreshold_dst = zero.to_le_bytes();
    *sqrtPriceLimit1_dst = max.to_le_bytes();
    *sqrtPriceLimit2_dst = max.to_le_bytes();
    exactInput_dst[0] = 0;
    aToB_dst[0] = aToB;

    let whirl_swap_instruction = Instruction {
      program_id: *program_id,
      accounts: accounts,
      data: data.to_vec(),
    };

    msg!("invoke whirl swap");
    invoke(&whirl_swap_instruction, &whirl_swap_accounts[..])?;

    Ok(())
  }

  /*
  fn marinade_deposit<'a> (
    program_id: &Pubkey,
    reserve_acc: &AccountInfo<'a>,
    referral_acc: &AccountInfo<'a>,
    state_acc: &AccountInfo<'a>,
    user_acc: &AccountInfo<'a>,
    recipient_acc: &AccountInfo<'a>,
    m_sol_mint: &AccountInfo<'a>,
    mint_auth: &AccountInfo<'a>,
    liq_pool_msol_auth: &AccountInfo<'a>,
    liq_pool_msol: &AccountInfo<'a>,
    liq_pool_msol_pda: &AccountInfo<'a>,
    spl_token_program: &AccountInfo<'a>,
    system_program: &AccountInfo<'a>,
    amount_in: u64,

  ) -> ProgramResult {

    let lido_deposit_accounts = vec![
      lido_acc.clone(),
      user_acc.clone(),
      recipient_acc.clone(),
      st_sol_mint.clone(),
      reserve_acc.clone(),
      mint_auth.clone(),
      spl_token_program.clone(),
      system_program.clone(),
    ];

    let accounts = vec![
      AccountMeta::new(*lido_acc.key, false),
      AccountMeta::new(*user_acc.key, true),
      AccountMeta::new(*recipient_acc.key, false),
      AccountMeta::new(*st_sol_mint.key, false),
      AccountMeta::new(*reserve_acc.key, false),
      AccountMeta::new(*mint_auth.key, false),
      AccountMeta::new(*spl_token_program.key, false),
      AccountMeta::new(*system_program.key, false),
    ];

    let mut data: [u8;9] = [0;9];
    let (
      instruction,
      amount,
    ) = mut_array_refs![&mut data, 1, 8];
    instruction[0] = 1;
    *amount = amount_in.to_le_bytes();

    let lido_deposit_instruction = Instruction {
      program_id: *program_id,
      accounts: accounts,
      data: data.to_vec(),
    };

    msg!("invoke lido deposit");
    invoke(&lido_deposit_instruction, &lido_deposit_accounts[..])?;

    Ok(())
  }
  */
}
