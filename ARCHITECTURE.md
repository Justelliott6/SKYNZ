# SKYNZ Architecture

Complete system design and technical architecture of the SKYNZ platform.

---

## System Overview

SKYNZ is a three-tier architecture: mobile app (frontend), REST API (backend), and Solana blockchain (NFT layer).

```
┌─────────────────────────────────────────────────────────────┐
│                     SKYNZ Platform                           │
├─────────────────────────────────────────────────────────────┤
│                                                               │
│  ┌──────────────────┐  ┌──────────────────┐                 │
│  │  Mobile App      │  │  Web App         │                 │
│  │  (React Native)  │  │  (Next.js)       │                 │
│  │  iOS/Android     │  │  Landing Page    │                 │
│  └────────┬─────────┘  └────────┬─────────┘                 │
│           │                     │                            │
│           └─────────────┬───────┘                            │
│                         │                                    │
│                    ┌────▼─────────┐                          │
│                    │  REST API    │                          │
│                    │  (Express)   │                          │
│                    └────┬─────────┘                          │
│                         │                                    │
│        ┌────────────────┼────────────────┐                  │
│        │                │                │                  │
│   ┌────▼────┐    ┌─────▼──────┐  ┌─────▼──────┐            │
│   │PostgreSQL│    │  Stripe    │  │  Solana    │            │
│   │Database  │    │  Payments  │  │ Blockchain │            │
│   └──────────┘    └────────────┘  └────────────┘            │
│                                                               │
│   ┌──────────────┐  ┌──────────────┐  ┌──────────────┐     │
│   │  AWS S3      │  │  IPFS        │  │  SendGrid    │     │
│   │  File Store  │  │  NFT Meta    │  │  Email       │     │
│   └──────────────┘  └──────────────┘  └──────────────┘     │
│                                                               │
└─────────────────────────────────────────────────────────────┘
```

---

## Layer 1: Frontend

### Mobile App (React Native + Expo)

**Responsibilities:**
- Creator page builder UI
- Live chat interface
- Analytics dashboard
- Wallet connection (Phantom)
- Payment flow (Stripe)
- NFT display

**Key Screens:**
- Home (creator discovery)
- Creator page (live chat, analytics, music)
- Page builder (create/edit page)
- Wallet (connect Phantom)
- Marketplace (buy badges)
- Profile (creator stats)

**State Management:**
- React Context for global state
- AsyncStorage for local persistence
- TanStack Query for server state

### Web App (Next.js)

**Responsibilities:**
- Landing page
- Creator profiles (public)
- Marketplace (public)
- Admin dashboard
- Blog/content

**Key Pages:**
- Landing page (teaser + waitlist)
- Creator profiles (public)
- Badge marketplace
- Blog
- Admin dashboard

---

## Layer 2: Backend API

### Express.js REST API

**Responsibilities:**
- User authentication
- Creator page management
- Live chat backend
- Analytics aggregation
- Payment processing
- NFT minting coordination

**API Endpoints:**

**Authentication**
- `POST /auth/login` — User login
- `POST /auth/register` — User registration
- `POST /auth/logout` — User logout
- `GET /auth/me` — Current user info

**Creator Pages**
- `GET /pages` — List all pages
- `GET /pages/:id` — Get page details
- `POST /pages` — Create new page
- `PUT /pages/:id` — Update page
- `DELETE /pages/:id` — Delete page

**Live Chat**
- `GET /pages/:id/chat` — Get chat history
- `POST /pages/:id/chat` — Send message
- `WS /pages/:id/chat` — WebSocket for real-time

**Analytics**
- `GET /pages/:id/analytics` — Get page analytics
- `GET /pages/:id/analytics/daily` — Daily breakdown
- `GET /pages/:id/analytics/visitors` — Visitor data

**Marketplace**
- `GET /badges` — List all badges
- `GET /badges/:id` — Get badge details
- `POST /badges` — Create badge
- `GET /marketplace` — Marketplace listings
- `POST /marketplace/:id/buy` — Purchase badge

**Payments**
- `POST /payments/create-intent` — Stripe payment intent
- `POST /payments/webhook` — Stripe webhook
- `GET /payments/history` — Payment history

**NFTs**
- `POST /nfts/mint` — Mint NFT certificate
- `GET /nfts/:id` — Get NFT details
- `GET /nfts/verify` — Verify NFT ownership

---

## Layer 3: Database

### PostgreSQL Schema

**Core Tables:**

**Users**
- id (UUID)
- email (unique)
- password_hash
- username
- profile_picture
- created_at
- updated_at

**Creator Pages**
- id (UUID)
- creator_id (FK to users)
- title
- description
- tier (basic, pro, supermodel)
- nft_certificate_id (FK to NFTs)
- live_chat_enabled
- created_at
- updated_at

**Live Chat**
- id (UUID)
- page_id (FK to pages)
- user_id (FK to users)
- message
- created_at

**Analytics**
- id (UUID)
- page_id (FK to pages)
- date
- page_views
- unique_visitors
- watch_time
- chat_messages
- created_at

**Badges**
- id (UUID)
- page_id (FK to pages)
- title
- description
- price
- supply_limit
- nft_contract_address
- created_at

**Badge Purchases**
- id (UUID)
- badge_id (FK to badges)
- buyer_id (FK to users)
- transaction_id
- price_paid
- created_at

**Ads**
- id (UUID)
- page_id (FK to pages)
- advertiser_id (FK to users)
- title
- description
- price
- start_date
- end_date
- created_at

**NFTs**
- id (UUID)
- creator_id (FK to users)
- contract_address
- token_id
- metadata_uri
- blockchain (solana)
- created_at

---

## Layer 4: Blockchain (Solana)

### Smart Contracts

**NFT Certificate Program**
- Mints NFT certificates for creator pages
- Uses Metaplex Token Metadata Program
- Stores page tier and serial number
- Verifies creator ownership

**Badge Program**
- Mints collectible badges
- Links to creator page
- Tracks supply limit
- Enables trading

### Wallet Integration

**Phantom Wallet**
- User connects wallet
- Signs transactions
- Receives NFTs
- Manages tokens

### NFT Metadata (IPFS)

**Certificate Metadata**
```json
{
  "name": "SKYNZ Pro Page #47",
  "description": "Creator-owned page on SKYNZ",
  "image": "ipfs://...",
  "attributes": [
    {"trait_type": "Tier", "value": "Pro"},
    {"trait_type": "Creator", "value": "Elliott R Palmer"},
    {"trait_type": "Serial", "value": "47"},
    {"trait_type": "Created", "value": "2026-06-08"}
  ]
}
```

---

## Data Flow

### 1. Creator Page Creation

```
User → Mobile App → Express API → PostgreSQL
                 ↓
              Stripe (payment)
                 ↓
            Solana (mint NFT)
                 ↓
              IPFS (metadata)
                 ↓
            Page created ✓
```

### 2. Live Chat

```
User 1 → Mobile App → WebSocket → Express API → PostgreSQL
                                        ↓
                                   Broadcast
                                        ↓
User 2 ← Mobile App ← WebSocket ← Express API
```

### 3. Badge Purchase

```
Buyer → Mobile App → Express API → Stripe (payment)
                                        ↓
                                   Solana (mint NFT)
                                        ↓
                                   IPFS (metadata)
                                        ↓
                                   PostgreSQL (record)
                                        ↓
                              Badge minted ✓
```

### 4. Analytics

```
Events → Mobile App → Express API → PostgreSQL
                                        ↓
                                   Aggregation
                                        ↓
                              Dashboard display
```

---

## Security Architecture

### Authentication

**JWT Tokens**
- Access token (15 minutes)
- Refresh token (7 days)
- Stored securely on mobile

**OAuth 2.0**
- Google login
- Apple login
- Optional for web

### Authorization

**Role-Based Access Control (RBAC)**
- Creator (can manage own page)
- Fan (can buy badges, chat)
- Admin (can moderate, manage platform)

### Data Protection

**Encryption**
- HTTPS for all API calls
- TLS 1.3 minimum
- Database encryption at rest

**Secrets Management**
- AWS Secrets Manager
- Environment variables
- No secrets in code

### Blockchain Security

**Smart Contract Audits**
- Use Metaplex standard (audited)
- No custom contract logic
- Minimize attack surface

**Wallet Security**
- Phantom handles key management
- No private keys stored in app
- User responsible for wallet security

---

## Scalability Architecture

### Database Scaling

**Read Replicas**
- Primary database for writes
- Read replicas for analytics
- Reduces query load

**Caching**
- Redis for frequently accessed data
- Cache invalidation strategy
- Reduces database load

### API Scaling

**Load Balancing**
- Multiple Express instances
- AWS Application Load Balancer
- Auto-scaling based on traffic

**Horizontal Scaling**
- Stateless API design
- Session stored in database
- Can scale to multiple servers

### File Storage Scaling

**AWS S3**
- Unlimited scalability
- CDN integration (CloudFront)
- Automatic replication

**IPFS**
- Distributed storage
- Redundancy built-in
- Content-addressed

---

## Deployment Architecture

### Development Environment

**Local Development**
- Expo dev server (mobile)
- Express dev server (backend)
- PostgreSQL local instance
- Solana devnet

### Staging Environment

**Pre-production Testing**
- AWS EC2 instances
- RDS PostgreSQL
- Solana testnet
- Stripe test mode

### Production Environment

**Live Platform**
- AWS App Runner (API)
- AWS RDS (database)
- AWS S3 (files)
- Solana mainnet
- Stripe production

---

## Monitoring & Observability

### Logging

**Application Logs**
- Winston logger
- Structured JSON format
- CloudWatch integration

**API Logs**
- Request/response logging
- Performance metrics
- Error tracking

### Metrics

**Application Metrics**
- API response time
- Database query time
- Error rate
- Uptime

**Business Metrics**
- Creator count
- Page views
- Revenue
- User retention

### Alerting

**Critical Alerts**
- API down
- Database down
- Payment processing errors
- Blockchain errors

**Warning Alerts**
- High error rate
- Slow queries
- High memory usage
- Disk space low

---

## Disaster Recovery

### Backup Strategy

**Database Backups**
- Daily automated backups
- Point-in-time recovery
- Cross-region replication

**File Backups**
- S3 versioning enabled
- Cross-region replication
- Lifecycle policies

### Recovery Plan

**Recovery Time Objective (RTO):** 1 hour
**Recovery Point Objective (RPO):** 1 hour

**Procedures:**
- Database restore from backup
- File restore from S3
- API redeployment
- User notification

---

## API Rate Limiting

**Endpoints**
- 100 requests per minute (authenticated)
- 10 requests per minute (unauthenticated)
- 1000 requests per hour (payment endpoints)

**Purpose**
- Prevent abuse
- Ensure fair usage
- Protect against DDoS

---

## Conclusion

SKYNZ architecture is designed for scalability, security, and reliability. The three-tier design (frontend, backend, blockchain) separates concerns and enables independent scaling. Solana blockchain integration provides ultra-cheap NFT minting. PostgreSQL provides reliable data persistence. AWS infrastructure provides enterprise-grade reliability.

**The architecture supports growth from 10 creators to 10,000+ creators without major changes.**
