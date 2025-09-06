use pinocchio::pubkey::Pubkey;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CreatorKYC {
    pub name: [u8; 32],      // max 32 bytes - multiple of 8
    pub email_id: [u8; 32],  // max 32 bytes
    pub wallet: Pubkey,      // wallet identity of creator
    pub verified: bool,      // whether KYC is approved
    pub bump:u8,
}


pub trait DataLen {
    //we dont add pub prefix because items share the visibility of their trait... 
    const LEN: usize;
}
impl DataLen for CreatorKYC {
     const LEN: usize = core::mem::size_of::<CreatorKYC>(); 
}


