use {
    pinocchio::{
        account_info::AccountInfo,
        program_error::ProgramError,
        pubkey::Pubkey,
        rent::Rent,
        sysvar::Sysvar,
        program::invoke_signed,
        system_instruction::create_account,
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
    pub account: &'a AccountInfo<'a>,
}

impl<'a> TryFrom<&'a AccountInfo<'a>> for SignerAccount<'a> {
    type Error = ProgramError;

    fn try_from(account: &'a AccountInfo<'a>) -> Result<Self, Self::Error> {
        SignerAccount::check(account)?;
        Ok(Self { account })
    }
}

impl<'a> AccountCheck for SignerAccount<'a> {
    fn check(account: &AccountInfo) -> Result<(), ProgramError> {
        if !account.is_signer {
            return Err(RWAError::NotSigner.into());
        }
        Ok(())
    }
}

/// System account
pub struct SystemAccount<'a> {
    pub account: &'a AccountInfo<'a>,
}

impl<'a> TryFrom<&'a AccountInfo<'a>> for SystemAccount<'a> {
    type Error = ProgramError;

    fn try_from(account: &'a AccountInfo<'a>) -> Result<Self, Self::Error> {
        SystemAccount::check(account)?;
        Ok(Self { account })
    }
}

impl<'a> AccountCheck for SystemAccount<'a> {
    fn check(account: &AccountInfo) -> Result<(), ProgramError> {
        if !account.is_owned_by(&pinocchio::system_program::ID) {
            return Err(RWAError::InvalidOwner.into());
        }
        Ok(())
    }
}

/// Program account (PDA)
pub struct ProgramAccount<'a> {
    pub account: &'a AccountInfo<'a>,
}

impl<'a> TryFrom<&'a AccountInfo<'a>> for ProgramAccount<'a> {
    type Error = ProgramError;

    fn try_from(account: &'a AccountInfo<'a>) -> Result<Self, Self::Error> {
        ProgramAccount::check(account)?;
        Ok(Self { account })
    }
}

impl<'a> AccountCheck for ProgramAccount<'a> {
    fn check(account: &AccountInfo) -> Result<(), ProgramError> {
         

        if !account.is_owned_by(&ID) {
            return Err(RWAError::InvalidOwner.into());
        }

        Ok(())
    }
}

/// Trait for initializing a program account (PDA)
pub trait ProgramAccountInit {
    fn init<'a>(
        payer: &AccountInfo<'a>,
        account: &AccountInfo<'a>,
        seeds: &[&[u8]],
        space: usize,
    ) -> Result<(), ProgramError>;

    fn init_if_needed<'a>(
        payer: &AccountInfo<'a>,
        account: &AccountInfo<'a>,
        seeds: &[&[u8]],
        space: usize,
    ) -> Result<(), ProgramError>;
}

impl ProgramAccountInit for ProgramAccount<'_> {
    fn init<'a>(
        payer: &AccountInfo<'a>,
        account: &AccountInfo<'a>,
        seeds: &[&[u8]],
        space: usize,
    ) -> Result<(), ProgramError> {
        let lamports = Rent::get()?.minimum_balance(space);
        let signer_seeds: &[&[u8]] = seeds;

        let ix = create_account(
            payer.key,
            account.key,
            lamports,
            space as u64,
            &ID,
        );

        invoke_signed(&ix, &[payer.clone(), account.clone()], &[signer_seeds])?;
        Ok(())
    }

    fn init_if_needed<'a>(
        payer: &AccountInfo<'a>,
        account: &AccountInfo<'a>,
        seeds: &[&[u8]],
        space: usize,
    ) -> Result<(), ProgramError> {
        match ProgramAccount::check(account) {
            Ok(_) => Ok(()),
            Err(_) => Self::init(payer, account, seeds, space),
        }
    }
}

/// Trait for closing a program account safely
pub trait AccountClose {
    fn close(account: &AccountInfo, destination: &AccountInfo) -> Result<(), ProgramError>;
}

impl AccountClose for ProgramAccount<'_> {
    fn close(account: &AccountInfo, destination: &AccountInfo) -> Result<(), ProgramError> {
        let lamports = **account.lamports.borrow();
        **destination.lamports.borrow_mut() += lamports;
        **account.lamports.borrow_mut() = 0;

        account.try_borrow_mut_data()?.fill(0);
        Ok(())
    }
}
