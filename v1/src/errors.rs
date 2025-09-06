// errors.rs
use {
    num_derive::FromPrimitive,
    pinocchio::program_error::{ProgramError, ToStr},
    shank::ShankType,
    thiserror_no_std::Error,
};
/// Custom program errors specific to your app
#[derive(Clone, Debug, Eq, Error, FromPrimitive, PartialEq, ShankType)]
pub enum MyProgramError {
    #[error("Invalid instruction data")]
    InvalidInstructionData,

    #[error("PDA mismatch")]
    PdaMismatch,

    #[error("Invalid account owner")]
    InvalidOwner,

    #[error("Expected a signer")]
    NotSigner,

    #[error("Invalid address")]
    InvalidAddress,

    #[error("Invalid account data")]
    InvalidAccountData,

    #[error("The pda doesn't exist onchain because it's balance is 0")]
    UninitialisedAccount
}


impl From<MyProgramError> for ProgramError {
    fn from(e: MyProgramError) -> Self {
        ProgramError::Custom(e as u32)
    }
}

impl TryFrom<u32> for MyProgramError {
    type Error = ProgramError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(MyProgramError::InvalidInstructionData),
            1 => Ok(MyProgramError::PdaMismatch),
            2 => Ok(MyProgramError::InvalidOwner),
            3 => Ok(MyProgramError::NotSigner),
            4 => Ok(MyProgramError::InvalidAddress),
            5 => Ok(MyProgramError::InvalidAccountData),
            6 => Ok(MyProgramError::UninitialisedAccount),
            _ => Err(ProgramError::InvalidArgument),
        }
    }
}

impl ToStr for MyProgramError {
    fn to_str<E>(&self) -> &'static str {
        match self {
            MyProgramError::InvalidInstructionData => "Invalid instruction data",
            MyProgramError::PdaMismatch => "PDA mismatch",
            MyProgramError::InvalidOwner => "Invalid account owner",
            MyProgramError::NotSigner => "Expected a signer",
            MyProgramError::InvalidAddress => "Invalid address",
            MyProgramError::InvalidAccountData => "Invalid account data",
            MyProgramError::UninitialisedAccount => "The pda doesn't exist onchain because it's balance is 0",
        }
    }
}