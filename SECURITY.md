# SKYNZ Security Guidelines

**How to keep SKYNZ secure and protect sensitive information**

---

## 🔐 Overview

SKYNZ contains sensitive information including:
- Smart contract code (valuable IP)
- Wallet addresses and keys
- API credentials
- Business logic
- Deployment configurations

This guide ensures we protect all of it.

---

## 🚫 Never Commit These Files

### Secrets & Credentials

```bash
# Environment files
.env
.env.local
.env.*.local
.env.production
.env.development

# Private keys
*.key
*.pem
*.keystore
private_key.json
wallet.json
solana-keypair.json

# API keys
api_keys.txt
credentials.json
secrets.json

# Database
database.yml
db_config.json
```

### Configuration Files

```bash
# Sensitive configs
config/secrets.yml
config/database.yml
config/credentials.yml

# Build artifacts
dist/
build/
*.o
*.a
```

### IDE & OS Files

```bash
# IDE
.vscode/settings.json
.idea/
*.swp
*.swo

# OS
.DS_Store
Thumbs.db
```

---

## ✅ .gitignore Template

Add this to `.gitignore` in project root:

```bash
# Environment variables
.env
.env.local
.env.*.local
.env.production
.env.development
.env.test

# Private keys & credentials
*.key
*.pem
*.keystore
private_key.json
wallet.json
solana-keypair.json
api_keys.txt
credentials.json
secrets.json
.aws/

# Database
database.yml
db_config.json
*.db
*.sqlite

# IDE
.vscode/
.idea/
*.swp
*.swo
*.sublime-project
*.sublime-workspace

# OS
.DS_Store
Thumbs.db
.env.*.swp

# Build artifacts
dist/
build/
node_modules/
target/
*.o
*.a

# Logs
logs/
*.log
npm-debug.log*

# Temporary files
tmp/
temp/
*.tmp
```

---

## 🔑 Managing Secrets

### Local Development

1. **Create `.env.local`** (never commit)
   ```bash
   SOLANA_RPC_URL=https://api.devnet.solana.com
   SOLANA_WALLET_SECRET_KEY=your_secret_key_here
   STRIPE_SECRET_KEY=sk_test_xxx
   ```

2. **Load in code**
   ```javascript
   require('dotenv').config({ path: '.env.local' });
   const apiKey = process.env.STRIPE_SECRET_KEY;
   ```

3. **Verify `.gitignore`**
   ```bash
   git check-ignore .env.local  # Should return .env.local
   ```

### Production Deployment

Use environment variables from deployment platform:

**Vercel:**
- Settings → Environment Variables
- Add secrets there (not in code)

**AWS:**
- Secrets Manager
- Parameter Store

**GitHub Actions:**
- Settings → Secrets
- Reference as `${{ secrets.SECRET_NAME }}`

---

## 🔍 Pre-Commit Checklist

Before committing code:

```bash
# 1. Check what you're committing
git diff --cached

# 2. Look for secrets
git diff --cached | grep -i "key\|secret\|password\|token"

# 3. Verify .gitignore is working
git check-ignore .env.local
git check-ignore *.key

# 4. Review file list
git status

# 5. If something looks wrong, unstage it
git reset HEAD <file>

# 6. Then commit safely
git commit -m "Your message"
```

---

## 🛡️ Credential Rotation

### When to Rotate

- ✅ After accidental commit (even if deleted)
- ✅ Every 90 days (best practice)
- ✅ After team member leaves
- ✅ After security incident
- ✅ Before major deployment

### How to Rotate

**Solana Wallet:**
```bash
# Generate new keypair
solana-keygen new --force

# Update .env.local with new key
# Redeploy smart contracts if needed
```

**API Keys:**
```bash
# Stripe
stripe.com → Settings → API Keys → Regenerate

# Magic Eden
magiceden.io → Developers → Regenerate API Key

# AWS
aws.amazon.com → IAM → Users → Security Credentials → Create New Key
```

**Database:**
```bash
# Generate new password
openssl rand -base64 32

# Update connection string
# Restart services
```

---

## 🚨 If You Accidentally Commit a Secret

### Immediate Actions

1. **Stop! Don't push yet**
   ```bash
   # Check if already pushed
   git log --oneline | head -5
   ```

2. **If not pushed yet:**
   ```bash
   # Undo the commit (keep changes)
   git reset --soft HEAD~1
   
   # Remove the secret from staging
   git reset HEAD <file-with-secret>
   
   # Edit file to remove secret
   nano <file>
   
   # Re-commit without secret
   git commit -m "Fix: remove secret"
   ```

3. **If already pushed:**
   ```bash
   # Remove from history (nuclear option)
   git filter-branch --tree-filter 'rm -f <file>' HEAD
   
   # Force push (careful!)
   git push origin --force
   
   # ROTATE THE SECRET IMMEDIATELY
   ```

4. **Rotate the credential**
   - Generate new API key
   - Update all systems using it
   - Verify old key is disabled

---

## 🔐 Smart Contract Security

### Code Review Checklist

Before deploying smart contracts:

- [ ] No hardcoded private keys
- [ ] No test wallets in production code
- [ ] Royalty logic verified
- [ ] Ownership checks in place
- [ ] No obvious vulnerabilities
- [ ] Gas optimization reviewed
- [ ] Error handling complete
- [ ] Events properly emitted

### Deployment Checklist

- [ ] Test on devnet first
- [ ] Verify contract address
- [ ] Check creator wallet
- [ ] Confirm royalty percentage (1000 = 10%)
- [ ] Test minting on devnet
- [ ] Test utility redemption
- [ ] Test secondary market transfer
- [ ] Only then deploy to mainnet

---

## 📊 Audit Trail

### What GitHub Tracks

✅ All commits with author & timestamp  
✅ All file changes and diffs  
✅ All branch merges  
✅ All pull requests  
✅ All issue discussions  
✅ Repository access logs  

### Viewing Audit Trail

```bash
# See all commits
git log --oneline

# See specific file history
git log --oneline <file>

# See who changed what
git blame <file>

# See detailed changes
git show <commit-hash>
```

---

## 🚨 Incident Response

### If You Suspect a Breach

1. **Assess the damage**
   - What was exposed?
   - How long was it exposed?
   - Who had access?

2. **Contain the breach**
   - Rotate all affected credentials
   - Disable compromised API keys
   - Change wallet addresses if needed

3. **Notify stakeholders**
   - Email: security@skynz.io
   - Include: What, when, who, how

4. **Document & prevent**
   - Update security procedures
   - Add additional checks
   - Brief team on lessons learned

---

## 🔒 Access Control

### Repository Access

**Public Repository:**
- Anyone can view code
- Only collaborators can commit
- Pull requests reviewed before merge

**Adding Collaborators:**
```bash
# GitHub → Settings → Collaborators
# Add user with appropriate role:
# - Pull access: Read-only
# - Push access: Can commit
# - Admin access: Full control
```

### Principle of Least Privilege

- Developers: Only what they need
- Interns: Read-only access
- Contractors: Temporary, limited access
- Public: View-only (no secrets)

---

## 📋 Security Checklist

### Before Each Release

- [ ] No secrets in code
- [ ] .gitignore properly configured
- [ ] All credentials rotated
- [ ] Smart contracts audited
- [ ] Tests passing
- [ ] Documentation updated
- [ ] Security review completed
- [ ] Team briefed on changes

### Monthly

- [ ] Rotate API keys
- [ ] Review access logs
- [ ] Check for exposed secrets
- [ ] Update dependencies
- [ ] Audit smart contracts
- [ ] Review pull requests

### Quarterly

- [ ] Full security audit
- [ ] Penetration testing
- [ ] Code review
- [ ] Dependency updates
- [ ] Disaster recovery test

---

## 📞 Security Contact

**Found a security issue?**

- **Email:** security@skynz.io
- **Do NOT:** Post on GitHub publicly
- **Do:** Describe the issue privately
- **We will:** Respond within 24 hours

---

## 📚 Resources

- **OWASP Top 10:** https://owasp.org/www-project-top-ten/
- **GitHub Security:** https://docs.github.com/en/code-security
- **Solana Security:** https://docs.solana.com/security
- **Git Secrets:** https://github.com/awslabs/git-secrets

---

**Version:** 1.0.0  
**Last Updated:** June 11, 2026  
**Status:** Active

**Security is everyone's responsibility. Thank you for helping keep SKYNZ safe.** 🔐
