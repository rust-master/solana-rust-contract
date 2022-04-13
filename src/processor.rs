use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

use crate::instruction::Addition;
use crate::state::Adder;

pub struct Processor {}

impl Processor {
    pub fn process_instruction(
        _program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        let instruction = Addition::try_from_slice(instruction_data)
            .map_err(|_| ProgramError::InvalidInstructionData)?;

        match instruction {
            Addition::Add => {
                msg!("Instruction Add");
                let accounts_iter = &mut accounts.iter();
                let add_ai = next_account_info(accounts_iter)?;
                let mut addition = Adder::try_from_slice(&add_ai.data.borrow())?;
                addition.a = 10;
                addition.b = 16;
                addition.result = addition.a + addition.b;
                msg!("Result {}", addition.result);
                addition.serialize(&mut *add_ai.data.borrow_mut())?;
            }
        }
        Ok(())
    }
}
