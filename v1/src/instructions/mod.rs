use pinocchio::program_error::ProgramError;
use crate::errors::MyProgramError;

pub mod create_rwa;
pub use create_rwa::*;

pub mod creator_kyc;
pub use creator_kyc::*;

pub mod init_token_config;
pub use init_token_config::*;

pub mod mint_rwa;
pub use mint_rwa::*;

pub mod admin;
pub use admin::*;


/// Instruction enum
#[repr(u8)]
#[derive(shank::ShankInstruction)]
pub enum RWAInstruction {
    #[account(0, name = "Signer")]
    InitGlobalConfig = 0,

    #[account(0, name = "Signer")]
    InitTokenConfig  = 1,

    #[account(0, name = "Signer")]
    CreateRWA        = 2,

    #[account(0, name = "Signer")]
    MintRWA          = 3,

    #[account(0, name = "Signer")]
    CreatorKYC       = 4,

}

/// Struct for InitGlobalConfig
#[repr(C)]
pub struct InitGlobalConfig {
    pub bump: u8,
    pub open_time: u64,
    pub max_decimal: u8,
    pub fees_bps: u8,
    pub active: bool,
}

impl DataLen for InitGlobalConfig {
    const LEN: usize = core::mem::size_of::<Self>();
}

impl<'a> ZeroCopyTryFrom<'a> for InitGlobalConfig {
    fn try_from_bytes(data: &'a [u8]) -> Result<&'a Self, ProgramError> {
        unsafe { load_ix_data::<Self>(data) }
    }
}

/// Struct for CreatorKYC
#[repr(C)]
pub struct CreatorKYC {
    pub name: [u8; 32],   // fixed-length, UTF-8 string
    pub email: [u8; 32],  // fixed-length, UTF-8 string
    pub wallet: [u8; 32],
}

impl DataLen for CreatorKYC {
    const LEN: usize = core::mem::size_of::<Self>();
}

impl<'a> ZeroCopyTryFrom<'a> for CreatorKYC {
    fn try_from_bytes(data: &'a [u8]) -> Result<&'a Self, ProgramError> {
        unsafe { load_ix_data::<Self>(data) }
    }
}

/// Struct for MintRWA
#[repr(C)]
pub struct MintRWA {
    pub amount: u64,
}

impl DataLen for MintRWA {
    const LEN: usize = core::mem::size_of::<Self>();
}

impl<'a> ZeroCopyTryFrom<'a> for MintRWA {
    fn try_from_bytes(data: &'a [u8]) -> Result<&'a Self, ProgramError> {
        unsafe { load_ix_data::<Self>(data) }
    }
}

/// Struct for CreateRWA
#[repr(C)]
pub struct CreateRWA {
    pub supply: u64,
    pub decimals: u8,
}

impl DataLen for CreateRWA {
    const LEN: usize = core::mem::size_of::<Self>();
}

impl<'a> ZeroCopyTryFrom<'a> for CreateRWA {
    fn try_from_bytes(data: &'a [u8]) -> Result<&'a Self, ProgramError> {
        unsafe { load_ix_data::<Self>(data) }
    }
}

/// Trait for getting struct size
pub trait DataLen {
    const LEN: usize;
}

/// Zero-copy trait
pub trait ZeroCopyTryFrom<'a>: Sized {
    fn try_from_bytes(data: &'a [u8]) -> Result<&'a Self, ProgramError>;
}

/// Zero-copy helpers
#[inline(always)]
pub unsafe fn load_acc_unchecked<T: DataLen>(bytes: &[u8]) -> Result<&T, ProgramError> {
    if bytes.len() != T::LEN {
        return Err(ProgramError::InvalidAccountData);
    }
    Ok(&*(bytes.as_ptr() as *const T))
}

#[inline(always)]
pub unsafe fn load_acc_mut_unchecked<T: DataLen>(bytes: &mut [u8]) -> Result<&mut T, ProgramError> {
    if bytes.len() != T::LEN {
        return Err(ProgramError::InvalidAccountData);
    }
    Ok(&mut *(bytes.as_mut_ptr() as *mut T))
}

#[inline(always)]
pub unsafe fn load_ix_data<T: DataLen>(bytes: &[u8]) -> Result<&T, ProgramError> {
    if bytes.len() != T::LEN {
        return Err(MyProgramError::InvalidInstructionData.into());
    }
    Ok(&*(bytes.as_ptr() as *const T))
}

pub unsafe fn to_bytes<T: DataLen>(data: &T) -> &[u8] {
    core::slice::from_raw_parts(data as *const T as *const u8, T::LEN)
}

pub unsafe fn to_mut_bytes<T: DataLen>(data: &mut T) -> &mut [u8] {
    core::slice::from_raw_parts_mut(data as *mut T as *mut u8, T::LEN)
}
