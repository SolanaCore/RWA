#![allow(unexpected_cfgs)]

use crate::{
    instructions::{self, RWAInstruction},
    errors::RWAError,
    pinocchio::{
    account_info::AccountInfo, default_panic_handler, msg, no_allocator, program_entrypoint,
    program_error::ProgramError, pubkey::Pubkey, ProgramResult,
   },
   pinocchio_log::log
};

// This is the entrypoint for the program.
program_entrypoint!(process_instruction);
//Do not allocate memory.
no_allocator!();
// Use the no_std panic handler.
default_panic_handler!();

#[inline(always)]
fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let (ix_disc, instruction_data) = instruction_data
        .split_first()
        .ok_or(RWAError::InvalidInstructionData)?;

        match ix_disc {
            0 => {
                #[cfg(not(feature = "perf"))]
                log!("INIT_GLOBAL_CONFIG");

                let mut ix = InitGlobalConfigInstruction::try_from((rest, accounts))?;
                ix.process(program_id)
                Ok(())
            },

            1 => {
                #[cfg(not(feature = "perf"))]
                log!("UPDATE_GLOBAL_CONFIG");
                // let mut ix = UpdateGlobalConfigInstruction::try_from((rest, accounts))?;
                // ix.process(program_id)
                Ok(())
            },

            2 => {
                #[cfg(not(feature = "perf"))]
                log!("CREATOR_KYC");
                let mut ix = CreatorKYCInstruction::try_from((rest, accounts))?;
                ix.process(program_id)
                Ok(())
            },
            3 => {
                #[cfg(not(feature = "perf"))]
                log!("VERIFY_CREATOR_KYC");
                // let mut ix = VerifyCreatorKYCInstruction::try_from((rest, accounts))?;
                // ix.process(program_id)
                Ok(())
            },
            4 => {
                #[cfg(not(feature = "perf"))]
                log!("INIT_TOKEN_CONFIGURATION");
                let mut ix = InitTokenConfigInstruction::try_from((rest, accounts))?;
                ix.process(program_id)
                Ok(())
            },
            5 => {
                #[cfg(not(feature = "perf"))]
                log!("VERIFY_TOKEN_CONFIGURATION");
                // let mut ix = VerifyTokenConfigInstruction::try_from((rest, accounts))?;
                // ix.process(program_id)
                Ok(())
            },
            6 => {
                #[cfg(not(feature = "perf"))]
                log!("CREATE_RWA_MINT & METADATA");
                // let mut ix = CreateRWAInstruction::try_from((rest, accounts))?;
                // ix.process(program_id)
                Ok(())
            },
            7 => {
                #[cfg(not(feature = "perf"))]
                log!("MINT_RWA_TOKEN");
                // let mut ix = MintRWAInstruction::try_from((rest, accounts))?;
                // ix.process(program_id)
                Ok(())
            },
            //batch processing 
            255 => {
                #[cfg(not(feature = "perf"))]
                log!("BATCH PROCESSING - FEAT: COMING SOON");
                Ok(())
            },
            _ => {
                log!("FALLBACK INSTRUCTION");
                Ok(())
            }
        }
       }
}