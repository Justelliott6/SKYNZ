# SKYNZ Deployment Guide

Complete deployment instructions for SKYNZ to production.

---

## Deployment Environments

SKYNZ has three deployment environments:

| Environment | Purpose | URL |
|-------------|---------|-----|
| **Development** | Local development | http://localhost:3000 |
| **Staging** | Pre-production testing | https://staging-api.skynz.io |
| **Production** | Live platform | https://api.skynz.io |

---

## Prerequisites

Before deploying, ensure:

- AWS account with appropriate permissions
- GitHub repository access
- Solana mainnet SOL for contract deployment
- Stripe production keys
- Domain name configured

---

## Part 1: Infrastructure Setup

### AWS Setup

#### 1. Create AWS Account

1. Go to [aws.amazon.com](https://aws.amazon.com)
2. Click "Create AWS Account"
3. Follow setup wizard
4. Enable MFA for security

#### 2. Create IAM User

```bash
# Go to IAM Dashboard
# Create new user: skynz-deploy
# Attach policies:
# - AmazonEC2FullAccess
# - AmazonRDSFullAccess
# - AmazonS3FullAccess
# - AWSLambdaFullAccess
```

#### 3. Create Access Keys

```bash
# Generate access keys
# Save in secure location
# Add to GitHub Secrets
```

#### 4. Create VPC

```bash
# Create VPC: skynz-vpc
# Create subnets:
# - Public subnet (for load balancer)
# - Private subnet (for database)
# - Private subnet (for app)
```

#### 5. Create Security Groups

```bash
# App security group:
# - Allow inbound: 3000 (app), 22 (SSH)
# - Allow outbound: all

# Database security group:
# - Allow inbound: 5432 (PostgreSQL)
# - Allow outbound: all
```

### Database Setup

#### 1. Create RDS Instance

```bash
# Create PostgreSQL RDS instance
# - Engine: PostgreSQL 14+
# - Instance class: db.t3.micro (for dev), db.t3.small (for prod)
# - Storage: 20 GB (gp2)
# - Multi-AZ: Yes (production)
# - Backup retention: 30 days
# - Encryption: Enabled
```

#### 2. Create Database

```bash
# Connect to RDS
psql -h <RDS_ENDPOINT> -U postgres

# Create database
CREATE DATABASE skynz_prod;

# Create user
CREATE USER skynz_app WITH PASSWORD 'STRONG_PASSWORD';

# Grant privileges
GRANT ALL PRIVILEGES ON DATABASE skynz_prod TO skynz_app;
```

#### 3. Run Migrations

```bash
# Set environment
export DATABASE_URL=postgresql://skynz_app:PASSWORD@RDS_ENDPOINT:5432/skynz_prod

# Run migrations
pnpm run db:push
```

### Storage Setup

#### 1. Create S3 Bucket

```bash
# Create bucket: skynz-prod-storage
# Block public access: Yes
# Enable versioning: Yes
# Enable encryption: Yes
```

#### 2. Create IAM Policy for S3

```json
{
  "Version": "2012-10-17",
  "Statement": [
    {
      "Effect": "Allow",
      "Action": [
        "s3:GetObject",
        "s3:PutObject",
        "s3:DeleteObject"
      ],
      "Resource": "arn:aws:s3:::skynz-prod-storage/*"
    }
  ]
}
```

---

## Part 2: Application Deployment

### Option A: AWS EC2 + Docker

#### 1. Create EC2 Instance

```bash
# Launch EC2 instance
# - AMI: Ubuntu 22.04 LTS
# - Instance type: t3.medium (production)
# - Security group: app-sg
# - Key pair: skynz-prod-key
```

#### 2. Install Docker

```bash
# SSH into instance
ssh -i skynz-prod-key.pem ubuntu@<INSTANCE_IP>

# Install Docker
curl -fsSL https://get.docker.com -o get-docker.sh
sudo sh get-docker.sh

# Add user to docker group
sudo usermod -aG docker ubuntu
```

#### 3. Create Dockerfile

```dockerfile
FROM node:18-alpine

WORKDIR /app

# Copy package files
COPY package.json pnpm-lock.yaml ./

# Install dependencies
RUN npm install -g pnpm && pnpm install --frozen-lockfile

# Copy source code
COPY . .

# Build
RUN pnpm run build

# Expose port
EXPOSE 3000

# Start server
CMD ["pnpm", "start"]
```

#### 4. Create Docker Compose

```yaml
version: '3.8'

services:
  app:
    build: .
    ports:
      - "3000:3000"
    environment:
      - NODE_ENV=production
      - DATABASE_URL=${DATABASE_URL}
      - STRIPE_SECRET_KEY=${STRIPE_SECRET_KEY}
      - JWT_SECRET=${JWT_SECRET}
    depends_on:
      - db
    restart: always

  db:
    image: postgres:14
    environment:
      - POSTGRES_DB=skynz_prod
      - POSTGRES_USER=skynz_app
      - POSTGRES_PASSWORD=${DB_PASSWORD}
    volumes:
      - postgres_data:/var/lib/postgresql/data
    restart: always

volumes:
  postgres_data:
```

#### 5. Deploy

```bash
# SSH into instance
ssh -i skynz-prod-key.pem ubuntu@<INSTANCE_IP>

# Clone repository
git clone https://github.com/Justelliott6/SKYNZ.git
cd SKYNZ

# Create .env file
cat > .env << EOF
NODE_ENV=production
DATABASE_URL=postgresql://skynz_app:PASSWORD@RDS_ENDPOINT:5432/skynz_prod
STRIPE_SECRET_KEY=sk_live_...
JWT_SECRET=your_secret
EOF

# Start with Docker Compose
docker-compose up -d

# Check logs
docker-compose logs -f app
```

### Option B: AWS Elastic Beanstalk

#### 1. Initialize Elastic Beanstalk

```bash
# Install EB CLI
pip install awsebcli

# Initialize
eb init -p node.js-18 skynz --region us-east-1
```

#### 2. Create Environment

```bash
# Create environment
eb create skynz-prod --instance-type t3.medium

# Configure environment
eb setenv NODE_ENV=production DATABASE_URL=... STRIPE_SECRET_KEY=...
```

#### 3. Deploy

```bash
# Deploy
eb deploy

# Check status
eb status

# View logs
eb logs
```

### Option C: Vercel (Recommended for Web)

#### 1. Connect Repository

1. Go to [vercel.com](https://vercel.com)
2. Click "Import Project"
3. Select GitHub repository
4. Configure project

#### 2. Set Environment Variables

```bash
# In Vercel dashboard:
# - DATABASE_URL
# - STRIPE_SECRET_KEY
# - JWT_SECRET
# - SOLANA_RPC_URL
```

#### 3. Deploy

```bash
# Deploy from Vercel dashboard or:
vercel deploy --prod
```

---

## Part 3: Smart Contract Deployment

### Deploy to Solana Mainnet

#### 1. Prepare for Mainnet

```bash
# Set to mainnet
solana config set --url mainnet-beta

# Check balance (need SOL for deployment)
solana balance
```

#### 2. Build for Mainnet

```bash
cd contracts

# Build release
cargo build-bpf --release

# Verify build
ls target/deploy/skynz_certificate.so
```

#### 3. Deploy Program

```bash
# Deploy certificate program
solana program deploy target/deploy/skynz_certificate.so

# Deploy badge program
solana program deploy target/deploy/skynz_badge.so

# Get program IDs
solana address -k target/deploy/skynz_certificate-keypair.json
solana address -k target/deploy/skynz_badge-keypair.json
```

#### 4. Update Configuration

```bash
# Update environment variables
export CERTIFICATE_PROGRAM_ID=<PROGRAM_ID>
export BADGE_PROGRAM_ID=<PROGRAM_ID>
```

---

## Part 4: Domain & SSL

### Configure Domain

#### 1. Point Domain to Server

1. Go to domain registrar
2. Update DNS records:
   ```
   A record: api.skynz.io → <EC2_IP>
   A record: skynz.io → <VERCEL_IP>
   ```

#### 2. Create SSL Certificate

```bash
# SSH into instance
ssh -i skynz-prod-key.pem ubuntu@<INSTANCE_IP>

# Install Certbot
sudo apt-get install certbot python3-certbot-nginx

# Get certificate
sudo certbot certonly --standalone -d api.skynz.io

# Auto-renew
sudo certbot renew --dry-run
```

#### 3. Configure HTTPS

```bash
# Update nginx config
sudo nano /etc/nginx/sites-available/default

# Add SSL configuration
server {
    listen 443 ssl;
    server_name api.skynz.io;
    
    ssl_certificate /etc/letsencrypt/live/api.skynz.io/fullchain.pem;
    ssl_certificate_key /etc/letsencrypt/live/api.skynz.io/privkey.pem;
    
    location / {
        proxy_pass http://localhost:3000;
    }
}

# Restart nginx
sudo systemctl restart nginx
```

---

## Part 5: Monitoring & Logging

### CloudWatch Setup

#### 1. Enable CloudWatch Logs

```bash
# In AWS Console:
# - Create log group: /aws/skynz/app
# - Create log stream: production
```

#### 2. Configure App Logging

```typescript
// In server code
import * as winston from 'winston';

const logger = winston.createLogger({
  transports: [
    new winston.transports.Console(),
    new winston.transports.File({ filename: 'error.log', level: 'error' }),
    new winston.transports.File({ filename: 'combined.log' })
  ]
});

export default logger;
```

#### 3. Set Up Alarms

```bash
# High CPU usage
aws cloudwatch put-metric-alarm \
  --alarm-name skynz-high-cpu \
  --alarm-description "Alert when CPU exceeds 80%" \
  --metric-name CPUUtilization \
  --namespace AWS/EC2 \
  --statistic Average \
  --period 300 \
  --threshold 80 \
  --comparison-operator GreaterThanThreshold
```

### Error Tracking

#### 1. Set Up Sentry

```bash
# Install Sentry
npm install @sentry/node

# Initialize in app
import * as Sentry from "@sentry/node";

Sentry.init({
  dsn: process.env.SENTRY_DSN,
  environment: process.env.NODE_ENV,
});
```

#### 2. Configure Alerts

1. Go to [sentry.io](https://sentry.io)
2. Create project
3. Set up alerts for errors
4. Configure notifications

---

## Part 6: CI/CD Pipeline

### GitHub Actions

#### 1. Create Workflow

Create `.github/workflows/deploy.yml`:

```yaml
name: Deploy to Production

on:
  push:
    branches: [main]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2
      - uses: actions/setup-node@v2
        with:
          node-version: '18'
      - run: npm install -g pnpm
      - run: pnpm install
      - run: pnpm run test
      - run: pnpm run lint

  deploy:
    needs: test
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2
      - name: Deploy to AWS
        run: |
          # Deploy script
          eb deploy skynz-prod
        env:
          AWS_ACCESS_KEY_ID: ${{ secrets.AWS_ACCESS_KEY_ID }}
          AWS_SECRET_ACCESS_KEY: ${{ secrets.AWS_SECRET_ACCESS_KEY }}
```

#### 2. Add Secrets

In GitHub repository settings:
- `AWS_ACCESS_KEY_ID`
- `AWS_SECRET_ACCESS_KEY`
- `DATABASE_URL`
- `STRIPE_SECRET_KEY`
- `JWT_SECRET`

---

## Part 7: Backup & Recovery

### Database Backups

#### 1. Automated Backups

```bash
# Enable RDS automated backups
aws rds modify-db-instance \
  --db-instance-identifier skynz-prod \
  --backup-retention-period 30 \
  --preferred-backup-window "03:00-04:00"
```

#### 2. Manual Backup

```bash
# Create snapshot
aws rds create-db-snapshot \
  --db-instance-identifier skynz-prod \
  --db-snapshot-identifier skynz-prod-backup-$(date +%Y%m%d)
```

#### 3. Restore from Backup

```bash
# Restore from snapshot
aws rds restore-db-instance-from-db-snapshot \
  --db-instance-identifier skynz-prod-restored \
  --db-snapshot-identifier skynz-prod-backup-20260608
```

### Application Backups

```bash
# Backup S3 bucket
aws s3 sync s3://skynz-prod-storage s3://skynz-prod-backup-$(date +%Y%m%d)
```

---

## Part 8: Scaling

### Horizontal Scaling

#### 1. Load Balancer

```bash
# Create Application Load Balancer
aws elbv2 create-load-balancer \
  --name skynz-alb \
  --subnets subnet-xxx subnet-yyy \
  --security-groups sg-xxx
```

#### 2. Auto Scaling Group

```bash
# Create launch template
aws ec2 create-launch-template \
  --launch-template-name skynz-template \
  --launch-template-data '{"ImageId":"ami-xxx","InstanceType":"t3.medium"}'

# Create auto scaling group
aws autoscaling create-auto-scaling-group \
  --auto-scaling-group-name skynz-asg \
  --launch-template LaunchTemplateName=skynz-template \
  --min-size 2 \
  --max-size 10 \
  --desired-capacity 3
```

### Vertical Scaling

```bash
# Upgrade instance type
aws ec2 modify-instance-attribute \
  --instance-id i-xxx \
  --instance-type '{"Value": "t3.large"}'
```

---

## Part 9: Security

### SSL/TLS

```bash
# Enable HSTS
add_header Strict-Transport-Security "max-age=31536000; includeSubDomains" always;
```

### Database Security

```bash
# Enable encryption at rest
aws rds modify-db-instance \
  --db-instance-identifier skynz-prod \
  --storage-encrypted
```

### API Security

```typescript
// Rate limiting
import rateLimit from 'express-rate-limit';

const limiter = rateLimit({
  windowMs: 15 * 60 * 1000, // 15 minutes
  max: 100 // limit each IP to 100 requests per windowMs
});

app.use('/api/', limiter);
```

---

## Part 10: Post-Deployment

### Verification Checklist

- [ ] API responding on production domain
- [ ] Database connected and migrated
- [ ] SSL certificate valid
- [ ] Monitoring and logging active
- [ ] Backups scheduled
- [ ] Alerts configured
- [ ] Load balancer working
- [ ] Auto-scaling configured
- [ ] DNS records correct
- [ ] Email notifications working

### Performance Testing

```bash
# Load test
ab -n 1000 -c 100 https://api.skynz.io/health

# Monitor response times
curl -w "@curl-format.txt" -o /dev/null -s https://api.skynz.io/health
```

---

## Troubleshooting

### Issue: Deployment Failed

**Solution:**
1. Check GitHub Actions logs
2. Verify environment variables
3. Check database connectivity
4. Review application logs

### Issue: High Latency

**Solution:**
1. Check CloudWatch metrics
2. Optimize database queries
3. Enable caching
4. Scale up resources

### Issue: Database Connection Error

**Solution:**
1. Verify security group rules
2. Check database credentials
3. Verify network connectivity
4. Check RDS status

---

## Conclusion

SKYNZ is now deployed to production. Monitor performance, maintain backups, and scale as needed.

For support, see [CONTRIBUTING.md](./CONTRIBUTING.md).
