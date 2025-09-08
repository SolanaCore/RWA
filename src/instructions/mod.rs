use {
    pinocchio::{
        program_error::ProgramError,
        pubkey::Pubkey,
    },
    crate::{
        errors::RWAError,
        utils::{load_ix_data, DataLen}
    },
};

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


/// Zero-copy trait
pub trait ZeroCopyTryFrom<'a>: Sized {
    fn try_from_bytes(data: &'a [u8]) -> Result<&'a Self, ProgramError>;
}


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

/// Init Global Config
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

#[repr(C)]
pub struct InitTokenConfig {
    pub bump: u8, 
    pub creator: Pubkey,         // 32 bytes
    pub mint: Pubkey,
    pub decimals: u8,
    //Default: false - it is set to true flag after the developers verify this token is legitimate
    pub description: [u8; 128],  // fixed max length (128 chars)
    pub asset_type: [u8; 8],          // e.g. gold, real_estate or some sort of physical asset
    pub audit_cid: [u8; 32],     // compact audit link (like IPFS CID)
    pub active: bool,
}

impl DataLen for InitTokenConfig {
    const LEN: usize = core::mem::size_of::<Self>();
}

impl<'a> ZeroCopyTryFrom<'a> for InitTokenConfig {
    fn try_from_bytes(data: &'a [u8]) -> Result<&'a Self, ProgramError> {
        unsafe { load_ix_data::<Self>(data) }
    }
}
/// Init Creator KYC
#[repr(C)]
pub struct CreatorKYC {
    pub name:  [u8; 32],   // fixed-length, UTF-8 string
    pub email:  [u8; 32],  // fixed-length, UTF-8 string
    pub wallet:  [u8; 32],
}

impl DataLen for CreatorKYC {
    const LEN: usize = core::mem::size_of::<Self>();
}

impl<'a> ZeroCopyTryFrom<'a> for CreatorKYC {
    fn try_from_bytes(data: &'a [u8]) -> Result<&'a Self, ProgramError> {
        unsafe { load_ix_data::<Self>(data) }
    }
}

/// Struct for Create RWA
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


/// Mint RWA
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
