use {
    pinocchio::pubkey::Pubkey,
    crate::{
        instructions::RWAInstruction::InitGlobalConfig,
        utils::DataLen,
    },
};


#[repr(C)] 
#[derive(Clone, Copy, Debug, PartialEq, shank::ShankAccount)]
pub struct GlobalConfig {
    pub config_authority: Pubkey,
    pub kyc_authority: Pubkey,
    pub active: bool,
    pub open_time: u64,
    pub max_decimal: u8,
    pub fees_bps: u8,  
    pub bump: u8,
}


impl DataLen for GlobalConfig {
const LEN: usize = core::mem::size_of::<GlobalConfig>();
 }
