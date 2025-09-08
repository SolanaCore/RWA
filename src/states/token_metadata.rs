use {
    crate::utils::DataLen,
}

#[repr(c)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Metadata {
    pub mint: Pubkey,
    pub update_authority: Pubkey,
    pub token_config: Pubkey,  // 32
    pub name: [u8; 32],        // 32
    pub symbol: [u8; 10],      // 10
    pub uri: [u8; 200],        // 200
    pub immutable : bool,
    pub bump: u8,              // 1
}


impl DataLen for Metadata {
    pub const LEN:usize = core::mem::size_of<Metadata>;
}