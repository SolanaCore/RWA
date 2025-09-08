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
    pub decimal: u8,
    //Default: false - it is set to true flag after the developers verify this token is legitimate
    pub description: [u8; 128],  // fixed max length (128 chars)
    pub asset_type: [u8; 8],          // e.g. gold, real_estate or some sort of physical asset
    pub audit_cid: [u8; 32],     // compact audit link (like IPFS CID)
    pub active: bool,            // 1 byte
}

impl DataLen for TokenConfig {
    const LEN: usize = core::mem::size_of::<TokenConfig>();
    // = 203 bytes
}
