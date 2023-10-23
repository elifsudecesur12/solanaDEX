use solana_program::{
    account_info::next_account_info,
    account_info::AccountInfo,
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    pubkey::Pubkey,
};
use spl_token::state::Account;
use spl_token::instruction::transfer;

entrypoint!(_entry);

fn _entry(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    input: &[u8],
) -> ProgramResult {
    msg!("Rust liquidity pool program entrypoint");

    let operation: PoolOperation = PoolOperation::unpack(input)?;
    
    match operation {
        PoolOperation::AddLiquidity { amount0, amount1 } => {
            add_liquidity(accounts, amount0, amount1)?;
        }
        PoolOperation::RemoveLiquidity { liquidity_amount } => {
            remove_liquidity(accounts, liquidity_amount)?;
        }
    }

    Ok(())
}

enum PoolOperation {
    AddLiquidity {
        amount0: u64,
        amount1: u64,
    },
    RemoveLiquidity {
        liquidity_amount: u64,
    },
}

impl PoolOperation {
    pub fn unpack(input: &[u8]) -> Option<Self> {
        if input.len() < 1 {
            return None;
        }
        let mut data = input;
        let operation_type = data.get(0)?;
        data = &data[1..];
        match operation_type {
            0 => {
                if data.len() != 16 {
                    return None;
                }
                let amount0 = u64::from_le_bytes(data[..8].try_into().unwrap());
                let amount1 = u64::from_le_bytes(data[8..].try_into().unwrap());
                Some(PoolOperation::AddLiquidity { amount0, amount1 })
            }
            1 => {
                if data.len() != 8 {
                    return None;
                }
                let liquidity_amount = u64::from_le_bytes(data.try_into().unwrap());
                Some(PoolOperation::RemoveLiquidity { liquidity_amount })
            }
            _ => None,
        }
    }
}

fn add_liquidity(accounts: &[AccountInfo], amount0: u64, amount1: u64) {
    let owner_account = next_account_info(accounts[0]).unwrap();
    let pool_token_account = next_account_info(accounts[1]).unwrap();
    let token0_account = next_account_info(accounts[2]).unwrap();
    let token1_account = next_account_info(accounts[3]).unwrap();

    let transfer_authority = accounts[4]; 
    let transfer_instruction = transfer(
        transfer_authority.key,
        token0_account.key,
        token1_account.key,
        amount0,
    );
    invoke(&transfer_instruction, &[token0_account.clone(), token1_account.clone()])?;

   
}

fn remove_liquidity(accounts: &[AccountInfo], liquidity_amount: u64) {
    let owner_account = next_account_info(accounts[0]).unwrap();
    let pool_token_account = next_account_info(accounts[1]).unwrap();
    let token0_account = next_account_info(accounts[2]).unwrap();
    let token1_account = next_account_info(accounts[3]).unwrap();

   
}


}
