use pinocchio_associated_token_account::{
    instruction::Create
};
use pinocchio::program_error::ProgramError;
use crate::errors::MyProgramError;
use pinocchio::ProgramResult;
use pinocchio::account_info::AccountInfo;
use pinocchio::pubkey::find_program_address;

pub trait AssociatedTokenAccountCheck {
    fn check(account: &AccountInfo, authority: &AccountInfo, mint: &AccountInfo, token_program: &AccountInfo) -> ProgramResult
}

pub struct AssociatedTokenAccount;
 
impl AssociatedTokenAccountCheck for AssociatedTokenAccount {
    fn check(
        account: &AccountInfo,
        authority: &AccountInfo,
        mint: &AccountInfo,
        token_program: &AccountInfo,
    ) -> ProgramResult {
        TokenAccount::check(account)?;
 
        if find_program_address(
            &[authority.key(), token_program.key(), mint.key()],
            &pinocchio_associated_token_account::ID,
        )
        .0
        .ne(account.key())
        {
            return Err(MyProgramError::InvalidAddress.into());
        }
 
        Ok(())
    }
}
 
impl AssociatedTokenAccountInit for AssociatedTokenAccount {
    fn init(account: &AccountInfo, mint: &AccountInfo, payer: &AccountInfo, owner: &AccountInfo, system_program: &AccountInfo, token_program: &AccountInfo) -> ProgramResult {
        Create {
            funding_account: payer,
            account,
            wallet: owner,
            mint,
            system_program,
            token_program,
        }.invoke()
    }
 
    fn init_if_needed(account: &AccountInfo, mint: &AccountInfo, payer: &AccountInfo, owner: &AccountInfo, system_program: &AccountInfo, token_program: &AccountInfo) -> ProgramResult {
        match Self::check(account, payer, mint) {
            Ok(_) => Ok(()),
            Err(_) => Self::init(account, mint, payer, owner, system_program, token_program),
        }
    }
}