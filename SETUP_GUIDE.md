# SKYNZ Setup Guide

Complete setup instructions for SKYNZ development environment.

---

## Prerequisites

Before starting, ensure you have:

- **Node.js:** v18+ ([download](https://nodejs.org/))
- **Git:** Latest version ([download](https://git-scm.com/))
- **Solana CLI:** Latest version ([install](https://docs.solana.com/cli/install-solana-cli-tools))
- **Rust:** Latest version ([install](https://rustup.rs/))
- **Anchor:** Latest version (`cargo install --git https://github.com/coral-xyz/anchor avm --locked && avm install latest && avm use latest`)

---

## 1. Clone Repository

```bash
git clone https://github.com/Justelliott6/SKYNZ.git
cd SKYNZ
```

---

## 2. Install Dependencies

### Root Dependencies

```bash
# Install pnpm (if not already installed)
npm install -g pnpm

# Install all dependencies
pnpm install
```

### App Dependencies

```bash
cd app
pnpm install
cd ..
```

### Server Dependencies

```bash
cd server
pnpm install
cd ..
```

### Contracts Dependencies

```bash
cd contracts
cargo build
cd ..
```

---

## 3. Environment Setup

### Create Environment Files

**App (.env.development)**

```bash
cp app/.env.example app/.env.development
```

Edit `app/.env.development`:

```
REACT_APP_API_URL=http://localhost:3000
REACT_APP_SOLANA_RPC=https://api.devnet.solana.com
REACT_APP_STRIPE_KEY=pk_test_YOUR_KEY
REACT_APP_PHANTOM_NETWORK=devnet
```

**Server (.env.development)**

```bash
cp server/.env.example server/.env.development
```

Edit `server/.env.development`:

```
NODE_ENV=development
PORT=3000
DATABASE_URL=postgresql://user:password@localhost:5432/skynz_dev
STRIPE_SECRET_KEY=sk_test_YOUR_KEY
SOLANA_RPC_URL=https://api.devnet.solana.com
SOLANA_NETWORK=devnet
JWT_SECRET=your_jwt_secret_key
```

---

## 4. Database Setup

### Install PostgreSQL

**macOS (Homebrew):**
```bash
brew install postgresql
brew services start postgresql
```

**Linux (Ubuntu):**
```bash
sudo apt-get install postgresql postgresql-contrib
sudo service postgresql start
```

**Windows:**
Download from [postgresql.org](https://www.postgresql.org/download/windows/)

### Create Database

```bash
# Connect to PostgreSQL
psql postgres

# Create database
CREATE DATABASE skynz_dev;

# Create user
CREATE USER skynz_user WITH PASSWORD 'password';

# Grant privileges
ALTER ROLE skynz_user SET client_encoding TO 'utf8';
ALTER ROLE skynz_user SET default_transaction_isolation TO 'read committed';
ALTER ROLE skynz_user SET default_transaction_deferrable TO on;
GRANT ALL PRIVILEGES ON DATABASE skynz_dev TO skynz_user;

# Exit psql
\q
```

### Run Migrations

```bash
cd server
pnpm run db:push
cd ..
```

---

## 5. Solana Setup

### Create Solana Keypair

```bash
# Create keypair (if you don't have one)
solana-keygen new

# Check keypair
solana address
```

### Configure Solana CLI

```bash
# Set to devnet
solana config set --url devnet

# Verify configuration
solana config get
```

### Airdrop SOL (Devnet Only)

```bash
# Request airdrop
solana airdrop 2

# Check balance
solana balance
```

---

## 6. Stripe Setup

### Create Stripe Account

1. Go to [stripe.com](https://stripe.com)
2. Sign up for account
3. Go to Dashboard → Developers → API Keys
4. Copy Test keys

### Add to Environment

Update `.env.development` with Stripe keys:

```
STRIPE_SECRET_KEY=sk_test_YOUR_KEY
REACT_APP_STRIPE_KEY=pk_test_YOUR_KEY
```

---

## 7. Smart Contracts Setup

### Build Contracts

```bash
cd contracts
cargo build-bpf
cd ..
```

### Deploy to Devnet

```bash
cd contracts

# Build for devnet
cargo build-bpf --release

# Deploy
solana program deploy target/deploy/skynz_certificate.so --url devnet

# Get program ID
solana address -k target/deploy/skynz_certificate-keypair.json

cd ..
```

### Update Program IDs

Update `server/.env.development`:

```
CERTIFICATE_PROGRAM_ID=<PROGRAM_ID_FROM_ABOVE>
BADGE_PROGRAM_ID=<BADGE_PROGRAM_ID>
```

---

## 8. Start Development Server

### Terminal 1: Backend API

```bash
cd server
pnpm run dev
```

Expected output:
```
Server running on http://localhost:3000
```

### Terminal 2: React Native App

```bash
cd app
pnpm run dev
```

Expected output:
```
Metro Bundler started
Expo dev server running at http://localhost:8081
```

### Terminal 3: Smart Contracts (Optional)

```bash
cd contracts
cargo watch -x build-bpf
```

---

## 9. Access Applications

### Mobile App

**iOS:**
```bash
cd app
pnpm run ios
```

**Android:**
```bash
cd app
pnpm run android
```

**Web:**
Open browser: http://localhost:8081

### Backend API

Base URL: http://localhost:3000

### Landing Page

Open `landing-page/index.html` in browser

---

## 10. Database Management

### Connect to Database

```bash
psql postgresql://skynz_user:password@localhost:5432/skynz_dev
```

### View Tables

```sql
\dt
```

### View Specific Table

```sql
SELECT * FROM users;
```

### Reset Database

```bash
cd server
pnpm run db:reset
cd ..
```

---

## 11. Testing

### Run Unit Tests

```bash
pnpm run test
```

### Run Integration Tests

```bash
pnpm run test:integration
```

### Run E2E Tests

```bash
pnpm run test:e2e
```

### Run Specific Test

```bash
pnpm run test -- auth.test.ts
```

---

## 12. Linting & Formatting

### Run Linter

```bash
pnpm run lint
```

### Fix Linting Issues

```bash
pnpm run lint:fix
```

### Format Code

```bash
pnpm run format
```

---

## 13. Build for Production

### Build App

```bash
cd app
pnpm run build
cd ..
```

### Build Server

```bash
cd server
pnpm run build
cd ..
```

### Build Contracts

```bash
cd contracts
cargo build-bpf --release
cd ..
```

---

## 14. Troubleshooting

### Issue: Port Already in Use

**Solution:**
```bash
# Kill process on port 3000
lsof -ti:3000 | xargs kill -9

# Kill process on port 8081
lsof -ti:8081 | xargs kill -9
```

### Issue: Database Connection Error

**Solution:**
```bash
# Check PostgreSQL is running
pg_isready

# Start PostgreSQL
sudo service postgresql start

# Verify connection string in .env
```

### Issue: Solana RPC Error

**Solution:**
```bash
# Check Solana configuration
solana config get

# Set to devnet
solana config set --url devnet

# Test connection
solana ping
```

### Issue: Stripe Key Error

**Solution:**
- Verify keys in `.env.development`
- Check keys are not expired
- Ensure you're using Test keys (pk_test_, sk_test_)

### Issue: Smart Contract Deployment Failed

**Solution:**
```bash
# Check balance
solana balance

# Airdrop SOL if needed
solana airdrop 2

# Try deployment again
solana program deploy target/deploy/skynz_certificate.so --url devnet
```

### Issue: Module Not Found

**Solution:**
```bash
# Clear node_modules and reinstall
rm -rf node_modules
pnpm install

# Clear cache
pnpm store prune
```

---

## 15. Development Workflow

### Creating a New Feature

1. Create feature branch
   ```bash
   git checkout -b feature/my-feature
   ```

2. Make changes

3. Run tests
   ```bash
   pnpm run test
   ```

4. Run linter
   ```bash
   pnpm run lint:fix
   ```

5. Commit changes
   ```bash
   git commit -m "feat: add my feature"
   ```

6. Push branch
   ```bash
   git push origin feature/my-feature
   ```

7. Create pull request

### Debugging

**Enable Debug Logging:**
```bash
DEBUG=* pnpm run dev
```

**Debug in VS Code:**
1. Open `.vscode/launch.json`
2. Add breakpoints
3. Press F5 to start debugging

---

## 16. IDE Setup

### VS Code Extensions

Recommended extensions:

- **Prettier** — Code formatter
- **ESLint** — Linting
- **TypeScript Vue Plugin** — TypeScript support
- **Solana** — Solana development
- **Rust** — Rust support
- **Thunder Client** — API testing

### VS Code Settings

Create `.vscode/settings.json`:

```json
{
  "editor.defaultFormatter": "esbenp.prettier-vscode",
  "editor.formatOnSave": true,
  "editor.codeActionsOnSave": {
    "source.fixAll.eslint": true
  },
  "[typescript]": {
    "editor.defaultFormatter": "esbenp.prettier-vscode"
  },
  "[rust]": {
    "editor.defaultFormatter": "rust-lang.rust-analyzer"
  }
}
```

---

## 17. Git Configuration

### Initial Setup

```bash
git config user.name "Your Name"
git config user.email "your.email@example.com"
```

### Commit Message Format

Follow Conventional Commits:

```
feat: add new feature
fix: fix bug
docs: update documentation
style: format code
refactor: refactor code
test: add tests
chore: update dependencies
```

---

## 18. Documentation

### Generate API Documentation

```bash
pnpm run docs:api
```

### Generate Code Documentation

```bash
pnpm run docs:code
```

---

## 19. Performance Optimization

### Profile App

```bash
cd app
pnpm run profile
cd ..
```

### Profile Server

```bash
cd server
pnpm run profile
cd ..
```

---

## 20. Next Steps

After setup:

1. ✅ Read [DEVELOPMENT_ROADMAP.md](./DEVELOPMENT_ROADMAP.md)
2. ✅ Read [API_SPECIFICATION.md](./API_SPECIFICATION.md)
3. ✅ Read [CONTRIBUTING.md](./CONTRIBUTING.md)
4. ✅ Start building features
5. ✅ Run tests regularly
6. ✅ Keep documentation updated

---

## Support

For issues:

1. Check [Troubleshooting](#14-troubleshooting) section
2. Search [GitHub Issues](https://github.com/Justelliott6/SKYNZ/issues)
3. Create new issue with details
4. Join Discord for community support

---

## Conclusion

SKYNZ development environment is now ready. You can start building features, running tests, and deploying to production.

Happy coding! 🚀
