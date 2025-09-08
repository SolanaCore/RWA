use {
    pinocchio::{
        account_info::AccountInfo,
        program_error::ProgramError,
        pubkey::Pubkey,
        rent::Rent,
        sysvar::Sysvar,
        program::invoke_signed,
        system_instruction::create_account,
        Signer,
        ProgramResult
    },
    core::convert::TryFrom,
    core::mem::size_of,
    crate::errors::RWAError,
    crate::states::GlobalConfig,
    crate::ID,
    crate::utils::{
        AccountCheck, load_acc_mut_unchecked
    },
};

/// Signer account
pub struct SignerAccount<'a> {
    pub account: &'a AccountInfo,
}

impl<'a> TryFrom<&'a AccountInfo> for SignerAccount<'a> {
    type Error = ProgramError;

    fn try_from(account: &'a AccountInfo) -> ProgramResult {
        SignerAccount::check(account)?;
        Ok(account)
    }
}

impl<'a> AccountCheck for SignerAccount<'a> {
    fn check(account: &AccountInfo) -> ProgramResult {
        if !account.is_signer() {
            return Err(RWAError::NotSigner.into());
        }
        Ok(())
    }
}

/// System account
pub struct SystemAccount<'a> {
    pub account: &'a AccountInfo,
}

impl<'a> TryFrom<&'a AccountInfo> for SystemAccount<'a> {
    type Error = ProgramError;

    fn try_from(account: &'a AccountInfo) -> ProgramResult {
        SystemAccount::check(account)?;
        Ok(account)
    }
}

impl<'a> AccountCheck for SystemAccount<'a> {
    fn check(account: &AccountInfo) -> ProgramResult {
        if !account.is_owned_by(&pinocchio::system_program::ID) {
            return Err(RWAError::InvalidOwner.into());
        }
        Ok(())
    }
}

/// Program account (PDA)
pub struct ProgramAccount<'a> {
    pub account: &'a AccountInfo,
}

impl<'a> TryFrom<&'a AccountInfo> for ProgramAccount<'a> {
    type Error = ProgramError;

    fn try_from(account: &'a AccountInfo) -> ProgramResult {
        ProgramAccount::check(account)?;
        Ok(Self { account })
    }
}

impl<'a> AccountCheck for ProgramAccount<'a> {
    fn check(account: &'a AccountInfo) -> ProgramResult {
         

        if !account.is_owned_by(&ID) {
            return Err(RWAError::InvalidOwner.into());
        }

        Ok(())
    }
}

/// Trait for initializing a program account (PDA)
pub trait ProgramAccountInit {
    fn init<'a>(
        payer: &'a AccountInfo,
        account: &'a AccountInfo,
        seeds: &[&[u8]],
        space: usize,
    ) -> Result<(), ProgramError>;

    fn init_if_needed<'a>(
        payer: &'a AccountInfo,
        account: &'a AccountInfo,
        seeds: &[&[u8]],
        space: usize,
    ) -> Result<(), ProgramError>;
}

impl ProgramAccountInit for ProgramAccount<'_> {
    fn init<'a>(
        payer: &'a AccountInfo,
        account: &'a AccountInfo,
        seeds: &[&[u8]],
        space: usize,
    ) -> ProgramResult {
        let lamports = Rent::get()?.minimum_balance(space);

        let ix = create_account(
            payer.key(),
            account.key(),
            lamports,
            space as u64,
            &ID,
        );
        let signer_seeds = Signer::from(signer_seeds);

        invoke_signed(&ix, &[&payer, &account], &[signer_seeds])?;
        Ok(())
    }

    fn init_if_needed<'a>(
        payer: &'a AccountInfo,
        account: &'a AccountInfo,
        seeds: &[&[u8]],
        space: usize,
    ) -> ProgramResult {
        match ProgramAccount::check(account) {
            Ok(_) => Ok(()),
            Err(_) => Self::init(payer, account, seeds, space),
        }
    }
}

/// Trait for closing a program account safely
pub trait AccountClose {
    fn close(account: &'a AccountInfo, destination: &'a AccountInfo) -> ProgramResult;
}

impl<'a> AccountClose<'a> for ProgramAccount<'_> {
    fn close<'a>(account: &'a AccountInfo, destination: &'a AccountInfo) -> ProgramResult {
        let lamports = **account.lamports.borrow();
        **destination.lamports.borrow_mut() += lamports;
        **account.lamports.borrow_mut() = 0;

        account.try_borrow_mut_data()?.fill(0);
        Ok(())
    }
}
