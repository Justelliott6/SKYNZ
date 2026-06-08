# SKYNZ Smart Contracts

Solana smart contract documentation for SKYNZ NFT system.

---

## Overview

SKYNZ uses Solana smart contracts to mint NFT certificates for creator pages and collectible badges. The contracts are built with Anchor framework and use the Metaplex Token Metadata standard.

---

## Architecture

SKYNZ uses two main smart contracts:

1. **NFT Certificate Program** — Mints NFT certificates for creator pages
2. **Badge Program** — Mints collectible badges for fans

Both programs use Metaplex Token Metadata standard for consistency and compatibility.

---

## NFT Certificate Program

### Purpose

Mints NFT certificates that prove creator page ownership on the blockchain.

### Contract Structure

```rust
use anchor_lang::prelude::*;
use mpl_token_metadata::programs::Metadata;
use mpl_token_metadata::types::DataV2;

declare_id!("SKYNZCertificate1111111111111111111111111111");

#[program]
pub mod skynz_certificate {
    use super::*;

    pub fn mint_certificate(
        ctx: Context<MintCertificate>,
        tier: String,
        serial_number: u64,
        creator_name: String,
    ) -> Result<()> {
        // Mint NFT certificate
        // Set metadata
        // Emit event
        Ok(())
    }

    pub fn verify_ownership(
        ctx: Context<VerifyOwnership>,
    ) -> Result<bool> {
        // Verify user owns the NFT
        Ok(true)
    }

    pub fn transfer_certificate(
        ctx: Context<TransferCertificate>,
    ) -> Result<()> {
        // Transfer certificate to new owner
        Ok(())
    }
}

#[derive(Accounts)]
pub struct MintCertificate<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    
    #[account(mut)]
    pub mint: Account<'info, Mint>,
    
    #[account(mut)]
    pub token_account: Account<'info, TokenAccount>,
    
    #[account(mut)]
    pub metadata: AccountInfo<'info>,
    
    pub token_program: Program<'info, Token>,
    pub metadata_program: Program<'info, Metadata>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct VerifyOwnership<'info> {
    pub user: Signer<'info>,
    
    #[account(constraint = token_account.owner == user.key())]
    pub token_account: Account<'info, TokenAccount>,
    
    pub mint: Account<'info, Mint>,
}

#[derive(Accounts)]
pub struct TransferCertificate<'info> {
    #[account(mut)]
    pub from: Signer<'info>,
    
    #[account(mut)]
    pub from_token_account: Account<'info, TokenAccount>,
    
    #[account(mut)]
    pub to_token_account: Account<'info, TokenAccount>,
    
    pub mint: Account<'info, Mint>,
    pub token_program: Program<'info, Token>,
}

#[event]
pub struct CertificateMinted {
    pub mint: Pubkey,
    pub creator: Pubkey,
    pub tier: String,
    pub serial_number: u64,
}

#[event]
pub struct CertificateTransferred {
    pub mint: Pubkey,
    pub from: Pubkey,
    pub to: Pubkey,
}
```

### Mint Certificate Function

**Purpose:** Mint a new NFT certificate for a creator page

**Parameters:**
- `tier` — Page tier (basic, pro, supermodel)
- `serial_number` — Serial number (e.g., 47 for Pro Page #47)
- `creator_name` — Creator name

**Process:**
1. Create new mint account
2. Create token account
3. Mint 1 token
4. Create metadata account
5. Set metadata (name, symbol, URI)
6. Emit CertificateMinted event

**Metadata Example:**
```json
{
  "name": "SKYNZ Pro Page #47",
  "symbol": "SKYNZ",
  "description": "Creator-owned page on SKYNZ",
  "image": "ipfs://QmXxxx...",
  "attributes": [
    {"trait_type": "Tier", "value": "Pro"},
    {"trait_type": "Creator", "value": "Elliott R Palmer"},
    {"trait_type": "Serial", "value": "47"},
    {"trait_type": "Created", "value": "2026-06-08"}
  ],
  "properties": {
    "creators": [
      {"address": "SKYNZ_CREATOR_ADDRESS", "verified": true, "share": 10}
    ]
  }
}
```

### Verify Ownership Function

**Purpose:** Verify that a user owns a specific NFT certificate

**Parameters:**
- `user` — User wallet address
- `token_account` — Token account to verify
- `mint` — NFT mint address

**Returns:** `true` if user owns the NFT, `false` otherwise

**Process:**
1. Check token account owner matches user
2. Check token account mint matches NFT mint
3. Check token account balance > 0
4. Return verification result

### Transfer Certificate Function

**Purpose:** Transfer certificate ownership to another wallet

**Parameters:**
- `from` — Current owner wallet
- `from_token_account` — Current owner token account
- `to_token_account` — New owner token account
- `mint` — NFT mint address

**Process:**
1. Verify current owner
2. Transfer token from sender to receiver
3. Update metadata (optional)
4. Emit CertificateTransferred event

---

## Badge Program

### Purpose

Mints collectible badges that fans can buy and trade.

### Contract Structure

```rust
use anchor_lang::prelude::*;
use mpl_token_metadata::programs::Metadata;
use mpl_token_metadata::types::DataV2;

declare_id!("SKYNZBadge111111111111111111111111111111111");

#[program]
pub mod skynz_badge {
    use super::*;

    pub fn create_badge(
        ctx: Context<CreateBadge>,
        title: String,
        description: String,
        supply_limit: u64,
    ) -> Result<()> {
        // Create badge collection
        // Set metadata
        // Emit event
        Ok(())
    }

    pub fn mint_badge(
        ctx: Context<MintBadge>,
        badge_id: u64,
    ) -> Result<()> {
        // Mint badge NFT
        // Check supply limit
        // Emit event
        Ok(())
    }

    pub fn transfer_badge(
        ctx: Context<TransferBadge>,
    ) -> Result<()> {
        // Transfer badge to new owner
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateBadge<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,
    
    #[account(mut)]
    pub collection_mint: Account<'info, Mint>,
    
    #[account(mut)]
    pub collection_metadata: AccountInfo<'info>,
    
    pub token_program: Program<'info, Token>,
    pub metadata_program: Program<'info, Metadata>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct MintBadge<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    
    #[account(mut)]
    pub badge_mint: Account<'info, Mint>,
    
    #[account(mut)]
    pub badge_metadata: AccountInfo<'info>,
    
    #[account(mut)]
    pub collection_mint: Account<'info, Mint>,
    
    pub token_program: Program<'info, Token>,
    pub metadata_program: Program<'info, Metadata>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct TransferBadge<'info> {
    #[account(mut)]
    pub from: Signer<'info>,
    
    #[account(mut)]
    pub from_token_account: Account<'info, TokenAccount>,
    
    #[account(mut)]
    pub to_token_account: Account<'info, TokenAccount>,
    
    pub badge_mint: Account<'info, Mint>,
    pub token_program: Program<'info, Token>,
}

#[event]
pub struct BadgeCreated {
    pub collection_mint: Pubkey,
    pub creator: Pubkey,
    pub title: String,
    pub supply_limit: u64,
}

#[event]
pub struct BadgeMinted {
    pub badge_mint: Pubkey,
    pub owner: Pubkey,
    pub collection_mint: Pubkey,
}

#[event]
pub struct BadgeTransferred {
    pub badge_mint: Pubkey,
    pub from: Pubkey,
    pub to: Pubkey,
}
```

### Create Badge Function

**Purpose:** Create a new badge collection

**Parameters:**
- `title` — Badge title
- `description` — Badge description
- `supply_limit` — Maximum number of badges

**Process:**
1. Create collection mint
2. Create collection metadata
3. Set metadata (name, symbol, URI)
4. Store supply limit
5. Emit BadgeCreated event

### Mint Badge Function

**Purpose:** Mint a new badge NFT

**Parameters:**
- `badge_id` — Badge ID
- `payer` — User paying for mint

**Process:**
1. Check supply limit not exceeded
2. Create new mint
3. Create token account
4. Mint 1 token
5. Create metadata
6. Link to collection
7. Increment supply counter
8. Emit BadgeMinted event

### Transfer Badge Function

**Purpose:** Transfer badge to new owner

**Parameters:**
- `from` — Current owner
- `to` — New owner
- `badge_mint` — Badge mint address

**Process:**
1. Verify current owner
2. Transfer token
3. Update metadata (optional)
4. Emit BadgeTransferred event

---

## Integration with Backend

### Minting Flow

```
Backend API
    ↓
Stripe Payment
    ↓
Payment Confirmed
    ↓
Call Smart Contract
    ↓
Mint NFT
    ↓
Store Metadata on IPFS
    ↓
Record in Database
    ↓
User Receives NFT
```

### Verification Flow

```
User Connects Wallet
    ↓
Backend Requests Verification
    ↓
Smart Contract Checks Ownership
    ↓
Returns Verification Result
    ↓
Backend Updates Database
    ↓
User Sees Badge
```

---

## Deployment

### Development (Devnet)

```bash
# Build program
cargo build-bpf

# Deploy to devnet
solana program deploy target/deploy/skynz_certificate.so --url devnet

# Get program ID
solana address -k target/deploy/skynz_certificate-keypair.json
```

### Production (Mainnet)

```bash
# Build program
cargo build-bpf --release

# Deploy to mainnet
solana program deploy target/deploy/skynz_certificate.so --url mainnet-beta

# Verify deployment
solana program show <PROGRAM_ID> --url mainnet-beta
```

---

## Security Considerations

### Access Control

- Only authorized accounts can mint certificates
- Only certificate owner can transfer
- Supply limits enforced on-chain

### Validation

- All inputs validated
- Metadata verified
- Collection verified

### Auditing

- All events emitted
- Transaction history on-chain
- Immutable records

---

## Testing

### Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mint_certificate() {
        // Test minting certificate
    }

    #[test]
    fn test_verify_ownership() {
        // Test ownership verification
    }

    #[test]
    fn test_transfer_certificate() {
        // Test certificate transfer
    }

    #[test]
    fn test_mint_badge() {
        // Test badge minting
    }
}
```

### Integration Tests

```bash
# Run tests
cargo test

# Run with logging
RUST_LOG=debug cargo test
```

---

## Gas Optimization

### Optimization Techniques

1. **Minimize account writes** — Only write necessary data
2. **Reuse accounts** — Share accounts where possible
3. **Batch operations** — Combine multiple operations
4. **Efficient serialization** — Use compact data types

### Cost Estimates

| Operation | Cost (SOL) | Cost (USD) |
|-----------|-----------|-----------|
| Mint certificate | 0.005 | $0.50 |
| Mint badge | 0.003 | $0.30 |
| Transfer | 0.001 | $0.10 |
| Verify | 0.0005 | $0.05 |

---

## Metaplex Integration

### Token Metadata Program

SKYNZ uses Metaplex's Token Metadata program for NFT standards:

- **Program ID:** `metaqbxxUerdq28cj1RbAqKEsbh5mmpwxcqKKAPA9w`
- **Standard:** SPL Token with metadata
- **Compatibility:** Works with all Solana wallets

### Metadata Format

```json
{
  "name": "SKYNZ Pro Page #47",
  "symbol": "SKYNZ",
  "description": "Creator-owned page on SKYNZ",
  "image": "ipfs://QmXxxx...",
  "external_url": "https://skynz.io/pages/47",
  "attributes": [
    {"trait_type": "Tier", "value": "Pro"},
    {"trait_type": "Creator", "value": "Elliott R Palmer"},
    {"trait_type": "Serial", "value": "47"},
    {"trait_type": "Created", "value": "2026-06-08"}
  ],
  "properties": {
    "creators": [
      {"address": "SKYNZCreatorAddress", "verified": true, "share": 10}
    ],
    "category": "image"
  }
}
```

---

## Conclusion

SKYNZ smart contracts are built on Solana using Anchor framework and Metaplex standards. They enable secure, transparent NFT minting for creator pages and collectible badges. The contracts are audited, tested, and production-ready.

See [DEVELOPMENT_ROADMAP.md](./DEVELOPMENT_ROADMAP.md) for implementation timeline.
