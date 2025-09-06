use pinocchio::account_info::AccountInfo;
#[repr(c)]
#[derive(Clone, Copy)]
pub struct MintRWAAccount<'a> {
    pub signer: &'a AccountInfo,
}