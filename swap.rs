use solana_program::{
    account_info::next_account_info,
    account_info::AccountInfo,
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    pubkey::Pubkey,
    sysvar::Sysvar,
};
use spl_token::state::Account;
use spl_token::instruction::transfer;

entrypoint!(_entry);

fn _entry(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    input: &[u8],
) -> ProgramResult {
    msg!("Rust swap program entrypoint");

    let swap_instruction: SwapInstruction = SwapInstruction::unpack(input)?;
    
    let num_required_accounts = match swap_instruction.swap_type {
        SwapType::SOLToToken | SwapType::TokenToSOL => 4,
        SwapType::TokenToToken => 5,
    };
    if accounts.len() != num_required_accounts {
        return Err(ProgramError::InvalidArgument);
    }

    match swap_instruction.swap_type {
        SwapType::SOLToToken => {
            sol_to_token_swap(accounts, swap_instruction.amount)?;
        }
        SwapType::TokenToSOL => {
            token_to_sol_swap(accounts, swap_instruction.amount)?;
        }
        SwapType::TokenToToken => {
            token_to_token_swap(accounts, swap_instruction.amount)?;
        }
    }

    Ok(())
}

enum SwapType {
    SOLToToken,
    TokenToSOL,
    TokenToToken,
}

struct SwapInstruction {
    swap_type: SwapType,
    amount: u64,
}

impl SwapInstruction {
    pub fn unpack(input: &[u8]) -> Option<Self> {
        let mut data = input;
        let swap_type = match data.get(0) {
            Some(0) => SwapType::SOLToToken,
            Some(1) => SwapType::TokenToSOL,
            Some(2) => SwapType::TokenToToken,
            _ => return None,
        };
        data = &data[1..];
        let amount = u64::from_le_bytes(data.try_into().ok()?);
        Some(SwapInstruction { swap_type, amount })
    }
}

fn sol_to_token_swap(accounts: &[AccountInfo], amount: u64) {
  
    let owner_account = next_account_info(accounts[0]).unwrap();
    let source_sol_account = next_account_info(accounts[1]).unwrap();
    let target_token_account = next_account_info(accounts[2]).unwrap();

  .
}


fn token_to_sol_swap(accounts: &[AccountInfo], amount: u64) {
    let owner_account = next_account_info(accounts[0]).unwrap();
    let source_token_account = next_account_info(accounts[1]).unwrap();
    let target_sol_account = next_account_info(accounts[2]).unwrap();

    
}


fn token_to_token_swap(accounts: &[AccountInfo], amount: u64) {
    let owner_account = next_account_info(accounts[0]).unwrap();
    let source_token_account = next_account_info(accounts[1]).unwrap();
    let target_token_account = next_account_info(accounts[2]).unwrap();

  
}
