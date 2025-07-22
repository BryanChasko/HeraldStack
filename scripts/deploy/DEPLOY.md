# Deployment Scripts

This directory contains scripts for deployment and infrastructure management.

## Migration Status

**These scripts remain as shell scripts by design** - deployment and
infrastructure scripts are better served by shell for orchestrating external
tools and system integration.

## Scripts

- `deploy.sh` - Deploy the application to development/staging/production
- `backup.sh` - Backup data and configuration (TODO)
- `monitor.sh` - Monitor system health and performance (TODO)

## Usage

All scripts should be run from the project root directory:

```bash
./scripts/deploy/script_name.sh [arguments]
```

### Deploy Script

The deploy script builds Rust binaries and handles deployment to different
environments:

```bash
# Deploy to development (default)
./scripts/deploy/deploy.sh

# Deploy to specific environment
./scripts/deploy/deploy.sh staging
./scripts/deploy/deploy.sh prod

# Build only (useful for CI/CD)
./scripts/deploy/deploy.sh --build-only

# Skip tests during deployment
./scripts/deploy/deploy.sh prod --no-tests

# Get help
./scripts/deploy/deploy.sh --help
```

## Integration with Rust Migration

The deployment script automatically builds all Rust binaries as part of the
deployment process, ensuring that migrated scripts are properly compiled and
available for deployment.

For detailed usage of each script, run with the --help flag:

```bash
./scripts/deploy/script_name.sh --help
```
