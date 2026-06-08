# SKYNZ Database Schema

Complete PostgreSQL database schema for SKYNZ.

---

## Overview

SKYNZ uses PostgreSQL with the following core tables:
- Users
- Creator Pages
- Live Chat
- Analytics
- Badges
- Badge Purchases
- Ads
- NFTs
- Payments

---

## Users Table

Stores user account information.

```sql
CREATE TABLE users (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  email VARCHAR(255) UNIQUE NOT NULL,
  password_hash VARCHAR(255) NOT NULL,
  username VARCHAR(100) UNIQUE NOT NULL,
  profile_picture TEXT,
  bio TEXT,
  follower_count INTEGER DEFAULT 0,
  following_count INTEGER DEFAULT 0,
  wallet_address VARCHAR(255),
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_users_email ON users(email);
CREATE INDEX idx_users_username ON users(username);
CREATE INDEX idx_users_wallet_address ON users(wallet_address);
```

**Fields:**
- `id` — Unique user identifier
- `email` — User email (unique)
- `password_hash` — Hashed password
- `username` — Display name (unique)
- `profile_picture` — URL to profile image
- `bio` — User bio
- `follower_count` — Number of followers
- `following_count` — Number following
- `wallet_address` — Solana wallet address
- `created_at` — Account creation timestamp
- `updated_at` — Last update timestamp

---

## Creator Pages Table

Stores creator page information.

```sql
CREATE TABLE creator_pages (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  creator_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  title VARCHAR(255) NOT NULL,
  description TEXT,
  tier VARCHAR(50) NOT NULL CHECK (tier IN ('basic', 'pro', 'supermodel')),
  nft_certificate_id UUID,
  live_chat_enabled BOOLEAN DEFAULT true,
  music_widget_enabled BOOLEAN DEFAULT true,
  analytics_enabled BOOLEAN DEFAULT true,
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_creator_pages_creator_id ON creator_pages(creator_id);
CREATE INDEX idx_creator_pages_tier ON creator_pages(tier);
CREATE INDEX idx_creator_pages_created_at ON creator_pages(created_at);
```

**Fields:**
- `id` — Unique page identifier
- `creator_id` — Reference to creator (user)
- `title` — Page title
- `description` — Page description
- `tier` — Page tier (basic, pro, supermodel)
- `nft_certificate_id` — Reference to NFT certificate
- `live_chat_enabled` — Enable/disable live chat
- `music_widget_enabled` — Enable/disable music widget
- `analytics_enabled` — Enable/disable analytics
- `created_at` — Page creation timestamp
- `updated_at` — Last update timestamp

---

## Live Chat Table

Stores live chat messages.

```sql
CREATE TABLE live_chat (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  page_id UUID NOT NULL REFERENCES creator_pages(id) ON DELETE CASCADE,
  user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  message TEXT NOT NULL,
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_live_chat_page_id ON live_chat(page_id);
CREATE INDEX idx_live_chat_user_id ON live_chat(user_id);
CREATE INDEX idx_live_chat_created_at ON live_chat(created_at);
```

**Fields:**
- `id` — Unique message identifier
- `page_id` — Reference to creator page
- `user_id` — Reference to message sender
- `message` — Message content
- `created_at` — Message timestamp

---

## Analytics Table

Stores daily analytics data.

```sql
CREATE TABLE analytics (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  page_id UUID NOT NULL REFERENCES creator_pages(id) ON DELETE CASCADE,
  date DATE NOT NULL,
  page_views INTEGER DEFAULT 0,
  unique_visitors INTEGER DEFAULT 0,
  watch_time_minutes INTEGER DEFAULT 0,
  chat_messages INTEGER DEFAULT 0,
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_analytics_page_id ON analytics(page_id);
CREATE INDEX idx_analytics_date ON analytics(date);
CREATE UNIQUE INDEX idx_analytics_page_date ON analytics(page_id, date);
```

**Fields:**
- `id` — Unique analytics record identifier
- `page_id` — Reference to creator page
- `date` — Date of analytics
- `page_views` — Number of page views
- `unique_visitors` — Number of unique visitors
- `watch_time_minutes` — Total watch time in minutes
- `chat_messages` — Number of chat messages
- `created_at` — Record creation timestamp

---

## Badges Table

Stores collectible badge information.

```sql
CREATE TABLE badges (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  page_id UUID NOT NULL REFERENCES creator_pages(id) ON DELETE CASCADE,
  title VARCHAR(255) NOT NULL,
  description TEXT,
  price DECIMAL(10, 2) NOT NULL,
  supply_limit INTEGER NOT NULL,
  supply_minted INTEGER DEFAULT 0,
  nft_contract_address VARCHAR(255),
  metadata_uri TEXT,
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_badges_page_id ON badges(page_id);
CREATE INDEX idx_badges_created_at ON badges(created_at);
```

**Fields:**
- `id` — Unique badge identifier
- `page_id` — Reference to creator page
- `title` — Badge title
- `description` — Badge description
- `price` — Badge price in USD
- `supply_limit` — Maximum number of badges
- `supply_minted` — Number of badges minted
- `nft_contract_address` — Solana contract address
- `metadata_uri` — IPFS metadata URI
- `created_at` — Badge creation timestamp
- `updated_at` — Last update timestamp

---

## Badge Purchases Table

Stores badge purchase transactions.

```sql
CREATE TABLE badge_purchases (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  badge_id UUID NOT NULL REFERENCES badges(id) ON DELETE CASCADE,
  buyer_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  transaction_id VARCHAR(255) UNIQUE NOT NULL,
  price_paid DECIMAL(10, 2) NOT NULL,
  nft_token_id VARCHAR(255),
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_badge_purchases_badge_id ON badge_purchases(badge_id);
CREATE INDEX idx_badge_purchases_buyer_id ON badge_purchases(buyer_id);
CREATE INDEX idx_badge_purchases_transaction_id ON badge_purchases(transaction_id);
CREATE INDEX idx_badge_purchases_created_at ON badge_purchases(created_at);
```

**Fields:**
- `id` — Unique purchase identifier
- `badge_id` — Reference to badge
- `buyer_id` — Reference to buyer (user)
- `transaction_id` — Stripe transaction ID
- `price_paid` — Price paid in USD
- `nft_token_id` — Solana NFT token ID
- `created_at` — Purchase timestamp

---

## Ads Table

Stores ad listings on creator pages.

```sql
CREATE TABLE ads (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  page_id UUID NOT NULL REFERENCES creator_pages(id) ON DELETE CASCADE,
  advertiser_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  title VARCHAR(255) NOT NULL,
  description TEXT,
  price DECIMAL(10, 2) NOT NULL,
  start_date DATE NOT NULL,
  end_date DATE NOT NULL,
  status VARCHAR(50) DEFAULT 'active' CHECK (status IN ('active', 'paused', 'expired')),
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_ads_page_id ON ads(page_id);
CREATE INDEX idx_ads_advertiser_id ON ads(advertiser_id);
CREATE INDEX idx_ads_status ON ads(status);
CREATE INDEX idx_ads_created_at ON ads(created_at);
```

**Fields:**
- `id` — Unique ad identifier
- `page_id` — Reference to creator page
- `advertiser_id` — Reference to advertiser (user)
- `title` — Ad title
- `description` — Ad description
- `price` — Ad price in USD
- `start_date` — Ad start date
- `end_date` — Ad end date
- `status` — Ad status (active, paused, expired)
- `created_at` — Ad creation timestamp
- `updated_at` — Last update timestamp

---

## NFTs Table

Stores NFT certificate information.

```sql
CREATE TABLE nfts (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  creator_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  page_id UUID REFERENCES creator_pages(id) ON DELETE SET NULL,
  contract_address VARCHAR(255) NOT NULL,
  token_id VARCHAR(255) NOT NULL,
  metadata_uri TEXT NOT NULL,
  blockchain VARCHAR(50) NOT NULL DEFAULT 'solana',
  owner_address VARCHAR(255),
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_nfts_creator_id ON nfts(creator_id);
CREATE INDEX idx_nfts_page_id ON nfts(page_id);
CREATE INDEX idx_nfts_contract_address ON nfts(contract_address);
CREATE INDEX idx_nfts_owner_address ON nfts(owner_address);
```

**Fields:**
- `id` — Unique NFT identifier
- `creator_id` — Reference to creator (user)
- `page_id` — Reference to creator page
- `contract_address` — Solana contract address
- `token_id` — NFT token ID
- `metadata_uri` — IPFS metadata URI
- `blockchain` — Blockchain name (solana)
- `owner_address` — Current owner wallet address
- `created_at` — NFT creation timestamp

---

## Payments Table

Stores payment transaction information.

```sql
CREATE TABLE payments (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  amount INTEGER NOT NULL,
  currency VARCHAR(3) DEFAULT 'usd',
  status VARCHAR(50) NOT NULL CHECK (status IN ('pending', 'succeeded', 'failed', 'refunded')),
  stripe_payment_id VARCHAR(255) UNIQUE NOT NULL,
  description TEXT,
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_payments_user_id ON payments(user_id);
CREATE INDEX idx_payments_status ON payments(status);
CREATE INDEX idx_payments_stripe_payment_id ON payments(stripe_payment_id);
CREATE INDEX idx_payments_created_at ON payments(created_at);
```

**Fields:**
- `id` — Unique payment identifier
- `user_id` — Reference to user
- `amount` — Amount in cents
- `currency` — Currency code (usd)
- `status` — Payment status
- `stripe_payment_id` — Stripe payment ID
- `description` — Payment description
- `created_at` — Payment creation timestamp
- `updated_at` — Last update timestamp

---

## Sessions Table

Stores user session information.

```sql
CREATE TABLE sessions (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  token VARCHAR(255) UNIQUE NOT NULL,
  expires_at TIMESTAMP NOT NULL,
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_sessions_user_id ON sessions(user_id);
CREATE INDEX idx_sessions_token ON sessions(token);
CREATE INDEX idx_sessions_expires_at ON sessions(expires_at);
```

**Fields:**
- `id` — Unique session identifier
- `user_id` — Reference to user
- `token` — Session token
- `expires_at` — Token expiration timestamp
- `created_at` — Session creation timestamp

---

## Relationships Diagram

```
users (1) ──────── (M) creator_pages
  │
  ├─ (1) ──────── (M) live_chat
  │
  ├─ (1) ──────── (M) badge_purchases
  │
  ├─ (1) ──────── (M) ads (as advertiser)
  │
  ├─ (1) ──────── (M) nfts
  │
  ├─ (1) ──────── (M) payments
  │
  └─ (1) ──────── (M) sessions

creator_pages (1) ──────── (M) analytics
creator_pages (1) ──────── (M) badges
creator_pages (1) ──────── (M) ads
creator_pages (1) ──────── (M) live_chat

badges (1) ──────── (M) badge_purchases
```

---

## Indexes

Strategic indexes for performance:

| Table | Index | Purpose |
|-------|-------|---------|
| users | email, username, wallet_address | Fast lookups |
| creator_pages | creator_id, tier, created_at | Filtering, sorting |
| live_chat | page_id, user_id, created_at | Message retrieval |
| analytics | page_id, date | Analytics queries |
| badges | page_id, created_at | Badge listing |
| badge_purchases | badge_id, buyer_id, created_at | Purchase history |
| ads | page_id, advertiser_id, status | Ad listing |
| nfts | creator_id, page_id, owner_address | NFT lookup |
| payments | user_id, status, created_at | Payment history |
| sessions | user_id, token, expires_at | Session validation |

---

## Data Types

| Type | Usage | Example |
|------|-------|---------|
| UUID | Primary keys, foreign keys | `550e8400-e29b-41d4-a716-446655440000` |
| VARCHAR | Strings with max length | `VARCHAR(255)` |
| TEXT | Long text | Bio, description |
| INTEGER | Whole numbers | Page views, counts |
| DECIMAL | Money amounts | `DECIMAL(10, 2)` |
| DATE | Dates | `2026-06-08` |
| TIMESTAMP | Dates with time | `2026-06-08T10:00:00Z` |
| BOOLEAN | True/false | `true`, `false` |

---

## Migrations

Schema changes are managed with Drizzle migrations:

```bash
# Generate migration
drizzle-kit generate

# Run migration
drizzle-kit migrate

# Rollback migration
drizzle-kit rollback
```

---

## Backup Strategy

**Daily automated backups:**
- Full database backup
- Point-in-time recovery
- Cross-region replication

**Retention:**
- Daily backups: 7 days
- Weekly backups: 4 weeks
- Monthly backups: 1 year

---

## Performance Optimization

**Connection pooling:**
- Min connections: 5
- Max connections: 20
- Idle timeout: 30 seconds

**Query optimization:**
- All indexes created
- Slow query logging enabled
- Query execution plans reviewed

**Caching:**
- Redis for frequently accessed data
- Cache invalidation on updates
- TTL: 1 hour

---

## Security

**Encryption:**
- Passwords hashed with bcrypt
- Sensitive data encrypted at rest
- SSL/TLS for data in transit

**Access control:**
- Row-level security (RLS) for sensitive data
- User can only see their own data
- Admin access restricted

---

## Conclusion

SKYNZ database schema is normalized, well-indexed, and optimized for performance. All relationships are properly defined with foreign keys. Indexes are strategic for common queries.

The schema supports growth from thousands to millions of records without major changes.
