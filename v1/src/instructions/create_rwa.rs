use {
    pinocchio::{
        account_info::AccountInfo,
        program_error::ProgramError,
        ProgramResult,
    },
    pinocchio_log,
    crate::{
        instructions::CreateRWA,
        utils::{
            ProgramAccount, SignerAccount, SystemAccount,Mint2022Account, AccountCheck
        },
        errors::RWAError,
    },
};

pub struct CreateRWAAccount<'a> {
    pub signer: &'a AccountInfo,
    pub mint_account: &'a AccountInfo, //mint account
    pub metadata_account: &'a AccountInfo, //metadata account to store the additional info for the mint
    pub mint_authority: &'a AccountInfo,
    pub freeze_authority: &'a AccountInfo,
    pub token_program_2022: &'a Option<AccountInfo>,
}

impl<'a> TryFrom<&'a [AccountInfo]> for CreateRWAAccount<'a> {
    type Error = ProgramError;

    fn try_from(infos: &'a [AccountInfo]) -> Result<Self, Self::Error> {
        let [signer, mint_account, metadata_account, mint_authority, freeze_authority, token_program_2022] = infos else {
            return Err(RWAError:NotEnoughAccountKeys.into())
        };
        let signer = SignerAccount::try_from(&signer)?;
        let mint_account = Mint2022Account::check(&mint_account)?;
        let metadata_account  = ProgramAccount::try_from(&metadata_account)?;
        let mint_authority = SystemAccount::try_from(&mint_authority);
        let freeze_authority =  SystemAccount::try_from(&freeze_authority);
        let token_program_2022 = SystemAccount::try_from(&token_program_2022)?;
        
        Ok(Self { signer, mint_account, metadata_account, mint_authority, freeze_authority, token_program_2022 });
    }
}

    pub struct CreateRWAInstruction<'a> {
        pub accounts: CreateRWAAccount<'a>,
        pub instruction_datas: CreateRWA,
    }

    impl<'a> TryFrom<(&'a [u8], &'a [AccountInfo])> for CreateRWAInstruction<'a> {
        type Error = ProgramError;
    
        fn try_from((data, accounts): (&'a [u8], &'a [AccountInfo])) -> Result<Self, Self::Error> {
            let accounts = CreateRWAAccount::try_from(accounts);
            let instruction_datas = CreateRWA::try_from(data);
    
            Ok(Self {
                accounts,
                instruction_datas,
            })
        }
    }
    //need to add lifetime
    impl CreateRWAInstruction {
        pub const DISCRIMINATOR: usize = 1;
        pub fn process() -> ProgramResult<> {

        }
    }

