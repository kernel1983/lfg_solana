mod error;

use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    msg,
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    sysvar::{rent::Rent, Sysvar},
    system_instruction,
    // system_program,
    program_pack::{IsInitialized, Pack, Sealed},
    program_error::ProgramError,
    program::invoke,
    program::invoke_signed,
};

use spl_token::{
    instruction::mint_to
};

use std::{
    convert::TryInto,
    mem,
    cmp
    // str,
};
// use std::cell::RefMut;
// use arrayref::{array_mut_ref, array_ref, array_refs, mut_array_refs};
use arrayref::{array_ref, array_refs};

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
            CustomInstruction::Setup => {
                msg!("Instruction: Setup");
                Self::process_setup(accounts, instruction_data, program_id)
            }
            CustomInstruction::Buy { amount } => {
                msg!("Instruction: Buy");
                msg!("amount {}", amount);
                Self::process_buy(accounts, amount, program_id)
            }
            CustomInstruction::Sell { amount } => {
                msg!("Instruction: Sell");
                Self::process_sell(accounts, amount, program_id)
            }
        }
    }

    fn process_setup(
        accounts: &[AccountInfo],
        instruction_data: &[u8],
        program_id: &Pubkey,
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();

        let from_account = next_account_info(account_info_iter)?;
        let app_account = next_account_info(account_info_iter)?;
        msg!("app_account {}", app_account.key);

        let seed_len:u8 = instruction_data[1];
        msg!("seed length {}", seed_len);
        let seed:&[u8] = &instruction_data[2..(2+seed_len) as usize];
        msg!("seed {:x?}", seed);
        let bump:u8 = instruction_data[(2+seed_len) as usize];
        msg!("bump {}", bump);
        let space:u16 = instruction_data.len() as u16 - 1;
        msg!("space {}", space);

        let rent_lamports = Rent::get()?.minimum_balance(space.try_into().expect("overflow"));

        invoke_signed(
            &system_instruction::create_account(
                &from_account.key,
                &app_account.key,
                rent_lamports,
                space.try_into().unwrap(),
                // &system_program::ID,
                program_id,
            ),
            &[
                from_account.clone(),
                app_account.clone(),
            ],
            // &[&[&from_account.key.as_ref(), seed, &[bump]]],
            &[&[seed, &[bump]]],
        )?;

        let mut app_data = app_account.try_borrow_mut_data().unwrap();
        (**app_data).copy_from_slice(&instruction_data[1..]);

        Ok(())
    }

    fn process_buy(
        accounts: &[AccountInfo],
        amount: u64,
        program_id: &Pubkey,
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();

        let from_account = next_account_info(account_info_iter)?;

        let app_account = next_account_info(account_info_iter)?;
        msg!("app_account {}", app_account.key);
        let app_data = app_account.try_borrow_data().unwrap();
        // msg!("data len {} bins {}", app_data.len(), (app_data.len() - 5) /mem::size_of::<Bin>());
        // msg!("app_data {:x?}", app_data);

        let seed_len:u8 = app_data[0];
        msg!("seed length {}", seed_len);
        let seed = app_data[1..(1+seed_len) as usize].to_vec();
        msg!("seed {:x?}", seed);
        let bump:u8 = app_data[(1+seed_len) as usize];
        msg!("bump {}", bump);

        let mut token_amount = 0;
        let mut amount_left = amount.clone();
        let mut i = 5;
        while i < app_data.len() {
            msg!("i {}", i);
            let mut bin:Bin = Bin::unpack_from_slice(&app_data[i..i + mem::size_of::<Bin>()]).unwrap();
            // msg!("price {} total {} amount {}", bin.price, bin.total, bin.amount); //token per lamport
            let amount_deduct = cmp::min(amount_left, ((bin.total - bin.amount) / bin.price as u128) as u64);
            amount_left -= amount_deduct;
            token_amount += amount_deduct * bin.price;
            // msg!("amount_left {} token_amount {}", amount_left, token_amount);
            bin.amount += (amount_deduct * bin.price) as u128;

            i += mem::size_of::<Bin>();
        }
        msg!("token_amount {}", token_amount);
        drop(app_data);

        let token_mint_account = next_account_info(account_info_iter)?;
        msg!("token_mint_account {}", token_mint_account.key);

        let token_account = next_account_info(account_info_iter)?;
        msg!("token_account {}", token_account.key);

        let system_program_account = next_account_info(account_info_iter)?;
        let token_program_account = next_account_info(account_info_iter)?;
        // let my_program_account = next_account_info(account_info_iter)?;

        // let ix_mint_to = mint_to(
        //     &spl_token::ID,
        //     token_mint_account.key,
        //     token_account.key,
        //     app_account.key,
        //     &[],
        //     token_amount).unwrap();
        let ix_mint_to = mint_to(
            &spl_token::ID,
            token_mint_account.key,
            token_account.key,
            app_account.key,
            &[],
            1000000000).unwrap();
        // https://docs.rs/spl-token/latest/spl_token/instruction/fn.mint_to.html
        // https://michaelhly.com/solana-py/spl/token/instructions/#spl.token.instructions.MintToParams
        invoke_signed(
            &ix_mint_to,
            &[
                app_account.clone(),
                token_mint_account.clone(),
                token_account.clone(),
                // from_account.clone(),
                // token_program_account.clone(),
            ],
            &[&[&seed, &[bump]]],
        )?;
        // https://docs.rs/solana-sdk/latest/solana_sdk/program/fn.invoke_signed.html

        // msg!("cost {} {} {}", amount, amount_left, amount - amount_left);
        // let ix_transfer = system_instruction::transfer(from_account.key, app_account.key, amount - amount_left);
        let ix_transfer = system_instruction::transfer(from_account.key, app_account.key, amount);
        invoke(
            &ix_transfer,
            &[
                from_account.clone(),
                app_account.clone(),
            ],
        )?;

        // let user_account = next_account_info(account_info_iter)?;
        //msg!(&str::from_utf8(&user_account.try_borrow_data().unwrap()).unwrap());
        // let mut data = user_account.try_borrow_mut_data().unwrap();
        // (**data).copy_from_slice(&amount.to_le_bytes());

        // let system_program_account = next_account_info(account_info_iter)?;

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
        //     return Err(error::InstructionError::NotRentExempt.into());
        // }

        // let mut escrow_info = Escrow::unpack_unchecked(&escrow_account.try_borrow_data()?)?;
        // if escrow_info.is_initialized() {
        //     return Err(ProgramError::AccountAlreadyInitialized);
        // }

        Ok(())
    }

    fn process_sell(
        accounts: &[AccountInfo],
        amount: u64,
        _program_id: &Pubkey,
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();

        let from_account = next_account_info(account_info_iter)?;

        let app_account = next_account_info(account_info_iter)?;

        // let ix = system_instruction::transfer(from_account.key, to_account.key, 1000000000);

        // invoke(
        //     &ix,
        //     &[from_account.clone(), to_account.clone()], // accounts required by instruction
        // )?;

        if **from_account.try_borrow_lamports()? < amount {
            return Err(error::InstructionError::InvalidInstruction.into());
        }
        // Debit from_account and credit to_account
        **from_account.try_borrow_mut_lamports()? -= amount;
        **app_account.try_borrow_mut_lamports()? += amount;

        Ok(())
    }
}

struct Bin {
    price: u64,
    total: u128,
    amount: u128,
}

// pub struct Escrow {
//     pub is_initialized: bool,
//     pub initializer_pubkey: Pubkey,
//     pub temp_token_account_pubkey: Pubkey,
//     pub initializer_token_to_receive_account_pubkey: Pubkey,
//     pub expected_amount: u64,
// }

impl Sealed for Bin {}

impl Pack for Bin {
    const LEN: usize = 40;
    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        let src = array_ref![src, 0, Bin::LEN];
        let (
            price_bytes,
            total_bytes,
            amount_bytes,
        ) = array_refs![src, 8, 16, 16];
//         let is_initialized = match is_initialized {
//             [0] => false,
//             [1] => true,
//             _ => return Err(ProgramError::InvalidAccountData),
//         };

        Ok(Bin {
            // initializer_pubkey: Pubkey::new_from_array(*initializer_pubkey),
            price: u64::from_le_bytes(*price_bytes),
            total: u128::from_le_bytes(*total_bytes),
            amount: u128::from_le_bytes(*amount_bytes),
        })
    }

    fn pack_into_slice(&self, dst: &mut [u8]) {
//         let dst = array_mut_ref![dst, 0, Escrow::LEN];
//         let (
//             is_initialized_dst,
//             initializer_pubkey_dst,
//             temp_token_account_pubkey_dst,
//             initializer_token_to_receive_account_pubkey_dst,
//             expected_amount_dst,
//         ) = mut_array_refs![dst, 1, 32, 32, 32, 8];

//         let Escrow {
//             is_initialized,
//             initializer_pubkey,
//             temp_token_account_pubkey,
//             initializer_token_to_receive_account_pubkey,
//             expected_amount,
//         } = self;

//         is_initialized_dst[0] = *is_initialized as u8;
//         initializer_pubkey_dst.copy_from_slice(initializer_pubkey.as_ref());
//         temp_token_account_pubkey_dst.copy_from_slice(temp_token_account_pubkey.as_ref());
//         initializer_token_to_receive_account_pubkey_dst
//             .copy_from_slice(initializer_token_to_receive_account_pubkey.as_ref());
//         *expected_amount_dst = expected_amount.to_le_bytes();
    }
}

// impl IsInitialized for Escrow {
//     fn is_initialized(&self) -> bool {
//         self.is_initialized
//     }
// }

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
    Setup,
    Buy {
        amount: u64
    },
    Sell {
        amount: u64
    },
}

impl CustomInstruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (tag, rest) = input.split_first().ok_or(error::InstructionError::InvalidInstruction)?;

        Ok(match tag {
            0 => Self::Setup,
            1 => Self::Buy {
                amount: Self::unpack_amount(rest)?,
            },
            2 => Self::Sell {
                amount: Self::unpack_amount(rest)?,
            },
            _ => return Err(error::InstructionError::InvalidInstruction.into()),
        })
    }

    fn unpack_amount(input: &[u8]) -> Result<u64, ProgramError> {
        let amount = input
            .get(..8)
            .and_then(|slice| slice.try_into().ok())
            .map(u64::from_le_bytes)
            .ok_or(error::InstructionError::InvalidInstruction)?;
        Ok(amount)
    }
}


