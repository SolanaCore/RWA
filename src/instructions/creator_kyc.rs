use {
    crate::{
        utils::{SignerAccount, ProgramAccount, load_acc_mut_unchecked, ProgramAccountInit},
        states::CreatorKYC,
        errors::RWAError
    },
    pinocchio::{
        account_info::AccountInfo,
        pubkey::find_program_address,
        program_error::ProgramError,
        seeds,
        Signer,
    },
    core::convert::TryFrom,
    core::io::Read,

};

/// Struct holding all relevant accounts for InitGlobalConfig
pub struct CreatorKYCAccount<'a> {
    pub signer: &'a AccountInfo,
    pub creator_kyc: &'a AccountInfo,
}

impl<'a> TryFrom<&'a [AccountInfo]> for CreatorKYCAccount<'a> {
    type Error = ProgramError;

    fn try_from(infos: &'a [AccountInfo]) -> Result<Self, Self::Error> {
        let [signer, creator_kyc] = infos else {
            return Err(RWAError::NotEnoughAccountKeys.into());
        };
        let mut iter = infos.iter();

        let signer = SignerAccount::try_from(signer);

        Ok(Self { signer, &creator_kyc })
    }
}

/// Instruction wrapper
pub struct CreatorKYCInstruction<'a> {
    pub accounts:  CreatorKYCAccount<'a>,
    pub instruction_datas:  CreatorKYC,
}

impl<'a> TryFrom<(&'a [u8], &'a [AccountInfo])> for CreatorKYCInstruction<'a> {
    type Error = ProgramError;

    fn try_from((data, accounts): (&'a [u8], &'a [AccountInfo])) -> Result<Self, Self::Error> {
        let accounts = CreatorKYCAccount::try_from(accounts);
        let instruction_datas = CreatorKYC::try_from(data); // deserialize your instruction data

        Ok(Self {
            accounts,
            instruction_datas,
        })
    }
}

impl<'a> CreatorKYCInstruction<'a> {
    pub const DISCRIMINATOR: u8 = 4;

    /// Process the instruction: initialize the GlobalConfig PDA
    pub fn process(&mut self, program_id: &Pubkey) -> Result<(), ProgramError> {
        // Derive the expected PDA
        let (expected_pda, bump) = find_program_address(&[b"creator-kyc"], self.accounts.creator_kyc.key().as_ref(),  program_id);

        // Check PDA matches the account passed in
        if *self.accounts.creator_kyc.key() != expected_pda {
            return Err(RWAError::InvalidAccountData.into());
        }
        let seeds_array = seeds!(b"creator-kyc", self.accounts.creator_kyc.key().as_ref(), program_id);
        let signer_pda = Signer::from(&seeds_array);

        ProgramAccount::init_if_needed(self.accounts.signer, self.accounts.creator_kyc, signer_pda, size_of::<CreatorKYC>());

        let creator_kyc_data = self.accounts.creator_kyc.try_borrow_mut_data()?;
        let creator_kyc = unsafe { load_acc_mut_unchecked<CreatorKYC>(creator_kyc_data)};
        // Write instruction data into the PDA
        creator_kyc.name = self.instruction_datas.name.bytes();
        creator_kyc.email_id = self.instruction_datas.email_id.bytes();
        creator_kyc.wallet = *self.accounts.signer.key().as_ref();
        //Default - false, until the config authority verifies the user_kyc by doing some offchain verification...
        creator_kyc.verified = false;
        creator_kyc.bump = self.instruction_datas.bump;
        Ok(())
    }
}
