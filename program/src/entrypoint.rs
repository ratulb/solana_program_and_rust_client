use crate::processor::Processor;
use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, msg, pubkey::Pubkey};

#[cfg(not(feature = "no-entrypoint"))]
use solana_program::entrypoint;

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    msg!(
        "process_instruction {}, account len {}, data {:?}",
        program_id,
        accounts.len(),
        instruction_data
    );
    Processor::process_instruction(program_id, accounts, instruction_data)
}
