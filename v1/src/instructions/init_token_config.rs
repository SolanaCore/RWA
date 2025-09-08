use {
    core::convert::TryFrom,
    pinocchio::{
        account_info::AccountInfo,
        instruction::Signer,
        program_error::ProgramError,
        pubkey::Pubkey,
        ProgramResult,
        seeds,
    },
    crate::{
        errors::RWAError,
        instructions::{
            CreatorKYC,
            InitGlobalConfig,
            InitTokenConfig,
        },
        states::{GlobalConfig, TokenConfig},
        utils::{ProgramAccount, SignerAccount, load_acc_mut_unchecked, ProgramAccountInit},
    },
};

/// Accounts required to initialize a token config
pub struct TokenConfigAccounts<'a> {
    pub signer: &'a AccountInfo,
    pub global_config: &'a AccountInfo,
    pub creator_kyc: &'a AccountInfo,
    pub token_config: &'a AccountInfo,

}

/// Instruction wrapper for InitTokenConfig
pub struct InitTokenConfigInstruction<'a> {
    pub accounts: TokenConfigAccounts<'a>,
    pub instruction_datas: InitTokenConfig,
}

impl<'a> TryFrom<&'a [AccountInfo]> for TokenConfigAccounts<'a> {
    type Error = ProgramError;

    fn try_from(infos: &'a [AccountInfo]) -> Result<Self, Self::Error> {
        let mut iter = infos.iter();
        let [signer, global_config, creator_kyc, token_config] = infos else {
                    return Err(RWAError::NotEnoughAccountKeys.into());
        };
        let signer = SignerAccount::try_from(&signer);
        let global_config = ProgramAccount::try_from(global_config);
        let creator_kyc = ProgramAccount::try_from(creator_kyc);
        let token_config = ProgramAccount::try_from(token_config);

        Ok(Self {
            signer,
            global_config,
            creator_kyc,
            token_config
        })
    }
}

impl<'a> TryFrom<(&'a [u8], &'a [AccountInfo])> for InitTokenConfigInstruction<'a> {
    type Error = ProgramError;

    fn try_from((data, accounts): (&'a [u8], &'a [AccountInfo])) -> Result<Self, Self::Error> {
        let accounts = TokenConfigAccounts::try_from(accounts);
        let instruction_datas = InitTokenConfig::try_from(data); // deserialize instruction data

        Ok(Self {
            accounts,
            instruction_datas,
        })
    }
}

impl<'a> InitTokenConfigInstruction<'a> {
    pub const DISCRIMINATOR: u8 = 5;
    pub const SEED_PREFIX: &'static [u8] = b"token-program";

    pub fn process(&mut self, program_id: &Pubkey) -> ProgramResult {
        // Derive PDA for token config
        let (expected_pda, bump) = Pubkey::find_program_address(&[Self::SEED_PREFIX], self.accounts.creator_kyc.key().as_ref(), program_id);

        if *self.accounts.token_config.key() != expected_pda {
            return Err(ProgramError::InvalidAccountData);
        }
        let seeds = seeds!(b"token-program", self.accounts.creator_kyc.key().as_ref(), core::mem::size_of::<TokenConfig>())?;
        ProgramAccount::init_if_needed(self.accounts.signer, self.accounts.token_config, seeds,  core::mem::size_of::<TokenConfig>())?;

        // Borrow PDA data mutably
        let token_config_data = &mut self.accounts.token_config.try_borrow_mut_data()?;
        let token_config: &mut TokenConfig = unsafe { load_acc_mut_unchecked(token_config_data)? };

        let global_config_data = &mut self.accounts.global_config.try_borrow_mut_data()?;
        let global_config: &mut GlobalConfig = unsafe { load_acc_mut_unchecked(global_config_data)? };

        if self.instruction_datas.decimals <= global_config.max_decimal {
            return Err(RWAError::InvalidInstructionData.into())
        };

        let creator_kyc_data = &mut self.accounts.creator_kyc.try_borrow_mut_data()?;
        let creator_kyc: &mut CreatorKYC = unsafe { load_acc_mut_unchecked(creator_kyc_data)? };

        // Write instruction data into PDA struct
        token_config.creator = self.instruction_datas.creator;
        token_config.active = false;
        token_config.description = self.instruction_datas.description;
        token_config.asset_type = self.instruction_datas.asset_type;
        token_config.audit_cid = self.instruction_datas.audit_cid;
        token_config.bump = bump;
        token_config.decimal = self.instruction_datas.decimals;

        Ok(())
    }
}
