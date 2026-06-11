# SKYNZ Badge Minting Guide — With Built-In Utilities

**Complete guide to minting SKYNZ badges with embedded utilities and redemption system**

---

## 📋 Table of Contents

1. [Overview](#overview)
2. [Badge Types & Utilities](#badge-types--utilities)
3. [Pre-Minting Setup](#pre-minting-setup)
4. [Minting Process](#minting-process)
5. [Utility Redemption](#utility-redemption)
6. [Marketplace Listing](#marketplace-listing)
7. [Troubleshooting](#troubleshooting)

---

## Overview

SKYNZ badges are not just collectibles—they're **utility-bearing NFTs** that grant exclusive access to premium features on the SKYNZ Creator Command Page platform.

### What Makes SKYNZ Badges Different?

✅ **Embedded Utilities** — Each badge includes 5-10 premium features worth $100-400/month  
✅ **Redeemable Benefits** — Holders can activate utilities directly from their wallet  
✅ **Passive Revenue** — 2% platform revenue share for top-tier badges  
✅ **Forever Royalties** — 10% on secondary market sales (enforced at contract level)  
✅ **Transferable** — Sell or gift your badge on Magic Eden or Tensor  
✅ **Limited Edition** — Only 500 badges total across 5 designs  

---

## Badge Types & Utilities

### Badge 1: Neon Nomad (100 available)
**Rarity:** Legendary  
**Price:** 0.5 SOL (~$50)  
**Monthly Value:** $422

**Included Utilities:**
1. **Unlimited Theme Customization** ($49/mo) — All 5 themes + unlimited colors
2. **24/7 Priority Support** ($29/mo) — 1-hour response guarantee
3. **2% Platform Revenue Share** (Variable) — Passive income forever
4. **Exclusive Marketplace Listing** ($19/mo) — Featured on Magic Eden/Tensor
5. **Advanced Real-Time Analytics** ($39/mo) — Detailed visitor insights
6. **Unlimited Live Streaming** ($59/mo) — No duration limits
7. **Full Developer API Access** ($99/mo) — Custom integrations
8. **Custom Domain Support** ($29/mo) — Use your own domain
9. **White Label Option** ($199/mo) — Remove SKYNZ branding
10. **Early Access to Features** (Exclusive) — Beta test new features

---

### Badge 2: Cipher Sage (100 available)
**Rarity:** Legendary  
**Price:** 0.5 SOL (~$50)  
**Monthly Value:** $115

**Included Utilities:**
1. **Premium Theme Customization** ($29/mo) — 3 themes + colors
2. **Priority Email Support** ($19/mo) — 4-hour response
3. **1% Platform Revenue Share** (Variable) — Passive income
4. **Advanced Analytics** ($19/mo) — Visitor insights
5. **Extended Live Streaming** ($29/mo) — Up to 24 hours

---

### Badge 3: Pulse Guardian (100 available)
**Rarity:** Legendary  
**Price:** 0.5 SOL (~$50)  
**Monthly Value:** $56

**Included Utilities:**
1. **Standard Theme Customization** ($19/mo) — 2 themes
2. **Standard Email Support** ($9/mo) — 24-hour response
3. **Basic Analytics** ($9/mo) — Visitor metrics
4. **Extended Live Streaming** ($19/mo) — Up to 12 hours

---

### Badge 4: Void Echo (100 available)
**Rarity:** Legendary  
**Price:** 0.5 SOL (~$50)  
**Monthly Value:** $9

**Included Utilities:**
1. **Theme Customization** ($9/mo) — 1 theme
2. **Email Support** (Free) — Standard support
3. **Basic Analytics** (Free) — Basic metrics

---

### Badge 5: Stellar Drift (100 available)
**Rarity:** Legendary  
**Price:** 0.5 SOL (~$50)  
**Monthly Value:** Priceless

**Included Utilities:**
1. **Founding Creator Status** (Exclusive) — Official badge
2. **Limited Edition Serial Number** (Collectible) — Proof of early adoption
3. **Lifetime Badge Holder Benefits** (Forever) — Permanent perks

---

## Pre-Minting Setup

### Step 1: Install Required Tools

```bash
# Install Metaplex Sugar CLI
npm install -g @metaplex-foundation/sugar

# Install Solana CLI
sh -c "$(curl -sSfL https://release.solana.com/v1.18.0/install)"

# Install Rust (required for Anchor)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Anchor
cargo install --git https://github.com/coral-xyz/anchor avm --locked --force
avm install latest
avm use latest
```

### Step 2: Set Up Solana Wallet

```bash
# Create Solana keypair (if not already done)
solana-keygen new

# Set RPC endpoint to devnet
solana config set --url https://api.devnet.solana.com

# Request SOL airdrop
solana airdrop 2

# Verify balance
solana balance
```

### Step 3: Prepare NFT Assets

```bash
# Create directory structure
mkdir -p assets/badges
cd assets/badges

# Add badge images (PNG format, 1200x1200px recommended)
# - neon-nomad.png
# - cipher-sage.png
# - pulse-guardian.png
# - void-echo.png
# - stellar-drift.png

# Add animation GIFs (optional, 1200x1200px)
# - neon-nomad-animation.gif
# - cipher-sage-animation.gif
# etc.
```

### Step 4: Create Metadata Files

Each badge needs a metadata JSON file (already created in `/nft-badges/`):

```bash
# Copy metadata files to assets directory
cp /home/ubuntu/SKYNZ/nft-badges/*.json ./metadata/
```

---

## Minting Process

### Step 1: Deploy Solana Program

```bash
# Navigate to contracts directory
cd /home/ubuntu/SKYNZ/contracts

# Build the Anchor program
anchor build

# Deploy to devnet
anchor deploy --provider.cluster devnet

# Note the program ID (you'll need this later)
```

### Step 2: Configure Candy Machine

```bash
# Copy config file
cp candy-machine-config.json ~/sugar-config.json

# Edit config with your details
nano ~/sugar-config.json

# Key fields to update:
# - creators[0].address: Your wallet address
# - awsS3Bucket: Your S3 bucket name
# - nftStorageKey: Your NFT.storage API key
# - bundlrAddress: Bundlr node address
```

### Step 3: Upload Assets to Arweave

```bash
# Navigate to assets directory
cd ~/assets/badges

# Upload using Bundlr (cheaper than direct Arweave)
sugar upload \
  --config ~/sugar-config.json \
  --candy-machine-program CndyV3LVqNd77ZfVj3XnYr67v7D1rKJMSnCvkMd5t42m

# This will:
# 1. Upload all images to Arweave
# 2. Upload all metadata JSON files
# 3. Generate a manifest file
# 4. Output the Candy Machine ID
```

### Step 4: Deploy Candy Machine

```bash
# Deploy Candy Machine
sugar deploy \
  --config ~/sugar-config.json \
  --candy-machine-program CndyV3LVqNd77ZfVj3XnYr67v7D1rKJMSnCvkMd5t42m

# Verify deployment
sugar show --config ~/sugar-config.json
```

### Step 5: Go Live

```bash
# Set go-live date (optional, for scheduled launch)
sugar update \
  --config ~/sugar-config.json \
  --go-live-date "2026-06-15T00:00:00Z"

# Start minting (if no go-live date set)
sugar mint \
  --config ~/sugar-config.json \
  --number 1

# Check mint status
sugar show --config ~/sugar-config.json
```

---

## Utility Redemption

### How Utilities Work

1. **Badge Holder** purchases badge on Magic Eden or Tensor
2. **Holder connects wallet** to SKYNZ platform
3. **System verifies** badge ownership via smart contract
4. **Utilities automatically unlock** in creator dashboard
5. **Holder can redeem** individual utilities as needed

### Redemption Flow

```typescript
// Example: Verify badge holder has utility
const verifyUtility = async (walletAddress, utilityName) => {
  // Check if wallet owns SKYNZ badge
  const badges = await connection.getParsedTokenAccountsByOwner(
    new PublicKey(walletAddress),
    { programId: TOKEN_PROGRAM_ID }
  );

  // Find SKYNZ badge
  const skynzBadge = badges.value.find(
    token => token.account.data.parsed.info.mint === SKYNZ_BADGE_MINT
  );

  if (!skynzBadge) {
    throw new Error("No SKYNZ badge found");
  }

  // Call smart contract to verify utility
  const hasUtility = await program.methods
    .verifyUtilityHolder(utilityName)
    .accounts({
      holder: new PublicKey(walletAddress),
      badgeData: badgeDataPDA,
    })
    .view();

  return hasUtility;
};
```

### Redeeming a Utility

```typescript
// Example: Redeem "advanced_analytics" utility
const redeemUtility = async (walletAddress, utilityName) => {
  const tx = await program.methods
    .redeemUtility(utilityName)
    .accounts({
      holder: new PublicKey(walletAddress),
      badgeData: badgeDataPDA,
    })
    .signers([wallet])
    .rpc();

  console.log("Utility redeemed:", tx);
  return tx;
};
```

### Utility Activation in Dashboard

Once redeemed, utilities activate automatically:

| Utility | Activation | Benefit |
|---------|-----------|---------|
| Theme Customization | Immediate | All theme options unlock |
| Priority Support | Immediate | Support email tag added |
| Revenue Share | Monthly | Automatic payment on 1st of month |
| Marketplace Listing | Immediate | Featured placement |
| Analytics | Immediate | Advanced dashboard unlocks |
| Live Streaming | Immediate | Duration limits removed |
| API Access | Immediate | API keys generated |
| Custom Domain | 24 hours | DNS updated |
| White Label | 24 hours | Branding removed |
| Early Access | Ongoing | Beta features available |

---

## Marketplace Listing

### List on Magic Eden

```bash
# Get your badge mint address from Candy Machine
BADGE_MINT="your_badge_mint_address"

# List on Magic Eden using CLI
curl -X POST https://api-mainnet.magiceden.dev/v2/launchpad/collections \
  -H "Authorization: Bearer YOUR_MAGIC_EDEN_API_KEY" \
  -H "Content-Type: application/json" \
  -d '{
    "symbol": "SKYNZ-NN",
    "name": "Neon Nomad - SKYNZ Badge",
    "description": "Legendary collectible badge granting exclusive SKYNZ features",
    "image": "https://arweave.net/neon-nomad-badge-image",
    "creators": ["YOUR_WALLET_ADDRESS"],
    "royalty_percentage": 10
  }'
```

### List on Tensor

```bash
# List on Tensor using GraphQL API
curl https://api.tensor.trade/graphql \
  -H "Authorization: Bearer YOUR_TENSOR_API_KEY" \
  -H "Content-Type: application/json" \
  -d '{
    "query": "mutation { createCollection(...) }"
  }'
```

---

## Troubleshooting

### Issue: "Insufficient SOL for transaction"
**Solution:** Request more devnet SOL
```bash
solana airdrop 2
```

### Issue: "Metadata upload failed"
**Solution:** Check Arweave/Bundlr connection
```bash
# Verify Bundlr balance
curl https://node1.bundlr.network/account/balance/solana

# Fund Bundlr account if needed
```

### Issue: "Candy Machine not found"
**Solution:** Verify program ID and config
```bash
# Check deployed programs
solana program show --programs

# Verify config file
cat ~/sugar-config.json | grep candyMachineProgram
```

### Issue: "Badge not showing in wallet"
**Solution:** Wait for blockchain confirmation
```bash
# Check transaction status
solana confirm YOUR_TRANSACTION_SIGNATURE

# Refresh wallet (Phantom → Settings → Reset)
```

### Issue: "Utility redemption failed"
**Solution:** Verify badge ownership
```bash
# Check token account
spl-token accounts YOUR_WALLET_ADDRESS

# Verify badge mint
spl-token supply YOUR_BADGE_MINT
```

---

## Testing Checklist

- [ ] Deployed Solana program successfully
- [ ] Uploaded assets to Arweave
- [ ] Deployed Candy Machine
- [ ] Minted test badge on devnet
- [ ] Badge appears in wallet
- [ ] Metadata displays correctly
- [ ] Verified badge ownership
- [ ] Redeemed test utility
- [ ] Listed on Magic Eden (testnet)
- [ ] Listed on Tensor (testnet)
- [ ] Tested secondary market transfer
- [ ] Verified royalty payment
- [ ] Tested utility redemption flow
- [ ] Confirmed utility activation in dashboard

---

## Production Deployment

### Before Going Live

1. **Audit Smart Contract**
   - Have Anchor program reviewed by security firm
   - Test all edge cases
   - Verify royalty enforcement

2. **Test on Mainnet-Beta**
   - Deploy to mainnet-beta first
   - Mint test badges
   - Verify marketplace listings
   - Test utility redemption

3. **Prepare Marketing**
   - Create landing page
   - Prepare social media content
   - Set up email waitlist
   - Brief first creators

### Mainnet Deployment

```bash
# Switch to mainnet
solana config set --url https://api.mainnet-beta.solana.com

# Deploy program to mainnet
anchor deploy --provider.cluster mainnet

# Deploy Candy Machine to mainnet
sugar deploy \
  --config ~/sugar-config.json \
  --candy-machine-program CndyV3LVqNd77ZfVj3XnYr67v7D1rKJMSnCvkMd5t42m

# Go live
sugar update \
  --config ~/sugar-config.json \
  --go-live-date "2026-06-15T00:00:00Z"
```

---

## Revenue Model

### Per Badge Minting

| Item | Amount |
|------|--------|
| Badge Price | 0.5 SOL (~$50) |
| Platform Fee (5%) | 0.025 SOL (~$2.50) |
| Creator Revenue (95%) | 0.475 SOL (~$47.50) |

### Secondary Market (Magic Eden/Tensor)

| Item | Amount |
|------|--------|
| Sale Price | Variable |
| Creator Royalty (10%) | 10% of sale |
| Marketplace Fee (2%) | 2% of sale |
| Seller Profit | 88% of sale |

### Utility Revenue (Annual)

| Utility | Monthly Value | Annual Value |
|---------|---------------|--------------|
| Theme Customization | $49 | $588 |
| Priority Support | $29 | $348 |
| Revenue Share | Variable | Variable |
| Analytics | $39 | $468 |
| API Access | $99 | $1,188 |
| **Total (Neon Nomad)** | **$422** | **$5,064** |

---

## Key Metrics

| Metric | Target |
|--------|--------|
| Total Badges Minted | 500 |
| Primary Revenue | $25,000 (500 × 0.5 SOL) |
| Platform Revenue (5%) | $1,250 |
| Creator Revenue (95%) | $23,750 |
| Secondary Market Volume (Year 1) | $100,000+ |
| Creator Royalties (Year 1) | $10,000+ |
| Utility Revenue (Year 1) | $50,000+ |

---

## Support & Resources

- **Metaplex Docs:** https://docs.metaplex.com
- **Anchor Docs:** https://www.anchor-lang.com
- **Solana Docs:** https://docs.solana.com
- **Magic Eden API:** https://docs.magiceden.io
- **Tensor API:** https://docs.tensor.trade

---

**Version:** 1.0.0  
**Last Updated:** June 11, 2026  
**Status:** Ready for Devnet Testing

**Still writing code after 8 months.** 🚀
