#![allow(unexpected_cfgs)]

use crate::{
    instructions::{self, RWAInstruction},
    errors::RWAError,
};
use pinocchio::{
    account_info::AccountInfo, default_panic_handler, msg, no_allocator, program_entrypoint,
    program_error::ProgramError, pubkey::Pubkey, ProgramResult,
};

// This is the entrypoint for the program.
program_entrypoint!(process_instruction);
//Do not allocate memory.
no_allocator!();
// Use the no_std panic handler.
default_panic_handler!();


#[inline(always)]
pub fn process_instruction(
    _program_id: &Pubkey,
    acccounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult{
Ok(())
}
// #[inline(always)]
// fn process_instruction(
//     _program_id: &Pubkey,
//     accounts: &[AccountInfo],
//     instruction_data: &[u8],
// ) -> ProgramResult {
//     let (ix_disc, instruction_data) = instruction_data
//         .split_first()
//         .ok_or(RWAError::InvalidInstructionData)?;

//     // match RWAInstruction::try_from(ix_disc)? {
//     //     RWAInstruction::InitializeState => {
//     //         msg!("init_global_cinfig");
//     //         init_global_config(accounts, instruction_data)
//     //     },
//     //     RWAInstruction::InitTokenConfig => {
//     //         msg!("Init_token_config");
//     //         init_token_config(accounts, instruction_data)
//     //     },
//     //     RWAInstruction::CreateRWA => {
//     //         msg!("Create RWA");
//     //         create_rwa(accounts, instruction_data);
//     //     },
//     //     RWAInstruction::MintRWA => {
//     //         msg!("Mint RWA");
//     //         mint_rwa(accounts, instruction_data);
//     //     },
//     // }
// }