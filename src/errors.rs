// errors.rs
use {
    num_derive::FromPrimitive,
    pinocchio::program_error::{ProgramError, ToStr},
    shank::ShankType,
    thiserror_no_std::Error,
};
/// Custom program errors specific to your app
#[derive(Clone, Debug, Eq, Error, FromPrimitive, PartialEq, ShankType)]
pub enum RWAError {
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
    UninitialisedAccount,

    #[error("Not Enough Accounts passed that were required by the ix...")]
    NotEnoughAccountKeys
}


impl From<RWAError> for ProgramError {
    fn from(e: RWAError) -> Self {
        ProgramError::Custom(e as u32)
    }
}

impl TryFrom<u32> for RWAError {
    type Error = ProgramError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(RWAError::InvalidInstructionData),
            1 => Ok(RWAError::PdaMismatch),
            2 => Ok(RWAError::InvalidOwner),
            3 => Ok(RWAError::NotSigner),
            4 => Ok(RWAError::InvalidAddress),
            5 => Ok(RWAError::InvalidAccountData),
            6 => Ok(RWAError::UninitialisedAccount),
            7 => Ok(RWAError::NotEnoughAccountKeys)
            _ => Err(ProgramError::InvalidArgument),
        }
    }
}

impl ToStr for RWAError {
    fn to_str<E>(&self) -> &'static str {
        match self {
            RWAError::InvalidInstructionData => "Invalid instruction data",
            RWAError::PdaMismatch => "PDA mismatch",
            RWAError::InvalidOwner => "Invalid account owner",
            RWAError::NotSigner => "Expected a signer",
            RWAError::InvalidAddress => "Invalid address",
            RWAError::InvalidAccountData => "Invalid account data",
            RWAError::UninitialisedAccount => "The pda doesn't exist onchain because it's balance is 0",
            RWAError::NotEnoughAccountKeys => "Not Enough Accounts passed that were required by the ix..."
        }
    }
}