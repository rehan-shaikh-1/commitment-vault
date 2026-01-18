use anchor_lang::prelude::*;
use anchor_lang::system_program;

// This is your program ID (Anchor will generate a new one when you build, update this later)
declare_id!("FP3Dz4vWctJg1kvx3DkuRDT1ToGBoRRegdrjxmEeW7GE");

#[program]
pub mod commitment_vault {
    use super::*;

    // 1. Initialize the Vault and Lock Funds
    // Arguments:
    // - lock_duration: How long (in seconds) to lock the funds.
    // - amount: How many lamports (SOL) to lock.
    pub fn initialize_vault(ctx: Context<Initialize>, lock_duration: i64, amount: u64) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        let owner = &ctx.accounts.owner;
        
        // Get the current on-chain time
        let clock = Clock::get()?;

        // SET STATE
        vault.owner = owner.key();
        vault.unlock_time = clock.unix_timestamp + lock_duration;
        vault.bump = ctx.bumps.vault;

        // TRANSFER FUNDS: User -> Vault PDA
        // We use the system_program to move native SOL
        let cpi_context = CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            system_program::Transfer {
                from: owner.to_account_info(),
                to: vault.to_account_info(),
            },
        );
        system_program::transfer(cpi_context, amount)?;

        msg!("Vault Initialized! Locked until timestamp: {}", vault.unlock_time);
        Ok(())
    }

    // 2. Withdraw Funds
    // This allows the user to close the vault and retrieve funds, 
    // BUT only if the time has passed.
    pub fn withdraw(ctx: Context<Withdraw>) -> Result<()> {
        let vault = &ctx.accounts.vault;
        let clock = Clock::get()?;

        // SECURITY CHECK: Time Lock
        if clock.unix_timestamp < vault.unlock_time {
            return err!(VaultError::VaultLocked);
        }

        // SECURITY CHECK: Access Control
        // (Note: Anchor verifies the signer matches the `owner` account constraint 
        // in the struct below, but explicit checks are good for clarity).
        require_keys_eq!(vault.owner, ctx.accounts.owner.key(), VaultError::Unauthorized);

        // TRANSFER LOGIC is handled by `close = owner` in the context!
        // This automatically moves all lamports from the vault back to the owner
        // and deletes the account to clean up chain state.
        
        msg!("Time lock passed. Withdrawal successful!");
        Ok(())
    }
}

// --- DATA STRUCTURES (STATE) ---

#[account]
pub struct Vault {
    pub owner: Pubkey,      // 32 bytes
    pub unlock_time: i64,   // 8 bytes
    pub bump: u8,           // 1 byte
}

// --- VALIDATION STRUCTS ---

#[derive(Accounts)]
pub struct Initialize<'info> {
    // We create a PDA (Program Derived Address) to act as the vault
    #[account(
        init, 
        payer = owner, 
        space = 8 + 32 + 8 + 1, // Discriminator + Owner + Time + Bump
        seeds = [b"vault", owner.key().as_ref()], 
        bump
    )]
    pub vault: Account<'info, Vault>,

    #[account(mut)]
    pub owner: Signer<'info>, // The user sending money

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(
        mut,
        seeds = [b"vault", owner.key().as_ref()], // Validate this is the correct vault
        bump = vault.bump,
        close = owner // CRITICAL: This sends all funds to owner and closes account
    )]
    pub vault: Account<'info, Vault>,

    #[account(mut)]
    pub owner: Signer<'info>, // Must be signed by the owner

    pub system_program: Program<'info, System>,
}

// --- ERROR HANDLING ---

#[error_code]
pub enum VaultError {
    #[msg("The vault is still locked. Patience is a virtue.")]
    VaultLocked,
    #[msg("You are not the owner of this vault.")]
    Unauthorized,
}