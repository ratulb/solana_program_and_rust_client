use borsh::{BorshDeserialize, BorshSerialize};
use common::instruction::CounterInstruction;
use common::state::Counter;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

pub struct Processor {}

impl Processor {
    pub fn process_instruction(
        _program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        let instruction = CounterInstruction::try_from_slice(instruction_data)
            .map_err(|_| ProgramError::InvalidInstructionData)?;
        match instruction {
            CounterInstruction::Increament => {
                msg!("Instruction increament");
                let accounts_iter = &mut accounts.iter();
                let counter_ai = next_account_info(accounts_iter)?;
                let mut counter = Counter::try_from_slice(&counter_ai.data.borrow())?;
                counter.count += 1;
                msg!("Updating counter {}", counter.count);
                counter.serialize(&mut *counter_ai.data.borrow_mut())?;
            }
        }

        Ok(())
    }
}
