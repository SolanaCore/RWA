use {
    pinocchio::{
        account_info::AccountInfo,
        instruction::Signer,
        program_error::ProgramError,
        pubkey::{
            Pubkey,
            find_program_address},
        rent::Rent,
        seeds,
        ProgramResult
    },
    pinocchio_token_2022::{
        instructions::{CreateAccount, InitializeMint2},
        states::Mint,
    },
    crate::{
        errors::RWAError,
        utils::{AccountCheck, load_acc_mut_unchecked},
        states::TokenMetadata
    },
};

/// Trait for initializing mints and metadata
pub trait MintInit {
    fn init(
        account: &AccountInfo,
        payer: &AccountInfo,
        decimals: u8,
        mint_authority: &[u8; 32],
        freeze_authority: Option<&[u8; 32]>,
    ) -> ProgramResult;

    fn init_if_needed(
        account: &AccountInfo,
        payer: &AccountInfo,
        decimals: u8,
        mint_authority: &[u8; 32],
        freeze_authority: Option<&[u8; 32]>,
    ) -> ProgramResult;

    fn init_metadata_account(
        metadata: &AccountInfo,
        mint_ai: &AccountInfo,
        program_id: &Pubkey,
        name: &str,
        symbol: &str,
        uri: &str,
    ) -> ProgramResult;

    fn init_if_needed_and_init_metadata_account(
        account: &AccountInfo,
        payer: &AccountInfo,
        decimals: u8,
        mint_authority: &[u8; 32],
        freeze_authority: Option<&[u8; 32]>,
        metadata_account: &AccountInfo,
        program_id: &Pubkey,
        name: &str,
        symbol: &str,
        uri: &str,
    ) -> ProgramResult;
}

/// Represents a Mint (Token-2022)
pub struct Mint2022Account;

impl<TokenMetadata> AccountCheck for Mint2022Account {
    fn check(account: &AccountInfo) -> ProgramResult {
        if !account.is_owned_by(&pinocchio_token_2022::ID) {
            return Err(RWAError::InvalidOwner.into());
        }

        let data = account.try_borrow_data()?;
        if data.len() != Mint::LEN {
            return Err(RWAError::InvalidAccountData.into());
        }

        Ok(())
    }
}

impl MintInit for Mint2022Account {
    fn init(
        account: &AccountInfo,
        payer: &AccountInfo,
        decimals: u8,
        mint_authority: &[u8; 32],
        freeze_authority: Option<&[u8; 32]>,
        token_program: &[u8; 32],
    ) -> ProgramResult {
        // Get required lamports for rent
        let lamports = Rent::get()?.minimum_balance(Mint::LEN);

        // Create account
        CreateAccount {
            from: payer,
            to: account,
            lamports,
            space: Mint::LEN as u64,
            owner: &crate::ID,
        }
        .invoke()?;

        // Initialize mint
        InitializeMint2 {
            mint: account,
            decimals,
            mint_authority,
            freeze_authority,
            token_program,
        }
        .invoke()?;

        Ok(())
    }

    fn init_if_needed(
        account: &AccountInfo,
        payer: &AccountInfo,
        decimals: u8,
        mint_authority: &[u8; 32],
        freeze_authority: Option<&[u8; 32]>,
    ) -> ProgramResult {
        match Self::check(account) {
            Ok(_) => Ok(()),
            Err(_) => Self::init(account, payer, decimals, mint_authority, freeze_authority),
        }
    }

    fn init_metadata_account(
        metadata: &AccountInfo,
        mint: &AccountInfo,
        authority: &AccountInfo,
        program_id: &Pubkey,
        name: &str,
        symbol: &str,
        uri: &str,
    ) -> ProgramResult {
        // Derive PDA
        let (expected_pda, bump) =
            find_program_address(&[b"metadata", mint.key().as_ref()], program_id);

        if metadata.key() != &expected_pda {
            return Err(ProgramError::InvalidSeeds);
        }
        let seeds = seeds!(b"metadata", mint.key().as_ref(), program_id);
        let signer_seeds = Signer::from(seeds);

        let _ = ProgramAccount::init_if_needed(&signer, &metadata, signer_seeds, core::mem::size_of::<TokenMetadata>());
        // Borrow PDA buffer
        let mut data = metadata.try_borrow_mut_data()?;
        let metadata: &mut crate::states::TokenMetadata =
            unsafe { load_acc_mut_unchecked::<TokenMetadata>(&mut data)? };

            if name.len() > 32 {
                return Err(RWAError::InvalidInstructionData.into())?;
            }
        // Fill metadata
        metadata.mint = *mint.key().as_ref();
        metadata.authority = *authority.key().as_ref();
        metadata.name = name.bytes();
        metadata.symbol = symbol.bytes();
        metadata.uri = uri.bytes();
        metadata.bump = bump;


        Ok(())
    }

    fn init_if_needed_and_init_metadata_account(
        account: &AccountInfo,
        payer: &AccountInfo,
        decimals: u8,
        mint_authority: &[u8; 32],
        freeze_authority: Option<&[u8; 32]>,
        metadata_account: &AccountInfo,
        program_id: &Pubkey,
        name: &str,
        symbol: &str,
        uri: &str,
    ) -> ProgramResult {
        match Self::check(account) {
            Ok(_) => Ok(()),
            Err(_) => {
                Self::init(account, payer, decimals, mint_authority, freeze_authority)?;
                Self::init_metadata_account(metadata_account, account, program_id, name, symbol, uri)
            }
        }
    }
}
