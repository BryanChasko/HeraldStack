#!/bin/bash
# deploy.sh - Deploy the HARALD application
#
# This script handles deployment of the HARALD application to various environments
#
# Usage:
#   ./scripts/deploy/deploy.sh [environment] [options]
#
# Options:
#   environment    Target environment (dev, staging, prod)
#   --build-only   Only build the Rust binaries, don't deploy
#   --no-tests     Skip running tests before deployment
#   --help         Show this help message
#
# Author: Bryan Chasko
# Date: July 21, 2025

set -euo pipefail

# Configuration
ENV="dev"  # Default environment
BUILD_ONLY=false
RUN_TESTS=true

# Parse command line arguments
while [[ $# -gt 0 ]]; do
  case $1 in
    --build-only)
      BUILD_ONLY=true
      shift
      ;;
    --no-tests)
      RUN_TESTS=false
      shift
      ;;
    --help|-h)
      echo "Usage: $0 [environment] [options]"
      echo "Environments: dev, staging, prod"
      echo "Options:"
      echo "  --build-only   Only build the Rust binaries"
      echo "  --no-tests     Skip running tests"
      echo "  --help         Show this help"
      exit 0
      ;;
    dev|staging|prod)
      ENV=$1
      shift
      ;;
    *)
      echo "Unknown option: $1"
      exit 1
      ;;
  esac
done

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

log_info() {
  echo -e "${BLUE}[INFO]${NC} $1"
}

log_success() {
  echo -e "${GREEN}[SUCCESS]${NC} $1"
}

log_warning() {
  echo -e "${YELLOW}[WARN]${NC} $1"
}

log_error() {
  echo -e "${RED}[ERROR]${NC} $1"
  exit 1
}

check_environment() {
  case $ENV in
    dev|staging|prod)
      log_info "Deploying to $ENV environment"
      ;;
    *)
      log_error "Invalid environment: $ENV. Use dev, staging, or prod"
      ;;
  esac
}

build_rust_binaries() {
  log_info "Building Rust binaries..."
  cd "$PROJECT_ROOT/src"
  
  if cargo build --release --features cli; then
    log_success "Rust binaries built successfully"
  else
    log_error "Failed to build Rust binaries"
  fi
  
  cd "$PROJECT_ROOT"
}

run_tests() {
  if [ "$RUN_TESTS" = true ]; then
    log_info "Running tests..."
    cd "$PROJECT_ROOT/src"
    
    if cargo test; then
      log_success "All tests passed"
    else
      log_error "Tests failed"
    fi
    
    cd "$PROJECT_ROOT"
  else
    log_warning "Skipping tests (--no-tests flag used)"
  fi
}

deploy_to_environment() {
  case $ENV in
    dev)
      log_info "Deploying to development environment..."
      # Development deployment logic
      log_warning "Development deployment not fully implemented"
      ;;
    staging)
      log_info "Deploying to staging environment..."
      # Staging deployment logic
      log_warning "Staging deployment not fully implemented"
      ;;
    prod)
      log_info "Deploying to production environment..."
      # Production deployment logic
      log_warning "Production deployment not fully implemented"
      ;;
  esac
}

main() {
  log_info "Starting HARALD deployment process"
  echo "Project root: $PROJECT_ROOT"
  echo "Target environment: $ENV"
  echo
  
  check_environment
  
  # Always build the binaries
  build_rust_binaries
  
  if [ "$BUILD_ONLY" = true ]; then
    log_success "Build completed (--build-only flag used)"
    exit 0
  fi
  
  # Run tests unless skipped
  run_tests
  
  # Deploy to target environment
  deploy_to_environment
  
  log_success "Deployment completed successfully"
}

main
