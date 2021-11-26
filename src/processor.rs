use solana_program::{
  account_info::{next_account_info, AccountInfo},
  entrypoint::ProgramResult,
  msg,
  pubkey::Pubkey,
  program_error::ProgramError,
};

use crate::instruction::EscrowInstruction;
use crate::error::EscrowError::InvalidInstruction;

pub struct Processor;

impl Processor {

  // Processor::process processes the transaction
  pub fn process(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
  ) -> ProgramResult {

    // call the main function from the EscrowInstruction implementation
    // this is the first step
    let instruction = EscrowInstruction::unpack(instruction_data)?;

    match instruction {

      // case is an instance of EscrowInstruction::InitEscrow{ x }
      // we take this (what is it???) and pass the details to a new private function
      // defined here
      EscrowInstruction::InitEscrow{ amount } => {
        msg!("Instruction: InitEscrow");
        Self::process_init_escrow(accounts, amount, program_id)
      }
    }
  }

  fn process_init_escrow(
    accounts: &[AccountInfo],
    amount: u64,
    program_id: &Pubkey,
  ) -> ProgramResult {

    // mut is a "mutable raw variable" it allows us to iterate through the
    // accounts
    let account_info_iter = &mut accounts.iter();

    // next_account_info is contained in account_info::AccountInfo
    // and allows us to iterate easily through the accounts
    let initializer = next_account_info(account_info_iter)?;

    if !initializer.is_signer {
      return Err(ProgramError::MissingRequiredSignature);
    }

    let temp_token_account = next_account_info(account_info_iter)?;

    let token_to_receive_account = next_account_info(account_info_iter)?;

    if *token_to_receive_account.owner != spl_token::id() {
      return Err(ProgramError::IncorrectProgramId);
    }

    Ok(())
  }

}