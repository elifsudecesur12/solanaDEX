use solana_program::account_info::next_account_info;
use solana_program::account_info::AccountInfo;
use solana_program::entrypoint;
use solana_program::entrypoint::ProgramResult;
use solana_program::msg;
use solana_program::program_error::ProgramError;
use solana_program::program_pack::Pack;
use solana_program::pubkey::Pubkey;
use solana_program::solana_entrypoint;
use solana_program::sysvar::rent::Rent;
use solana_program::sysvar::Sysvar;
use solana_sdk::program_pack::IsInitialized;
use solana_sdk::program_utils::limited_deserialize;
use solana_sdk::pubkey::Pubkey as SolanaPubkey;
use solana_sdk::spl_token;
use solana_sdk::spl_token::state::Account;
use spl_token::error::TokenError;
use spl_token::instruction;
use spl_token::state::Mint;

entrypoint!(_entry);

fn _entry(program_id: &SolanaPubkey, accounts: &[AccountInfo], input: &[u8]) -> ProgramResult {
    msg!("Rust program entrypoint");

    let instruction = limited_deserialize(input)?;
    match instruction {
        Instruction::InitializeMint { decimals, mint_authority } => {
            msg!("Initialize Mint");
            initialize_mint(accounts, decimals, mint_authority)?;
        }
        Instruction::InitializeAccount { mint, owner } => {
            msg!("Initialize Account");
            initialize_account(accounts, mint, owner)?;
        }
        Instruction::Transfer { amount } => {
            msg!("Transfer");
            transfer(accounts, amount)?;
        }
    }

    Ok(())
}

enum Instruction {
    InitializeMint {
        decimals: u8,
        mint_authority: Pubkey,
    },
    InitializeAccount {
        mint: Pubkey,
        owner: Pubkey,
    },
    Transfer {
        amount: u64,
    },
}

fn initialize_mint(accounts: &[AccountInfo], decimals: u8, mint_authority: Pubkey) -> ProgramResult {
    let mint_info = next_account_info(accounts)?;
    let rent_info = next_account_info(accounts)?;

    if mint_info.data.borrow().len() != Mint::LEN {
        return Err(ProgramError::InvalidAccountData);
    }

    let rent = &Rent::from_account_info(rent_info)?;

    let mut mint = Mint::unpack_unchecked(&mint_info.data.borrow())?;
    mint.is_initialized = true;
    mint.decimals = decimals;
    mint.mint_authority = COption::Some(mint_authority.into());

    let rent_exempt_reserve = rent.minimum_balance(mint_data_len)?;
    mint_info.checked_balance()?;
    if mint_info.lamports() < rent_exempt_reserve {
        return Err(TokenError::NotRentExempt.into());
    }

    Mint::pack(mint, &mut mint_info.data.borrow_mut())?;

    Ok(())
}

fn initialize_account(accounts: &[AccountInfo], mint: Pubkey, owner: Pubkey) -> ProgramResult {
    let account_info = next_account_info(accounts)?;

    if account_info.data.borrow().len() != Account::LEN {
        return Err(ProgramError::InvalidAccountData);
    }

    let mut account = Account::unpack_unchecked(&account_info.data.borrow())?;
    account.mint = mint;
    account.owner = owner;

    Account::pack(account, &mut account_info.data.borrow_mut())?;

    Ok(())
}

fn transfer(accounts: &[AccountInfo], amount: u64) -> ProgramResult {
    let source_info = next_account_info(accounts)?;
    let destination_info = next_account_info(accounts)?;

    let source = Account::unpack(&source_info.data.borrow())?;
    if !source.is_initialized {
        return Err(ProgramError::UninitializedAccount);
    }

    let destination = Account::unpack(&destination_info.data.borrow())?;
    if !destination.is_initialized {
        return Err(ProgramError::UninitializedAccount);
    }

    if source.mint != destination.mint {
        return Err(TokenError::MintMismatch.into());
    }

    if source.owner != *source_info.owner {
        return Err(ProgramError::Custom(1)); 
    }

    if source.amount < amount {
        return Err(TokenError::InsufficientFunds.into());
    }

    source.amount -= amount;
    destination.amount += amount;

    Account::pack(source, &mut source_info.data.borrow_mut())?;
    Account::pack(destination, &mut destination_info.data.borrow_mut())?;

    Ok(())
}
