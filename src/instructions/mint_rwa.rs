use {
    pinocchio::account_info::AccountInfo,
};

#[repr(C)]
#[derive(Clone, Copy)]
pub struct MintRWAAccount<'a> {
    pub signer: &'a AccountInfo,
}