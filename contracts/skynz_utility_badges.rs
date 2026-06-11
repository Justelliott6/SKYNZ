// SKYNZ Utility Badge Minting Program
// Solana Program for minting NFT badges with built-in utilities
// Uses Anchor framework and Metaplex standards

use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Mint};
use anchor_spl::associated_token::AssociatedToken;
use mpl_token_metadata::instruction::{create_metadata_accounts_v3, update_metadata_accounts_v2};

declare_id!("SKYNZBadgeMintingProgram123456789");

#[program]
pub mod skynz_utility_badges {
    use super::*;

    /// Mint a new SKYNZ utility badge with embedded utilities
    pub fn mint_utility_badge(
        ctx: Context<MintUtilityBadge>,
        badge_type: String,
        serial_number: String,
        metadata_uri: String,
        utilities: Vec<BadgeUtility>,
    ) -> Result<()> {
        // Validate inputs
        require!(badge_type.len() > 0, ErrorCode::InvalidBadgeType);
        require!(serial_number.len() > 0, ErrorCode::InvalidSerialNumber);
        require!(utilities.len() > 0, ErrorCode::NoUtilitiesProvided);

        // Create mint
        let mint = &ctx.accounts.mint;
        let payer = &ctx.accounts.payer;

        // Initialize mint
        token::initialize_mint(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                token::InitializeMint {
                    mint: mint.to_account_info(),
                    rent: ctx.accounts.rent.to_account_info(),
                },
            ),
            0, // decimals
            &payer.key(),
            Some(&payer.key()),
        )?;

        // Create associated token account
        let ata = &ctx.accounts.associated_token_account;
        token::initialize_account(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                token::InitializeAccount {
                    account: ata.to_account_info(),
                    mint: mint.to_account_info(),
                    owner: payer.to_account_info(),
                    rent: ctx.accounts.rent.to_account_info(),
                },
            ),
        )?;

        // Mint 1 token to holder
        token::mint_to(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                token::MintTo {
                    mint: mint.to_account_info(),
                    to: ata.to_account_info(),
                    authority: payer.to_account_info(),
                },
            ),
            1,
        )?;

        // Create metadata account
        let metadata_account = &ctx.accounts.metadata;
        create_metadata_accounts_v3(
            CpiContext::new(
                ctx.accounts.token_metadata_program.to_account_info(),
                mpl_token_metadata::accounts::CreateMetadataAccountsV3 {
                    metadata: metadata_account.to_account_info(),
                    mint: mint.to_account_info(),
                    mint_authority: payer.to_account_info(),
                    payer: payer.to_account_info(),
                    update_authority: payer.to_account_info(),
                    system_program: ctx.accounts.system_program.to_account_info(),
                    rent: ctx.accounts.rent.to_account_info(),
                },
            ),
            mpl_token_metadata::types::DataV2 {
                name: format!("SKYNZ {} Badge #{}", badge_type, serial_number),
                symbol: "SKYNZ".to_string(),
                uri: metadata_uri.clone(),
                seller_fee_basis_points: 1000, // 10% royalty
                creators: Some(vec![
                    mpl_token_metadata::types::Creator {
                        address: payer.key(),
                        verified: true,
                        share: 100,
                    },
                ]),
                collection: None,
                uses: None,
            },
            true,
            true,
            None,
        )?;

        // Store badge data in PDA
        let badge_data = &mut ctx.accounts.badge_data;
        badge_data.mint = mint.key();
        badge_data.badge_type = badge_type.clone();
        badge_data.serial_number = serial_number.clone();
        badge_data.holder = payer.key();
        badge_data.metadata_uri = metadata_uri;
        badge_data.utilities = utilities.clone();
        badge_data.created_at = Clock::get()?.unix_timestamp;
        badge_data.redeemed = false;

        // Emit event
        emit!(BadgeMinted {
            mint: mint.key(),
            holder: payer.key(),
            badge_type: badge_type.clone(),
            serial_number: serial_number.clone(),
            utilities_count: utilities.len() as u32,
            timestamp: Clock::get()?.unix_timestamp,
        });

        Ok(())
    }

    /// Redeem a utility from a badge
    pub fn redeem_utility(
        ctx: Context<RedeemUtility>,
        utility_name: String,
    ) -> Result<()> {
        let badge_data = &mut ctx.accounts.badge_data;

        // Verify holder
        require_eq!(
            badge_data.holder,
            ctx.accounts.holder.key(),
            ErrorCode::NotBadgeHolder
        );

        // Find utility
        let utility = badge_data
            .utilities
            .iter_mut()
            .find(|u| u.name == utility_name)
            .ok_or(ErrorCode::UtilityNotFound)?;

        // Verify not already redeemed (if single-use)
        require!(!utility.redeemed, ErrorCode::UtilityAlreadyRedeemed);

        // Mark as redeemed
        utility.redeemed = true;
        utility.redeemed_at = Some(Clock::get()?.unix_timestamp);

        // Emit event
        emit!(UtilityRedeemed {
            mint: badge_data.mint,
            holder: ctx.accounts.holder.key(),
            utility_name: utility_name.clone(),
            timestamp: Clock::get()?.unix_timestamp,
        });

        Ok(())
    }

    /// Transfer badge to new owner
    pub fn transfer_badge(
        ctx: Context<TransferBadge>,
        amount: u64,
    ) -> Result<()> {
        // Transfer NFT
        token::transfer(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                token::Transfer {
                    from: ctx.accounts.from_token_account.to_account_info(),
                    to: ctx.accounts.to_token_account.to_account_info(),
                    authority: ctx.accounts.owner.to_account_info(),
                },
            ),
            amount,
        )?;

        // Update badge data holder
        let badge_data = &mut ctx.accounts.badge_data;
        badge_data.holder = ctx.accounts.to.key();

        // Emit event
        emit!(BadgeTransferred {
            mint: badge_data.mint,
            from: ctx.accounts.owner.key(),
            to: ctx.accounts.to.key(),
            timestamp: Clock::get()?.unix_timestamp,
        });

        Ok(())
    }

    /// Verify badge holder has specific utility
    pub fn verify_utility_holder(
        ctx: Context<VerifyUtilityHolder>,
        utility_name: String,
    ) -> Result<bool> {
        let badge_data = &ctx.accounts.badge_data;

        // Verify holder
        require_eq!(
            badge_data.holder,
            ctx.accounts.holder.key(),
            ErrorCode::NotBadgeHolder
        );

        // Check if utility exists
        let has_utility = badge_data
            .utilities
            .iter()
            .any(|u| u.name == utility_name && !u.redeemed);

        Ok(has_utility)
    }
}

// ============================================================================
// ACCOUNTS
// ============================================================================

#[derive(Accounts)]
pub struct MintUtilityBadge<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        init,
        payer = payer,
        mint::decimals = 0,
        mint::authority = payer,
        mint::freeze_authority = payer
    )]
    pub mint: Account<'info, Mint>,

    #[account(
        init,
        payer = payer,
        associated_token::mint = mint,
        associated_token::authority = payer
    )]
    pub associated_token_account: Account<'info, TokenAccount>,

    /// CHECK: Metaplex metadata account
    #[account(mut)]
    pub metadata: UncheckedAccount<'info>,

    #[account(
        init,
        payer = payer,
        space = 8 + BadgeData::INIT_SPACE,
        seeds = [b"badge", mint.key().as_ref()],
        bump
    )]
    pub badge_data: Account<'info, BadgeData>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,

    /// CHECK: Metaplex token metadata program
    pub token_metadata_program: UncheckedAccount<'info>,
}

#[derive(Accounts)]
pub struct RedeemUtility<'info> {
    #[account(mut)]
    pub holder: Signer<'info>,

    #[account(mut)]
    pub badge_data: Account<'info, BadgeData>,
}

#[derive(Accounts)]
pub struct TransferBadge<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    /// CHECK: New owner
    pub to: UncheckedAccount<'info>,

    #[account(mut)]
    pub from_token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub to_token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub badge_data: Account<'info, BadgeData>,

    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct VerifyUtilityHolder<'info> {
    pub holder: Signer<'info>,
    pub badge_data: Account<'info, BadgeData>,
}

// ============================================================================
// DATA STRUCTURES
// ============================================================================

#[account]
pub struct BadgeData {
    pub mint: Pubkey,
    pub badge_type: String,
    pub serial_number: String,
    pub holder: Pubkey,
    pub metadata_uri: String,
    pub utilities: Vec<BadgeUtility>,
    pub created_at: i64,
    pub redeemed: bool,
}

impl BadgeData {
    const INIT_SPACE: usize = 8 + 32 + 50 + 50 + 32 + 200 + 1000 + 8 + 1;
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize, Debug)]
pub struct BadgeUtility {
    pub name: String,
    pub description: String,
    pub tier: String, // "premium", "exclusive", "enterprise"
    pub monthly_value: String,
    pub redeemed: bool,
    pub redeemed_at: Option<i64>,
    pub single_use: bool,
}

// ============================================================================
// EVENTS
// ============================================================================

#[event]
pub struct BadgeMinted {
    pub mint: Pubkey,
    pub holder: Pubkey,
    pub badge_type: String,
    pub serial_number: String,
    pub utilities_count: u32,
    pub timestamp: i64,
}

#[event]
pub struct UtilityRedeemed {
    pub mint: Pubkey,
    pub holder: Pubkey,
    pub utility_name: String,
    pub timestamp: i64,
}

#[event]
pub struct BadgeTransferred {
    pub mint: Pubkey,
    pub from: Pubkey,
    pub to: Pubkey,
    pub timestamp: i64,
}

// ============================================================================
// ERRORS
// ============================================================================

#[error_code]
pub enum ErrorCode {
    #[msg("Invalid badge type")]
    InvalidBadgeType,

    #[msg("Invalid serial number")]
    InvalidSerialNumber,

    #[msg("No utilities provided")]
    NoUtilitiesProvided,

    #[msg("Not badge holder")]
    NotBadgeHolder,

    #[msg("Utility not found")]
    UtilityNotFound,

    #[msg("Utility already redeemed")]
    UtilityAlreadyRedeemed,

    #[msg("Invalid metadata")]
    InvalidMetadata,

    #[msg("Unauthorized")]
    Unauthorized,
}
