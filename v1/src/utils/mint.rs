use {
    pinocchio::{
        account_info::AccountInfo,
        instruction::Signer,
        program_error::ProgramError,
        pubkey::Pubkey,
        rent::Rent,
    },
    pinocchio_token_2022::{
        instruction::{CreateAccount, InitializeMint2},
        states::Mint,
    },
    crate::{
        errors::RWAError,
        utils::{AccountCheck, load_acc_mut_unchecked},
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
    ) -> Result<(), ProgramError>;

    fn init_if_needed(
        account: &AccountInfo,
        payer: &AccountInfo,
        decimals: u8,
        mint_authority: &[u8; 32],
        freeze_authority: Option<&[u8; 32]>,
    ) -> Result<(), ProgramError>;

    fn init_metadata_account(
        metadata: &AccountInfo,
        mint_ai: &AccountInfo,
        program_id: &Pubkey,
        name: &str,
        symbol: &str,
        uri: &str,
    ) -> Result<(), ProgramError>;

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
    ) -> Result<(), ProgramError>;
}

/// Represents a Mint (Token-2022)
pub struct Mint2022Account;

impl AccountCheck for Mint2022Account {
    fn check(account: &AccountInfo) -> Result<(), ProgramError> {
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
    ) -> Result<(), ProgramError> {
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
    ) -> Result<(), ProgramError> {
        match Self::check(account) {
            Ok(_) => Ok(()),
            Err(_) => Self::init(account, payer, decimals, mint_authority, freeze_authority),
        }
    }

    fn init_metadata_account(
        metadata: &AccountInfo,
        mint: &AccountInfo,
        program_id: &Pubkey,
        name: &str,
        symbol: &str,
        uri: &str,
    ) -> Result<(), ProgramError> {
        // Derive PDA
        let (expected_pda, bump) =
            Pubkey::find_program_address(&[b"metadata", mint.key.as_ref()], program_id);

        if metadata.key != &expected_pda {
            return Err(ProgramError::InvalidSeeds);
        }
        let seeds = seeds!(b"metadata", mint.key.as_ref, program_id);
        let signer_seeds = Signer::from(seeds);

        ProgramAccount::init_if_needed(&signer, &metadata, signer_seeds, )
        // Borrow PDA buffer
        let mut data = metadata.try_borrow_mut_data()?;
        let metadata: &mut crate::state::TokenMetadata =
            unsafe { load_acc_mut_unchecked::<TokenMetadata>(&mut data)? };

            if name.len() > 32 {
                return Err(RWAError::InvalidInstructionData.into())?;
            }
        // Fill metadata
        metadata.mint = mint.as_bytes();
        metadata.authority = authority.as_bytes();
        metadata.name = name.as_bytes();
        metadata.symbol = symbol.as_bytes();
        metadata.uri = uri.as_bytes();
        metadata.bump = bump


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
    ) -> Result<(), ProgramError> {
        match Self::check(account) {
            Ok(_) => Ok(()),
            Err(_) => {
                Self::init(account, payer, decimals, mint_authority, freeze_authority)?;
                Self::init_metadata_account(metadata_account, account, program_id, name, symbol, uri)
            }
        }
    }
}
