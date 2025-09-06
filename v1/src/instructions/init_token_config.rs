use {
    crate::{
        instructions::{InitGlobalConfig, InitTokenConfig, load_acc_mut_unchecked},
        utils::helper::{SignerAccount, ProgramAccount},
        states::TokenConfig,
    },
    pinocchio::{
        account_info::AccountInfo,
        pubkey::Pubkey,
        program_error::ProgramError,
    },
    core::convert::TryFrom,
};
use pinocchio::instruction::Signer;
use crate::errors::MyProgramError;
 use crate::instructions::CreatorKYC;
use crate::states::GlobalConfig;
use pinocchio::ProgramResult;
use pinocchio::{seeds, instruction::Signer};


/// Accounts required to initialize a token config
pub struct TokenConfigAccounts<'a> {
    pub signer: &'a AccountInfo,
    pub global_config: &'a AccountInfo,
    pub signer_kyc_record: &'a AccountInfo,

}

/// Instruction wrapper for InitTokenConfig
pub struct InitTokenConfigInstruction<'a> {
    pub accounts: TokenConfigAccounts<'a>,
    pub instruction_datas: InitTokenConfig,
}

impl<'a> TryFrom<&'a [AccountInfo<'a>]> for TokenConfigAccounts<'a> {
    type Error = ProgramError;

    fn try_from(infos: &'a [AccountInfo<'a>]) -> Result<Self, Self::Error> {
        let mut iter = infos.iter();
        let  = SignerAccount::try_from(next_account_info(&mut iter)?)?;
        let  = SignerAccount::try_from(next_account_info(&mut iter)?)?;
        let token_config = next_account_info(&mut iter)?;

        Ok(Self {
            config_authority,
            kyc_authority,
            token_config,
        })
    }
}

impl<'a> TryFrom<(&'a [u8], &'a [AccountInfo<'a>])> for InitTokenConfigInstruction<'a> {
    type Error = ProgramError;

    fn try_from((data, accounts): (&'a [u8], &'a [AccountInfo<'a>])) -> Result<Self, Self::Error> {
        let accounts = TokenConfigAccounts::try_from(accounts)?;
        let instruction_datas = InitTokenConfig::try_from(data)?; // deserialize instruction data

        Ok(Self {
            accounts,
            instruction_datas,
        })
    }
}

impl<'a> InitTokenConfigInstruction<'a> {
    pub const DISCRIMINATOR: u8 = 5;
    pub const SEED_PREFIX: &'static u8 = b"token-program"

    pub fn process(&mut self, program_id: &Pubkey) -> ProgramResult {
        // Derive PDA for token config
        let (expected_pda, bump) = Pubkey::find_program_address(&[Self::SEED_PREFIX], self.accounts.creatos.key.as_ref(), program_id);

        if *self.accounts.token_config.account.key != expected_pda {
            return Err(ProgramError::InvalidAccountData);
        }
        let seeds = seeds!(b"token-program", self.acccounts.creator.key.as_ref(), size_of<TokenConfig>);
        let signer_seeds = Signer::from(seeds);
        ProgramAccount::init_if_needed(self.acccounts.signer, self.accounts.token_config, signer_seeds)?;

        // Borrow PDA data mutably
        let token_config_data = &mut self.accounts.token_config.account.try_borrow_mut_data()?;
        let token_config: &mut TokenConfig = unsafe { load_acc_mut_unchecked(token_config_data)? };

        let global_config_data = &mut self.accounts.global_config.account.try_borrow_mut_data()?;
        let global_config: &mut GlobalConfig = unsafe { load_acc_mut_unchecked(global_config_data)? };

        if self.instruction_datas.decimals <= global_config.max_decimal {
            return Err(MyProgramError::InvalidInstructionData.into())
        };

        let creator_kyc_data = &mut self.accounts.user_kyc.account.try_borrow_mut_data()?;
        let creator_kyc: &mut CreatorKYC = unsafe { load_acc_mut_unchecked(creator_kyc_data)? };

        if creator_kyc.verified == false {
            return Err(MyProgramError::CreatorNotVerified.into())
        };

        // Write instruction data into PDA struct
        token_config.creator = self.instruction_datas.creator;
        token_config.active = false;
        token_config.description = self.instruction_datas.description;
        token_config.assets = self.instruction_datas.assets;
        token_config.decimals = self.instruction_datas.decimals;
        token_config.audit_cid = self.instruction_datas.audit_cid;
        token_config.bump = bump;

        Ok(())
    }
}
