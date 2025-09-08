use {
    pinocchio::pubkey::Pubkey,
    crate::utils::DataLen,
};

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, shank::ShankAccount)]
pub struct TokenConfig {
    pub bump: u8, 
    pub creator: Pubkey,         // 32 bytes
    pub mint: Pubkey,
    //Default: false - it is set to true flag after the developers verify this token is legitimate
    pub active: bool,            // 1 byte
    pub description: [u8; 128],  // fixed max length (128 chars)
    pub asset: [u8; 8],          // e.g. gold, real_estate or some sort of physical asset
    pub audit_cid: [u8; 32],     // compact audit link (like IPFS CID)
}

impl DataLen for TokenConfig {
    const LEN: usize = core::mem::size_of::<TokenConfig>();
    // = 203 bytes
}
