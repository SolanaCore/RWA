use {
    pinocchio::{
        account_info::AccountInfo,
        instruction::next_account_info,
        program_error::ProgramError,
        ProgramResult,
    },
    pinocchio_log,
    crate::{
        instructions::CreateRWA,
        utils::{
            ProgramAccount, SignerAccount, SystemAccount,Mint2022Account,
        },
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

impl<'a> TryFrom<&'a [AccountInfo<'a>]> for CreateRWAAccount<'a> {
    type Error = ProgramError;

    fn try_from(infos: &'a [AccountInfo<'a>]) -> Result<Self, Self::Error> {
        let mut iter = infos.iter();

        let signer = SignerAccount::try_from(next_account_info(&mut iter)?)?;
        let mint_account = Mint2022Account::check(next_account_info(&mut iter)?)?;
        let metadata_account  = next_account_info(&mut iter)?;
        let mint_authority = SystemAccount::check(next_account_info(&mut));
        let freeze_authority =  // SystemAccount::check(next_account_info(&mut));
        let token_program_2022 = SystemAccount::check(next_account_info(&mut));
        
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
            let accounts = CreateRWAAccount::try_from(accounts)?;
            let instruction_datas = CreateRWA::try_from(data)?;
    
            Ok(Self {
                accounts,
                instruction_datas,
            })
        }
    }

    impl CreateRWAInstruction<'a> {
        pub const DISCRIMINATOR: usize = 1;
        pub fn process() -> ProgramResult<> {

        }
    }

