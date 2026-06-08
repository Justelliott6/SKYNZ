# SKYNZ API Specification

Complete REST API documentation for SKYNZ backend.

---

## Base URL

**Development:** `http://localhost:3000`
**Staging:** `https://staging-api.skynz.io`
**Production:** `https://api.skynz.io`

---

## Authentication

All endpoints (except public ones) require JWT token in Authorization header:

```
Authorization: Bearer <jwt_token>
```

### Login Endpoint

**POST** `/auth/login`

Request:
```json
{
  "email": "creator@example.com",
  "password": "secure_password"
}
```

Response (200):
```json
{
  "access_token": "eyJhbGc...",
  "refresh_token": "eyJhbGc...",
  "user": {
    "id": "uuid",
    "email": "creator@example.com",
    "username": "creator_name"
  }
}
```

### Register Endpoint

**POST** `/auth/register`

Request:
```json
{
  "email": "creator@example.com",
  "password": "secure_password",
  "username": "creator_name"
}
```

Response (201):
```json
{
  "id": "uuid",
  "email": "creator@example.com",
  "username": "creator_name"
}
```

---

## Creator Pages API

### List All Pages

**GET** `/pages`

Query Parameters:
- `limit` (optional): Number of results (default: 20)
- `offset` (optional): Pagination offset (default: 0)
- `tier` (optional): Filter by tier (basic, pro, supermodel)

Response (200):
```json
{
  "data": [
    {
      "id": "uuid",
      "creator_id": "uuid",
      "title": "Elliott's Page",
      "description": "Music and vibes",
      "tier": "pro",
      "nft_certificate_id": "uuid",
      "live_chat_enabled": true,
      "created_at": "2026-06-08T10:00:00Z"
    }
  ],
  "total": 100,
  "limit": 20,
  "offset": 0
}
```

### Get Page Details

**GET** `/pages/:id`

Response (200):
```json
{
  "id": "uuid",
  "creator_id": "uuid",
  "title": "Elliott's Page",
  "description": "Music and vibes",
  "tier": "pro",
  "nft_certificate_id": "uuid",
  "live_chat_enabled": true,
  "music_widgets": [
    {
      "id": "uuid",
      "platform": "spotify",
      "playlist_id": "spotify_id"
    }
  ],
  "created_at": "2026-06-08T10:00:00Z"
}
```

### Create Page

**POST** `/pages`

Request:
```json
{
  "title": "Elliott's Page",
  "description": "Music and vibes",
  "tier": "pro"
}
```

Response (201):
```json
{
  "id": "uuid",
  "creator_id": "uuid",
  "title": "Elliott's Page",
  "description": "Music and vibes",
  "tier": "pro",
  "nft_certificate_id": "uuid",
  "created_at": "2026-06-08T10:00:00Z"
}
```

### Update Page

**PUT** `/pages/:id`

Request:
```json
{
  "title": "Elliott's Updated Page",
  "description": "New description"
}
```

Response (200):
```json
{
  "id": "uuid",
  "title": "Elliott's Updated Page",
  "description": "New description",
  "updated_at": "2026-06-08T11:00:00Z"
}
```

### Delete Page

**DELETE** `/pages/:id`

Response (204): No content

---

## Live Chat API

### Get Chat History

**GET** `/pages/:id/chat`

Query Parameters:
- `limit` (optional): Number of messages (default: 50)
- `offset` (optional): Pagination offset (default: 0)

Response (200):
```json
{
  "data": [
    {
      "id": "uuid",
      "user_id": "uuid",
      "username": "fan_name",
      "message": "Great content!",
      "created_at": "2026-06-08T10:00:00Z"
    }
  ],
  "total": 500,
  "limit": 50,
  "offset": 0
}
```

### Send Chat Message

**POST** `/pages/:id/chat`

Request:
```json
{
  "message": "Great content!"
}
```

Response (201):
```json
{
  "id": "uuid",
  "user_id": "uuid",
  "username": "fan_name",
  "message": "Great content!",
  "created_at": "2026-06-08T10:00:00Z"
}
```

### WebSocket Connection

**WS** `/pages/:id/chat`

Connect with JWT token in query:
```
ws://localhost:3000/pages/:id/chat?token=<jwt_token>
```

Events:
- `message:new` — New message received
- `user:joined` — User joined chat
- `user:left` — User left chat
- `message:deleted` — Message deleted

---

## Analytics API

### Get Page Analytics

**GET** `/pages/:id/analytics`

Query Parameters:
- `period` (optional): daily, weekly, monthly (default: daily)
- `days` (optional): Number of days (default: 30)

Response (200):
```json
{
  "page_id": "uuid",
  "period": "daily",
  "data": [
    {
      "date": "2026-06-08",
      "page_views": 1200,
      "unique_visitors": 450,
      "watch_time_minutes": 3600,
      "chat_messages": 150
    }
  ],
  "summary": {
    "total_page_views": 36000,
    "total_unique_visitors": 13500,
    "total_watch_time_hours": 108,
    "total_chat_messages": 4500
  }
}
```

### Get Visitor Details

**GET** `/pages/:id/analytics/visitors`

Response (200):
```json
{
  "page_id": "uuid",
  "visitors": [
    {
      "user_id": "uuid",
      "username": "fan_name",
      "first_visit": "2026-06-01T10:00:00Z",
      "last_visit": "2026-06-08T10:00:00Z",
      "visit_count": 5,
      "watch_time_minutes": 120
    }
  ]
}
```

---

## Marketplace API

### List Badges

**GET** `/badges`

Query Parameters:
- `page_id` (optional): Filter by creator page
- `limit` (optional): Number of results (default: 20)
- `offset` (optional): Pagination offset (default: 0)

Response (200):
```json
{
  "data": [
    {
      "id": "uuid",
      "page_id": "uuid",
      "title": "SKYNZ Pro Member",
      "description": "Exclusive badge",
      "price": 9.99,
      "supply_limit": 100,
      "supply_minted": 45,
      "nft_contract_address": "0x...",
      "created_at": "2026-06-08T10:00:00Z"
    }
  ],
  "total": 500,
  "limit": 20,
  "offset": 0
}
```

### Get Badge Details

**GET** `/badges/:id`

Response (200):
```json
{
  "id": "uuid",
  "page_id": "uuid",
  "title": "SKYNZ Pro Member",
  "description": "Exclusive badge",
  "price": 9.99,
  "supply_limit": 100,
  "supply_minted": 45,
  "nft_contract_address": "0x...",
  "metadata_uri": "ipfs://...",
  "created_at": "2026-06-08T10:00:00Z"
}
```

### Create Badge

**POST** `/badges`

Request:
```json
{
  "page_id": "uuid",
  "title": "SKYNZ Pro Member",
  "description": "Exclusive badge",
  "price": 9.99,
  "supply_limit": 100
}
```

Response (201):
```json
{
  "id": "uuid",
  "page_id": "uuid",
  "title": "SKYNZ Pro Member",
  "price": 9.99,
  "supply_limit": 100,
  "created_at": "2026-06-08T10:00:00Z"
}
```

### Buy Badge

**POST** `/badges/:id/buy`

Request:
```json
{
  "quantity": 1
}
```

Response (201):
```json
{
  "transaction_id": "uuid",
  "badge_id": "uuid",
  "quantity": 1,
  "total_price": 9.99,
  "nft_minted": true,
  "created_at": "2026-06-08T10:00:00Z"
}
```

---

## Payments API

### Create Payment Intent

**POST** `/payments/create-intent`

Request:
```json
{
  "amount": 9999,
  "currency": "usd",
  "description": "SKYNZ Pro Page"
}
```

Response (200):
```json
{
  "client_secret": "pi_1234567890_secret_abcdefg",
  "amount": 9999,
  "currency": "usd"
}
```

### Payment Webhook

**POST** `/payments/webhook`

Stripe sends webhook events:
- `payment_intent.succeeded` — Payment successful
- `payment_intent.payment_failed` — Payment failed
- `charge.refunded` — Refund processed

### Get Payment History

**GET** `/payments/history`

Query Parameters:
- `limit` (optional): Number of results (default: 20)
- `offset` (optional): Pagination offset (default: 0)

Response (200):
```json
{
  "data": [
    {
      "id": "uuid",
      "amount": 9999,
      "currency": "usd",
      "status": "succeeded",
      "description": "SKYNZ Pro Page",
      "created_at": "2026-06-08T10:00:00Z"
    }
  ],
  "total": 50,
  "limit": 20,
  "offset": 0
}
```

---

## NFT API

### Mint NFT Certificate

**POST** `/nfts/mint`

Request:
```json
{
  "page_id": "uuid",
  "tier": "pro",
  "serial_number": 47
}
```

Response (201):
```json
{
  "id": "uuid",
  "contract_address": "0x...",
  "token_id": "47",
  "metadata_uri": "ipfs://...",
  "blockchain": "solana",
  "created_at": "2026-06-08T10:00:00Z"
}
```

### Get NFT Details

**GET** `/nfts/:id`

Response (200):
```json
{
  "id": "uuid",
  "contract_address": "0x...",
  "token_id": "47",
  "metadata_uri": "ipfs://...",
  "owner_address": "0x...",
  "blockchain": "solana",
  "created_at": "2026-06-08T10:00:00Z"
}
```

### Verify NFT Ownership

**POST** `/nfts/verify`

Request:
```json
{
  "nft_id": "uuid",
  "wallet_address": "0x..."
}
```

Response (200):
```json
{
  "verified": true,
  "owner": "0x...",
  "nft_id": "uuid"
}
```

---

## User Profile API

### Get Current User

**GET** `/users/me`

Response (200):
```json
{
  "id": "uuid",
  "email": "creator@example.com",
  "username": "creator_name",
  "profile_picture": "https://...",
  "follower_count": 5000,
  "following_count": 150,
  "created_at": "2026-01-01T10:00:00Z"
}
```

### Update Profile

**PUT** `/users/me`

Request:
```json
{
  "username": "new_username",
  "profile_picture": "https://...",
  "bio": "Creator bio"
}
```

Response (200):
```json
{
  "id": "uuid",
  "username": "new_username",
  "profile_picture": "https://...",
  "bio": "Creator bio",
  "updated_at": "2026-06-08T10:00:00Z"
}
```

---

## Error Responses

All errors follow this format:

```json
{
  "error": {
    "code": "INVALID_REQUEST",
    "message": "Invalid request parameters",
    "details": {
      "field": "email",
      "reason": "Invalid email format"
    }
  }
}
```

### Common Error Codes

| Code | Status | Meaning |
|------|--------|---------|
| INVALID_REQUEST | 400 | Bad request |
| UNAUTHORIZED | 401 | Missing/invalid token |
| FORBIDDEN | 403 | Not authorized |
| NOT_FOUND | 404 | Resource not found |
| CONFLICT | 409 | Resource conflict |
| RATE_LIMIT | 429 | Too many requests |
| SERVER_ERROR | 500 | Server error |

---

## Rate Limiting

API rate limits:
- **Authenticated:** 100 requests/minute
- **Unauthenticated:** 10 requests/minute
- **Payment endpoints:** 1000 requests/hour

Response headers include:
- `X-RateLimit-Limit` — Request limit
- `X-RateLimit-Remaining` — Requests remaining
- `X-RateLimit-Reset` — Reset time (Unix timestamp)

---

## Pagination

List endpoints support pagination:

Query Parameters:
- `limit` — Number of results (default: 20, max: 100)
- `offset` — Pagination offset (default: 0)

Response includes:
- `data` — Array of results
- `total` — Total number of items
- `limit` — Limit used
- `offset` — Offset used

---

## Versioning

API version in URL: `/v1/`

Current version: `v1`

---

## Testing

### Test Credentials

**Email:** test@skynz.io
**Password:** test_password_123

### Test Payment Cards

**Visa:** 4242 4242 4242 4242
**Mastercard:** 5555 5555 5555 4444
**Expiry:** Any future date
**CVC:** Any 3 digits

---

## Conclusion

SKYNZ API is RESTful, well-documented, and production-ready. All endpoints are tested and versioned. Rate limiting and error handling are built-in.

See [DEVELOPMENT_ROADMAP.md](./DEVELOPMENT_ROADMAP.md) for implementation timeline.
