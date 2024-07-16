mod error;

use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    msg,
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    sysvar::{rent::Rent, Sysvar},
    system_instruction::transfer,
    program_pack::{IsInitialized, Pack, Sealed},
    program_error::ProgramError,
    program::invoke,
};

use std::convert::TryInto;

// program entrypoint's implementation
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8]
) -> ProgramResult {
    let _ = Processor::process(program_id, accounts, instruction_data);
 
    // gracefully exit the program
    Ok(())
}

// declare and export the program's entrypoint
entrypoint!(process_instruction);


pub struct Processor;
impl Processor {
    pub fn process(program_id: &Pubkey, accounts: &[AccountInfo], instruction_data: &[u8]) -> ProgramResult {
        let instruction = CustomInstruction::unpack(instruction_data)?;

        match instruction {
            CustomInstruction::Mint { amount } => {
                msg!("Instruction: Mint");
                Self::process_mint(accounts, amount, program_id)
            }
            CustomInstruction::Burn { amount } => {
                msg!("Instruction: Burn");
                Self::process_burn(accounts, amount, program_id)
            }
        }
    }

    fn process_mint(
        accounts: &[AccountInfo],
        amount: u64,
        _program_id: &Pubkey,
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();

        let from_account = next_account_info(account_info_iter)?;

        let to_account = next_account_info(account_info_iter)?;

        let ix = transfer(from_account.key, to_account.key, 1000000000);

        invoke(
            &ix,
            &[from_account.clone(), to_account.clone()], // accounts required by instruction
        )?;

        // if !initializer.is_signer {
        //     return Err(ProgramError::MissingRequiredSignature);
        // }

        // let token_to_receive_account = next_account_info(account_info_iter)?;
        // if *token_to_receive_account.owner != spl_token::id() {
        //     return Err(ProgramError::IncorrectProgramId);
        // }

        // let escrow_account = next_account_info(account_info_iter)?;
        // let rent = &Rent::from_account_info(next_account_info(account_info_iter)?)?;

        // if !rent.is_exempt(escrow_account.lamports(), escrow_account.data_len()) {
        //     return Err(error::EscrowError::NotRentExempt.into());
        // }

        // let mut escrow_info = Escrow::unpack_unchecked(&escrow_account.try_borrow_data()?)?;
        // if escrow_info.is_initialized() {
        //     return Err(ProgramError::AccountAlreadyInitialized);
        // }

        Ok(())
    }

    fn process_burn(
        accounts: &[AccountInfo],
        amount: u64,
        _program_id: &Pubkey,
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();

        let from_account = next_account_info(account_info_iter)?;

        let to_account = next_account_info(account_info_iter)?;

        // let ix = transfer(from_account.key, to_account.key, 1000000000);

        // invoke(
        //     &ix,
        //     &[from_account.clone(), to_account.clone()], // accounts required by instruction
        // )?;

        if **from_account.try_borrow_lamports()? < 1000000000 {
            return Err(error::EscrowError::InvalidInstruction.into());
        }
        // Debit from_account and credit to_account
        **from_account.try_borrow_mut_lamports()? -= 1000000000;
        **to_account.try_borrow_mut_lamports()? += 1000000000;

        Ok(())
    }
}

use arrayref::{array_mut_ref, array_ref, array_refs, mut_array_refs};

pub struct Escrow {
    pub is_initialized: bool,
    pub initializer_pubkey: Pubkey,
    pub temp_token_account_pubkey: Pubkey,
    pub initializer_token_to_receive_account_pubkey: Pubkey,
    pub expected_amount: u64,
}

impl Sealed for Escrow {}

impl Pack for Escrow {
    const LEN: usize = 105;
    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        let src = array_ref![src, 0, Escrow::LEN];
        let (
            is_initialized,
            initializer_pubkey,
            temp_token_account_pubkey,
            initializer_token_to_receive_account_pubkey,
            expected_amount,
        ) = array_refs![src, 1, 32, 32, 32, 8];
        let is_initialized = match is_initialized {
            [0] => false,
            [1] => true,
            _ => return Err(ProgramError::InvalidAccountData),
        };

        Ok(Escrow {
            is_initialized,
            initializer_pubkey: Pubkey::new_from_array(*initializer_pubkey),
            temp_token_account_pubkey: Pubkey::new_from_array(*temp_token_account_pubkey),
            initializer_token_to_receive_account_pubkey: Pubkey::new_from_array(
                *initializer_token_to_receive_account_pubkey,
            ),
            expected_amount: u64::from_le_bytes(*expected_amount),
        })
    }

    fn pack_into_slice(&self, dst: &mut [u8]) {
        let dst = array_mut_ref![dst, 0, Escrow::LEN];
        let (
            is_initialized_dst,
            initializer_pubkey_dst,
            temp_token_account_pubkey_dst,
            initializer_token_to_receive_account_pubkey_dst,
            expected_amount_dst,
        ) = mut_array_refs![dst, 1, 32, 32, 32, 8];

        let Escrow {
            is_initialized,
            initializer_pubkey,
            temp_token_account_pubkey,
            initializer_token_to_receive_account_pubkey,
            expected_amount,
        } = self;

        is_initialized_dst[0] = *is_initialized as u8;
        initializer_pubkey_dst.copy_from_slice(initializer_pubkey.as_ref());
        temp_token_account_pubkey_dst.copy_from_slice(temp_token_account_pubkey.as_ref());
        initializer_token_to_receive_account_pubkey_dst
            .copy_from_slice(initializer_token_to_receive_account_pubkey.as_ref());
        *expected_amount_dst = expected_amount.to_le_bytes();
    }
}

impl IsInitialized for Escrow {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}

pub enum CustomInstruction {
    // Starts the trade by creating and populating an escrow account and transferring ownership of the given temp token account to the PDA
    //
    //
    // Accounts expected:
    //
    // 0. `[signer]` The account of the person initializing the escrow
    // 1. `[writable]` Temporary token account that should be created prior to this instruction and owned by the initializer
    // 2. `[]` The initializer's token account for the token they will receive should the trade go through
    // 3. `[writable]` The escrow account, it will hold all necessary info about the trade.
    // 4. `[]` The rent sysvar
    // 5. `[]` The token program
    Mint {
        // The amount party A expects to receive of token Y
        amount: u64
    },
    Burn {
        // The amount party A expects to receive of token Y
        amount: u64
    },
}

impl CustomInstruction {
    /// Unpacks a byte buffer into a [CustomInstruction](enum.CustomInstruction.html).
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (tag, rest) = input.split_first().ok_or(error::EscrowError::InvalidInstruction)?;

        Ok(match tag {
            0 => Self::Mint {
                amount: Self::unpack_amount(rest)?,
            },
            1 => Self::Burn {
                amount: Self::unpack_amount(rest)?,
            },
            _ => return Err(error::EscrowError::InvalidInstruction.into()),
        })
    }

    fn unpack_amount(input: &[u8]) -> Result<u64, ProgramError> {
        let amount = input
            .get(..8)
            .and_then(|slice| slice.try_into().ok())
            .map(u64::from_le_bytes)
            .ok_or(error::EscrowError::InvalidInstruction)?;
        Ok(amount)
    }
}


