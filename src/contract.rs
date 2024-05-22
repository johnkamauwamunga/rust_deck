use solana_program::{

    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey
};

//declare amd expopry tyhe programs entrypoint
entrypoint entrypoint!(process_instruction);

//program entrypoints implementation.

fn process_instruction(
    program_id:&Pubkey,
    accounts:&[AccountInfo],
    instruction_data:&[u8],
) -> ProgramResult{
    // iterating accounts is safer than indexing
    let account = next_account_info(accounts_iter)?;

    // decode the instruction data 
    let instruction = instruction_data[0];

    if instruction == 0{
        msg!("Sending funds...")

    // logic  for sending funds here
    }else if instruction ==1{
    msg!("Withdrawing funds...");
    // withdra logic here
    } else if instruction ==2{
        msg!("Getting balance ...");

        // logic to fetch bal here
    }else{
        msg!("Unknown instruction");

        return Err(ProgramError::invalidInstructiondata);
    }

    Ok(())
}