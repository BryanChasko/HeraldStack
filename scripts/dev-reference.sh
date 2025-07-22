#!/bin/bash
# dev-reference.sh - Quick developer reference for HeraldStack tools
#
# This script provides quick guidance on when to use which tools
#
# Usage: ./scripts/dev-reference.sh [topic]
#
# Topics: build, deploy, tools, migration

show_build_info() {
  echo "🦀 Building Rust Components"
  echo "============================"
  echo
  echo "Build all Rust binaries:"
  echo "  cd src && cargo build --release --features cli"
  echo
  echo "Available binaries:"
  echo "  format_json          - JSON formatting and validation"
  echo "  validate_json_schema - Schema validation and generation"
  echo "  ingest_chunked      - Character-based data ingestion"
  echo "  embedding_tool      - Embedding generation and testing"
  echo "  text_chunker        - Text processing utilities"
  echo
  echo "Run a specific tool:"
  echo "  ./src/target/release/format_json --help"
  echo "  ./src/target/release/validate_json_schema --help"
}

show_deploy_info() {
  echo "🚀 Deployment"
  echo "============="
  echo
  echo "Quick build (CI/CD):"
  echo "  ./scripts/deploy/deploy.sh --build-only"
  echo
  echo "Deploy to environments:"
  echo "  ./scripts/deploy/deploy.sh           # dev (default)"
  echo "  ./scripts/deploy/deploy.sh staging   # staging"
  echo "  ./scripts/deploy/deploy.sh prod      # production"
  echo
  echo "Skip tests (faster deployment):"
  echo "  ./scripts/deploy/deploy.sh prod --no-tests"
  echo
  echo "Why shell for deployment?"
  echo "  - Orchestrates external tools (Docker, AWS CLI)"
  echo "  - Rapid iteration needed"
  echo "  - Standard DevOps practice"
}

show_tools_info() {
  echo "🔧 Tool Selection Guide"
  echo "======================="
  echo
  echo "✅ Use Rust for:"
  echo "  - Data processing (JSON, text chunking)"
  echo "  - Application logic (embedding, validation)"
  echo "  - Performance-critical operations"
  echo "  - Type-safe operations"
  echo
  echo "✅ Use Shell for:"
  echo "  - Infrastructure operations"
  echo "  - Deployment orchestration"
  echo "  - System administration"
  echo "  - CI/CD pipeline steps"
  echo
  echo "Current Rust tools (src/utils/json_tools/):"
  echo "  - format_json.rs"
  echo "  - validate_json_schema.rs"
  echo
  echo "Current Shell tools (scripts/):"
  echo "  - deploy/deploy.sh"
}

show_migration_info() {
  echo "📋 Migration Status"
  echo "==================="
  echo
  echo "✅ Completed migrations to Rust:"
  echo "  - format_json.sh → format_json.rs"
  echo "  - validate_json_schema.sh → validate_json_schema.rs"
  echo "  - ingest_chunked.sh → ingest_chunked.rs"
  echo
  echo "🚫 Staying as shell scripts:"
  echo "  - deploy.sh (infrastructure orchestration)"
  echo
  echo "For detailed migration information:"
  echo "  See docs/migration/"
}

show_help() {
  echo "HeraldStack Developer Reference"
  echo "==============================="
  echo
  echo "Usage: $0 [topic]"
  echo
  echo "Topics:"
  echo "  build      - Building Rust components"
  echo "  deploy     - Deployment with shell scripts"
  echo "  tools      - Tool selection guide (Rust vs Shell)"
  echo "  migration  - Migration status overview"
  echo
  echo "Examples:"
  echo "  $0 build"
  echo "  $0 deploy"
}

main() {
  case "${1:-help}" in
    build)
      show_build_info
      ;;
    deploy)
      show_deploy_info
      ;;
    tools)
      show_tools_info
      ;;
    migration)
      show_migration_info
      ;;
    help|--help|-h)
      show_help
      ;;
    *)
      echo "Unknown topic: $1"
      echo
      show_help
      exit 1
      ;;
  esac
}

main "$@"
