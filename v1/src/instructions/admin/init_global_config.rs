use {
    core::convert::TryFrom,
    pinocchio::{
        account_info::AccountInfo,
        instruction::{Signer},
        program_error::ProgramError,
        pubkey::find_program_address,
        seeds,
    },
    crate::{
        instructions::{
            RWAInstruction, InitGlobalConfig
        }, // or `InitGlobalConfig` if you only need that variant
        utils::{ProgramAccount, SignerAccount, load_acc_mut_unchecked},
        errors::RWAError,
    },
};


/// Struct holding all relevant accounts for InitGlobalConfig
pub struct GlobalConfigAccounts<'a> {
    pub config_authority: &'a AccountInfo,
    pub kyc_authority: &'a AccountInfo,
    pub global_config: &'a AccountInfo,
}

impl<'a> TryFrom<&'a [AccountInfo]> for GlobalConfigAccounts<'a> {
    type Error = ProgramError;

    fn try_from(infos: &'a [AccountInfo]) -> Result<Self, Self::Error> {
        let mut iter = infos.iter();
        let [config_authority, kyc_authority, global_config]= infos else {
                    return Err(RWAError::NotEnoughAccountKeys.into());
        }
        // Extract and validate accounts
        let config_authority = SignerAccount::try_from(config_authority);
        let kyc_authority = SignerAccount::try_from(kyc_authority);
        let global_config =  ProgramAccount::try_from(global_config);

        Ok(Self {
            &config_authority,
            &kyc_authority,
            &global_config,
        })
    }
}

/// Instruction wrapper
pub struct InitGlobalConfigInstruction<'a> {
    pub accounts: GlobalConfigAccounts<'a>,
    pub instruction_datas: InitGlobalConfig,
}

impl<'a> TryFrom<(&'a [u8], &'a [AccountInfo])> for InitGlobalConfigInstruction<'a> {
    type Error = ProgramError;

    fn try_from((data, accounts): (&'a [u8], &'a [AccountInfo])) -> Result<Self, Self::Error> {
        let accounts = GlobalConfigAccounts::try_from(accounts)?;
        let instruction_datas = InitGlobalConfig::try_from(data)?; // deserialize your instruction data

        Ok(Self {
            accounts,
            instruction_datas,
        })
    }
}

impl<'a> InitGlobalConfigInstruction<'a> {
    pub const DISCRIMINATOR: u8 = 4;
    
    /// Process the instruction: initialize the GlobalConfig PDA
     fn process(&mut self, program_id: &Pubkey) -> Result<(), ProgramError> {
        // Derive the expected PDA
        let (expected_pda, bump) = find_program_address(&[b"global-config"], program_id);

        // Check PDA matches the account passed in
        if *self.accounts.global_config.key() != expected_pda {
            return Err(ProgramError::InvalidAccountData);
        }
        let seeds = seeds!("global-config", program_id);
        let signer_seeds = Signer::from(&seeds);

        ProgramAccount::init_if_needed(self.accounts.signer, self.accounts.global_config, &signer_seeds, core::mem::size_of<GlobalConfig>);
        // Borrow PDA data mutably
        let global_config_data = &mut self.accounts.global_config.try_borrow_mut_data()?;
        let global_config: &mut crate::states::GlobalConfig =
            unsafe { load_acc_mut_unchecked(global_config_data)};

        // Write instruction data into the PDA
        global_config.config_authority = *self.accounts.config_authority.key();
        global_config.kyc_authority = *self.accounts.kyc_authority.key();
        global_config.fees_bps = self.instruction_datas.fee_bps;
        global_config.max_decimal = self.instruction_datas.max_decimal;
        global_config.open_time = self.instruction_datas.open_time;
        global_config.active = self.instruction_datas.active;
        global_config.bump = bump;

        Ok(())
    }
}
