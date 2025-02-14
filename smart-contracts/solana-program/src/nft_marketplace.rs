use solana_program::{
    account_info::{AccountInfo, next_account_info},
    entrypoint::ProgramResult,
    program::invoke_signed,
    pubkey::Pubkey,
    system_instruction,
};

pub struct SpaceNFT;

impl SpaceNFT {
    pub fn mint_encrypted_coordinate(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        coordinate_data: &[u8],
        bump_seed: u8
    ) -> ProgramResult {
        let accounts_iter = &mut accounts.iter();
        let payer = next_account_info(accounts_iter)?;
        let nft_account = next_account_info(accounts_iter)?;
        let system_program = next_account_info(accounts_iter)?;

        let space_seeds = &[
            b"space_coord".as_ref(),
            coordinate_data,
            &[bump_seed]
        ];
        
        let ix = system_instruction::create_account(
            payer.key,
            nft_account.key,
            1_000_000_000, // 1 SOL
            4200,          // Account space
            program_id
        );

        invoke_signed(
            &ix,
            &[payer.clone(), nft_account.clone(), system_program.clone()],
            &[space_seeds]
        )?;

        // Store encrypted coordinate data
        let mut data = nft_account.try_borrow_mut_data()?;
        data[0..coordinate_data.len()].copy_from_slice(coordinate_data);
        
        Ok(())
    }

    pub fn verify_ownership(
        &self,
        account: &AccountInfo,
        expected_owner: &Pubkey
    ) -> ProgramResult {
        if account.owner != expected_owner {
            return Err(ProgramError::InvalidAccountOwner);
        }
        Ok(())
    }
}
