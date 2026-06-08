# SKYNZ Contributing Guide

How to contribute to SKYNZ development.

---

## Code of Conduct

We are committed to providing a welcoming and inclusive environment for all contributors. Please read and follow our Code of Conduct.

**Be respectful, inclusive, and professional.**

---

## Getting Started

### 1. Fork Repository

```bash
# Fork on GitHub
# https://github.com/Justelliott6/SKYNZ

# Clone your fork
git clone https://github.com/YOUR_USERNAME/SKYNZ.git
cd SKYNZ

# Add upstream remote
git remote add upstream https://github.com/Justelliott6/SKYNZ.git
```

### 2. Set Up Development Environment

Follow [SETUP_GUIDE.md](./SETUP_GUIDE.md) to set up your local environment.

### 3. Create Feature Branch

```bash
# Update main branch
git fetch upstream
git checkout main
git merge upstream/main

# Create feature branch
git checkout -b feature/my-feature
```

---

## Development Workflow

### 1. Make Changes

Edit files in your feature branch.

### 2. Run Tests

```bash
# Run all tests
pnpm run test

# Run specific test
pnpm run test -- auth.test.ts

# Run with coverage
pnpm run test:coverage
```

### 3. Run Linter

```bash
# Check for issues
pnpm run lint

# Fix issues automatically
pnpm run lint:fix
```

### 4. Format Code

```bash
# Format code
pnpm run format
```

### 5. Commit Changes

Follow Conventional Commits format:

```bash
# Good commits
git commit -m "feat: add live chat functionality"
git commit -m "fix: resolve database connection issue"
git commit -m "docs: update API documentation"
git commit -m "test: add tests for badge minting"

# Bad commits
git commit -m "update stuff"
git commit -m "fix bug"
git commit -m "changes"
```

**Commit Types:**
- `feat:` — New feature
- `fix:` — Bug fix
- `docs:` — Documentation
- `style:` — Code style (formatting, missing semicolons)
- `refactor:` — Code refactoring
- `test:` — Adding tests
- `chore:` — Dependency updates, build changes

### 6. Push Changes

```bash
# Push to your fork
git push origin feature/my-feature
```

### 7. Create Pull Request

1. Go to GitHub
2. Click "Compare & pull request"
3. Fill in PR description
4. Submit PR

---

## Pull Request Guidelines

### PR Title

Use Conventional Commits format:

```
feat: add live chat to creator pages
fix: resolve Solana transaction timeout
docs: update API documentation
```

### PR Description

Include:

```markdown
## Description
Brief description of changes

## Type of Change
- [ ] New feature
- [ ] Bug fix
- [ ] Documentation
- [ ] Performance improvement
- [ ] Breaking change

## Related Issues
Fixes #123

## Testing
- [ ] Unit tests added
- [ ] Integration tests added
- [ ] Manual testing completed

## Checklist
- [ ] Code follows style guidelines
- [ ] Tests pass
- [ ] Documentation updated
- [ ] No console errors
- [ ] No breaking changes
```

### PR Review

- Respond to feedback promptly
- Make requested changes
- Push updates to same branch
- Request re-review when ready

---

## Coding Standards

### TypeScript

```typescript
// ✅ Good
interface User {
  id: string;
  email: string;
  username: string;
}

function getUser(id: string): Promise<User> {
  // Implementation
}

// ❌ Bad
interface User {
  id,
  email,
  username
}

function getUser(id) {
  // Implementation
}
```

### React Components

```typescript
// ✅ Good
interface ChatMessageProps {
  message: string;
  sender: string;
  timestamp: Date;
}

export function ChatMessage({ message, sender, timestamp }: ChatMessageProps) {
  return (
    <View className="p-4">
      <Text className="font-bold">{sender}</Text>
      <Text>{message}</Text>
      <Text className="text-sm text-muted">{timestamp.toLocaleTimeString()}</Text>
    </View>
  );
}

// ❌ Bad
export function ChatMessage(props) {
  return (
    <View>
      <Text>{props.sender}</Text>
      <Text>{props.message}</Text>
    </View>
  );
}
```

### Error Handling

```typescript
// ✅ Good
try {
  const result = await fetchData();
  return result;
} catch (error) {
  logger.error('Failed to fetch data', { error });
  throw new Error('Data fetch failed');
}

// ❌ Bad
try {
  const result = await fetchData();
  return result;
} catch (error) {
  console.log(error);
}
```

### Testing

```typescript
// ✅ Good
describe('User Authentication', () => {
  it('should login with valid credentials', async () => {
    const user = await login('test@example.com', 'password');
    expect(user.email).toBe('test@example.com');
  });

  it('should reject invalid credentials', async () => {
    await expect(login('test@example.com', 'wrong')).rejects.toThrow();
  });
});

// ❌ Bad
describe('User', () => {
  it('works', async () => {
    const result = await login('test@example.com', 'password');
    expect(result).toBeDefined();
  });
});
```

---

## File Naming

| Type | Format | Example |
|------|--------|---------|
| Components | PascalCase | `ChatMessage.tsx` |
| Utilities | camelCase | `formatDate.ts` |
| Constants | UPPER_SNAKE_CASE | `API_BASE_URL.ts` |
| Tests | `.test.ts` or `.spec.ts` | `auth.test.ts` |
| Directories | kebab-case | `chat-messages/` |

---

## Documentation

### Code Comments

```typescript
// ✅ Good
/**
 * Mints an NFT certificate for a creator page
 * @param tier - Page tier (basic, pro, supermodel)
 * @param serialNumber - Serial number of the certificate
 * @returns Promise<string> - NFT token ID
 */
async function mintCertificate(tier: string, serialNumber: number): Promise<string> {
  // Implementation
}

// ❌ Bad
// mint certificate
function mint(t, s) {
  // do stuff
}
```

### README Updates

When adding features, update relevant documentation:
- `README.md` — Feature overview
- `API_SPECIFICATION.md` — API endpoints
- `SETUP_GUIDE.md` — Setup instructions
- Inline code comments

---

## Testing Requirements

### Unit Tests

```bash
# All functions must have unit tests
# Minimum 80% code coverage
pnpm run test:coverage
```

### Integration Tests

```bash
# Test API endpoints
# Test database operations
# Test external integrations
pnpm run test:integration
```

### E2E Tests

```bash
# Test complete user flows
# Test critical paths
pnpm run test:e2e
```

---

## Performance Guidelines

### Bundle Size

- Keep bundle size under 500KB
- Lazy load large components
- Tree-shake unused code

### Database Queries

- Use indexes for common queries
- Limit result sets with pagination
- Use prepared statements

### API Responses

- Keep response time under 200ms
- Implement caching where appropriate
- Use compression for large payloads

---

## Security Guidelines

### Never Commit

- ❌ API keys or secrets
- ❌ Database credentials
- ❌ Private keys
- ❌ Sensitive data

### Always Use

- ✅ Environment variables for secrets
- ✅ HTTPS for all connections
- ✅ Input validation
- ✅ SQL parameterized queries

### Code Review

All code must be reviewed before merging. Reviewers check for:
- Security vulnerabilities
- Performance issues
- Code quality
- Test coverage

---

## Issue Reporting

### Bug Report

```markdown
## Description
Brief description of the bug

## Steps to Reproduce
1. Step 1
2. Step 2
3. Step 3

## Expected Behavior
What should happen

## Actual Behavior
What actually happens

## Environment
- OS: macOS 13.0
- Node: 18.0.0
- Browser: Chrome 120.0

## Screenshots
[If applicable]
```

### Feature Request

```markdown
## Description
Brief description of the feature

## Use Case
Why is this feature needed?

## Proposed Solution
How should it work?

## Alternatives
Any alternative approaches?
```

---

## Review Process

### For Maintainers

1. **Code Review** — Check code quality, tests, documentation
2. **Testing** — Run tests locally
3. **Security** — Check for vulnerabilities
4. **Approval** — Approve or request changes
5. **Merge** — Merge to main branch

### For Contributors

1. **Submit PR** — Create pull request
2. **Respond to Feedback** — Address review comments
3. **Update Code** — Make requested changes
4. **Re-request Review** — Ask for re-review
5. **Celebrate** — PR merged! 🎉

---

## Release Process

### Version Numbering

Follow Semantic Versioning: `MAJOR.MINOR.PATCH`

- `MAJOR` — Breaking changes
- `MINOR` — New features
- `PATCH` — Bug fixes

### Release Steps

1. Update version in `package.json`
2. Update `CHANGELOG.md`
3. Create git tag: `v1.0.0`
4. Push to GitHub
5. Create GitHub release
6. Deploy to production

---

## Communication

### Slack Channel

Join our Slack for discussions:
- `#general` — General discussion
- `#development` — Development topics
- `#bugs` — Bug reports
- `#features` — Feature requests

### Discord Server

Join our Discord community:
- `#announcements` — Important updates
- `#general` — General chat
- `#help` — Getting help
- `#showcase` — Show your work

### GitHub Discussions

Use GitHub Discussions for:
- Questions
- Ideas
- Polls
- Announcements

---

## Contributor Recognition

We recognize contributors in:
- `CONTRIBUTORS.md` — List of all contributors
- GitHub insights — Contribution graph
- Release notes — Contributors mentioned

---

## Resources

- [SETUP_GUIDE.md](./SETUP_GUIDE.md) — Development setup
- [DEVELOPMENT_ROADMAP.md](./DEVELOPMENT_ROADMAP.md) — Development timeline
- [API_SPECIFICATION.md](./API_SPECIFICATION.md) — API documentation
- [TECH_STACK.md](./TECH_STACK.md) — Technology stack
- [Conventional Commits](https://www.conventionalcommits.org/) — Commit format

---

## FAQ

### Q: How do I get started?

A: Follow [SETUP_GUIDE.md](./SETUP_GUIDE.md) to set up your development environment.

### Q: Can I work on any issue?

A: Yes! Look for issues labeled `good first issue` or `help wanted`. Comment on the issue to claim it.

### Q: How long does review take?

A: Usually 24-48 hours. Complex changes may take longer.

### Q: What if my PR is rejected?

A: Don't worry! We'll provide feedback on how to improve it. You can make changes and resubmit.

### Q: Can I contribute documentation?

A: Absolutely! Documentation is as important as code. Submit PRs for documentation improvements.

---

## Support

Need help?

- Check [SETUP_GUIDE.md](./SETUP_GUIDE.md) for common issues
- Ask in GitHub Discussions
- Join our Discord community
- Email: support@skynz.io

---

## Thank You

Thank you for contributing to SKYNZ! Your efforts help make this platform better for creators everywhere.

**Happy coding!** 🚀
