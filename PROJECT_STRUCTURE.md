# SKYNZ Project Structure

Complete file organization and project structure for SKYNZ.

---

## Directory Overview

```
SKYNZ/
├── app/                          # React Native mobile app
├── server/                       # Node.js backend API
├── contracts/                    # Solana smart contracts
├── landing-page/                 # Marketing website
├── docs/                         # Documentation
├── scripts/                      # Utility scripts
├── tests/                        # Test files
├── .github/                      # GitHub configuration
├── .env.example                  # Environment variables template
├── .gitignore                    # Git ignore rules
├── package.json                  # Root package configuration
├── pnpm-workspace.yaml          # PNPM workspace config
├── tsconfig.json                # TypeScript configuration
├── README.md                    # Main README
├── TECH_STACK.md               # Technology documentation
├── BUSINESS_MODEL.md           # Business model
├── FUNDING.md                  # Funding strategy
├── ARCHITECTURE.md             # System architecture
├── DEVELOPMENT_ROADMAP.md      # Development timeline
├── API_SPECIFICATION.md        # API documentation
├── DATABASE_SCHEMA.md          # Database schema
├── SMART_CONTRACTS.md          # Smart contract docs
├── PROJECT_STRUCTURE.md        # This file
├── SETUP_GUIDE.md             # Setup instructions
├── DEPLOYMENT.md              # Deployment guide
└── CONTRIBUTING.md            # Contributing guidelines
```

---

## App Directory (React Native)

```
app/
├── app/                         # Expo Router screens
│   ├── _layout.tsx             # Root layout
│   ├── (tabs)/                 # Tab-based navigation
│   │   ├── _layout.tsx         # Tab bar configuration
│   │   ├── index.tsx           # Home screen
│   │   ├── explore.tsx         # Explore/discover
│   │   ├── profile.tsx         # User profile
│   │   └── settings.tsx        # Settings
│   ├── auth/                   # Authentication screens
│   │   ├── login.tsx           # Login screen
│   │   ├── register.tsx        # Registration screen
│   │   └── forgot-password.tsx # Password reset
│   ├── pages/                  # Creator page screens
│   │   ├── [id].tsx            # Page detail
│   │   ├── create.tsx          # Create page
│   │   ├── edit.tsx            # Edit page
│   │   └── analytics.tsx       # Page analytics
│   ├── marketplace/            # Badge marketplace
│   │   ├── index.tsx           # Marketplace listing
│   │   ├── [id].tsx            # Badge detail
│   │   └── purchases.tsx       # My badges
│   └── oauth/                  # OAuth callbacks
│       └── callback.tsx        # OAuth redirect
│
├── components/                 # Reusable components
│   ├── screen-container.tsx   # SafeArea wrapper
│   ├── themed-view.tsx        # Themed container
│   ├── haptic-tab.tsx         # Tab with haptics
│   ├── ui/                    # UI components
│   │   ├── icon-symbol.tsx    # Icon mapping
│   │   ├── button.tsx         # Button component
│   │   ├── input.tsx          # Input component
│   │   ├── card.tsx           # Card component
│   │   ├── modal.tsx          # Modal component
│   │   └── badge.tsx          # Badge display
│   ├── chat/                  # Chat components
│   │   ├── chat-message.tsx   # Message item
│   │   ├── chat-input.tsx     # Message input
│   │   └── chat-list.tsx      # Message list
│   ├── analytics/             # Analytics components
│   │   ├── stats-card.tsx     # Stat display
│   │   ├── chart.tsx          # Chart component
│   │   └── analytics-view.tsx # Analytics dashboard
│   └── marketplace/           # Marketplace components
│       ├── badge-card.tsx     # Badge display
│       ├── badge-grid.tsx     # Badge grid
│       └── purchase-modal.tsx # Purchase flow
│
├── hooks/                      # Custom React hooks
│   ├── use-auth.ts            # Authentication hook
│   ├── use-colors.ts          # Theme colors hook
│   ├── use-color-scheme.ts    # Dark mode hook
│   ├── use-chat.ts            # Chat hook
│   ├── use-analytics.ts       # Analytics hook
│   ├── use-wallet.ts          # Wallet hook
│   └── use-marketplace.ts     # Marketplace hook
│
├── lib/                        # Utility libraries
│   ├── _core/                 # Core utilities
│   │   ├── api.ts             # API client
│   │   ├── auth.ts            # Auth utilities
│   │   ├── theme.ts           # Theme utilities
│   │   └── nativewind-pressable.ts
│   ├── theme-provider.tsx     # Theme context
│   ├── utils.ts               # Utility functions
│   └── trpc.ts                # tRPC client
│
├── constants/                  # Constants
│   ├── const.ts               # App constants
│   ├── oauth.ts               # OAuth config
│   └── theme.ts               # Theme constants
│
├── assets/                     # Static assets
│   ├── images/                # Image files
│   │   ├── icon.png           # App icon
│   │   ├── splash-icon.png    # Splash screen
│   │   ├── favicon.png        # Web favicon
│   │   ├── android-icon-foreground.png
│   │   ├── android-icon-background.png
│   │   └── android-icon-monochrome.png
│   └── fonts/                 # Custom fonts
│
├── app.config.ts              # Expo configuration
├── babel.config.js            # Babel configuration
├── metro.config.js            # Metro bundler config
├── tailwind.config.js         # Tailwind configuration
├── theme.config.js            # Theme colors
├── theme.config.d.ts          # Theme types
├── global.css                 # Global styles
├── tsconfig.json              # TypeScript config
├── package.json               # App dependencies
└── .env.example               # Environment template
```

---

## Server Directory (Node.js Backend)

```
server/
├── _core/                      # Core services
│   ├── index.ts               # Server entry point
│   ├── context.ts             # Request context
│   ├── auth.ts                # Authentication
│   ├── cookies.ts             # Cookie handling
│   ├── dataApi.ts             # Data API client
│   ├── env.ts                 # Environment config
│   ├── heartbeat.ts           # Health check
│   ├── imageGeneration.ts     # Image generation
│   ├── llm.ts                 # LLM integration
│   ├── notification.ts        # Notifications
│   ├── oauth.ts               # OAuth handling
│   ├── sdk.ts                 # SDK utilities
│   ├── storageProxy.ts        # Storage proxy
│   ├── systemRouter.ts        # System routes
│   ├── trpc.ts                # tRPC setup
│   ├── voiceTranscription.ts  # Voice to text
│   └── types/                 # Type definitions
│       ├── cookie.d.ts        # Cookie types
│       └── manusTypes.ts      # Manus types
│
├── routers/                    # API routers
│   ├── auth.ts                # Auth endpoints
│   ├── pages.ts               # Page endpoints
│   ├── chat.ts                # Chat endpoints
│   ├── analytics.ts           # Analytics endpoints
│   ├── badges.ts              # Badge endpoints
│   ├── marketplace.ts         # Marketplace endpoints
│   ├── payments.ts            # Payment endpoints
│   ├── nfts.ts                # NFT endpoints
│   └── users.ts               # User endpoints
│
├── db.ts                       # Database connection
├── storage.ts                  # Storage utilities
├── routers.ts                  # Router exports
├── package.json               # Server dependencies
└── tsconfig.json              # TypeScript config
```

---

## Contracts Directory (Solana)

```
contracts/
├── programs/                   # Anchor programs
│   ├── skynz-certificate/      # Certificate program
│   │   ├── src/
│   │   │   ├── lib.rs         # Program entry
│   │   │   ├── instructions/  # Instruction handlers
│   │   │   │   ├── mint.rs    # Mint certificate
│   │   │   │   ├── verify.rs  # Verify ownership
│   │   │   │   └── transfer.rs # Transfer cert
│   │   │   ├── state/         # State definitions
│   │   │   │   └── certificate.rs
│   │   │   └── errors.rs      # Error definitions
│   │   ├── Cargo.toml         # Dependencies
│   │   └── Xargo.toml         # Build config
│   │
│   └── skynz-badge/           # Badge program
│       ├── src/
│       │   ├── lib.rs         # Program entry
│       │   ├── instructions/  # Instruction handlers
│       │   │   ├── create.rs  # Create badge
│       │   │   ├── mint.rs    # Mint badge
│       │   │   └── transfer.rs # Transfer badge
│       │   ├── state/         # State definitions
│       │   │   └── badge.rs
│       │   └── errors.rs      # Error definitions
│       ├── Cargo.toml         # Dependencies
│       └── Xargo.toml         # Build config
│
├── tests/                      # Integration tests
│   ├── skynz-certificate.ts   # Certificate tests
│   └── skynz-badge.ts         # Badge tests
│
├── Anchor.toml                # Anchor configuration
├── Cargo.toml                 # Workspace config
└── package.json               # Node dependencies
```

---

## Landing Page Directory

```
landing-page/
├── index.html                 # Main page
├── styles.css                 # Styles
├── script.js                  # Interactivity
├── assets/                    # Images/media
│   ├── logo.png              # SKYNZ logo
│   ├── hero.png              # Hero image
│   ├── feature-1.png         # Feature image
│   ├── feature-2.png         # Feature image
│   └── feature-3.png         # Feature image
└── pages/                     # Additional pages
    ├── about.html            # About page
    ├── pricing.html          # Pricing page
    ├── blog.html             # Blog page
    └── contact.html          # Contact page
```

---

## Docs Directory

```
docs/
├── README.md                  # Documentation index
├── getting-started.md         # Getting started guide
├── architecture/
│   ├── system-design.md      # System design
│   ├── database.md           # Database design
│   └── api-design.md         # API design
├── development/
│   ├── setup.md              # Development setup
│   ├── coding-standards.md   # Code standards
│   ├── testing.md            # Testing guide
│   └── debugging.md          # Debugging guide
├── deployment/
│   ├── production.md         # Production deployment
│   ├── staging.md            # Staging deployment
│   └── monitoring.md         # Monitoring
├── api/
│   ├── authentication.md     # Auth API
│   ├── pages.md              # Pages API
│   ├── chat.md               # Chat API
│   └── marketplace.md        # Marketplace API
└── blockchain/
    ├── solana.md             # Solana integration
    ├── nft-minting.md        # NFT minting
    └── wallet.md             # Wallet integration
```

---

## Scripts Directory

```
scripts/
├── setup.sh                   # Initial setup
├── dev.sh                     # Start development
├── build.sh                   # Build for production
├── deploy.sh                  # Deploy to production
├── migrate.sh                 # Run migrations
├── seed.sh                    # Seed database
├── test.sh                    # Run tests
└── lint.sh                    # Run linting
```

---

## Tests Directory

```
tests/
├── unit/                      # Unit tests
│   ├── auth.test.ts          # Auth tests
│   ├── pages.test.ts         # Page tests
│   ├── chat.test.ts          # Chat tests
│   └── analytics.test.ts     # Analytics tests
├── integration/              # Integration tests
│   ├── api.test.ts           # API tests
│   ├── database.test.ts      # Database tests
│   └── blockchain.test.ts    # Blockchain tests
└── e2e/                       # End-to-end tests
    ├── auth-flow.test.ts     # Auth flow
    ├── page-creation.test.ts # Page creation
    └── purchase-flow.test.ts # Purchase flow
```

---

## GitHub Directory

```
.github/
├── workflows/                 # GitHub Actions
│   ├── test.yml              # Run tests
│   ├── lint.yml              # Run linting
│   ├── build.yml             # Build app
│   ├── deploy.yml            # Deploy to production
│   └── security.yml          # Security checks
├── ISSUE_TEMPLATE/           # Issue templates
│   ├── bug_report.md         # Bug report template
│   ├── feature_request.md    # Feature request template
│   └── question.md           # Question template
└── pull_request_template.md  # PR template
```

---

## Configuration Files

### Root Level

| File | Purpose |
|------|---------|
| `.env.example` | Environment variables template |
| `.gitignore` | Git ignore rules |
| `package.json` | Root dependencies |
| `pnpm-workspace.yaml` | PNPM workspace config |
| `tsconfig.json` | TypeScript config |
| `README.md` | Main README |

### App Level

| File | Purpose |
|------|---------|
| `app.config.ts` | Expo configuration |
| `babel.config.js` | Babel configuration |
| `metro.config.js` | Metro bundler config |
| `tailwind.config.js` | Tailwind configuration |
| `theme.config.js` | Theme colors |
| `global.css` | Global styles |

### Server Level

| File | Purpose |
|------|---------|
| `tsconfig.json` | TypeScript config |
| `package.json` | Server dependencies |

### Contracts Level

| File | Purpose |
|------|---------|
| `Anchor.toml` | Anchor configuration |
| `Cargo.toml` | Rust dependencies |

---

## Asset Organization

### Images

```
assets/images/
├── app/                       # App-specific images
│   ├── icon.png              # App icon
│   ├── splash-icon.png       # Splash screen
│   └── favicon.png           # Web favicon
├── android/                  # Android-specific
│   ├── android-icon-foreground.png
│   ├── android-icon-background.png
│   └── android-icon-monochrome.png
├── marketing/                # Marketing images
│   ├── hero.png              # Hero image
│   ├── feature-1.png         # Feature image
│   └── feature-2.png         # Feature image
└── badges/                   # Badge templates
    ├── basic.png             # Basic badge
    ├── pro.png               # Pro badge
    └── supermodel.png        # SuperModel badge
```

### Fonts

```
assets/fonts/
├── inter/
│   ├── Inter-Regular.ttf
│   ├── Inter-Bold.ttf
│   └── Inter-SemiBold.ttf
└── roboto-mono/
    ├── RobotoMono-Regular.ttf
    └── RobotoMono-Bold.ttf
```

---

## Environment Variables

### Development (.env.development)

```
REACT_APP_API_URL=http://localhost:3000
REACT_APP_SOLANA_RPC=https://api.devnet.solana.com
REACT_APP_STRIPE_KEY=pk_test_...
REACT_APP_PHANTOM_NETWORK=devnet
```

### Production (.env.production)

```
REACT_APP_API_URL=https://api.skynz.io
REACT_APP_SOLANA_RPC=https://api.mainnet-beta.solana.com
REACT_APP_STRIPE_KEY=pk_live_...
REACT_APP_PHANTOM_NETWORK=mainnet-beta
```

---

## Naming Conventions

### Files

- **Components:** PascalCase (e.g., `ChatMessage.tsx`)
- **Utilities:** camelCase (e.g., `formatDate.ts`)
- **Constants:** UPPER_SNAKE_CASE (e.g., `API_BASE_URL.ts`)
- **Tests:** `.test.ts` or `.spec.ts` suffix

### Directories

- **Components:** PascalCase (e.g., `components/ChatMessage/`)
- **Utilities:** camelCase (e.g., `lib/utils/`)
- **Features:** kebab-case (e.g., `app/auth-flow/`)

### Imports

```typescript
// Absolute imports (preferred)
import { Button } from '@/components/ui/button';
import { formatDate } from '@/lib/utils';

// Relative imports (when necessary)
import { ChatMessage } from '../components/ChatMessage';
```

---

## Build Output

### App Build

```
build/
├── ios/                       # iOS build
├── android/                   # Android build
└── web/                       # Web build
```

### Server Build

```
dist/
├── index.js                   # Compiled server
├── routers/                   # Compiled routers
└── _core/                     # Compiled core
```

### Contracts Build

```
target/
├── deploy/                    # Deployed programs
│   ├── skynz_certificate.so
│   └── skynz_badge.so
└── idl/                       # IDL files
    ├── skynz_certificate.json
    └── skynz_badge.json
```

---

## Conclusion

SKYNZ project structure is organized, scalable, and follows industry best practices. Each directory has a clear purpose. Configuration is centralized. Assets are organized logically.

The structure supports growth from MVP to enterprise-scale platform.
