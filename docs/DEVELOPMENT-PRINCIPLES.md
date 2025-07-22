# HARALD Development Principles

This document outlines the core development philosophy and architectural
decisions for the HARALD project. For practical contribution guidance, see
[CONTRIBUTING.md](CONTRIBUTING.md).

## 🚫 Absolutely no new shell scripts

### We give preferential treatment to writing code in Rust in this project

Creating new shell scripts for application logic where Rust can perform equally
is counterproductive.
technical strategy.

### ✅ Do Instead

1. **Update existing documentation** (README.md, relevant .md files)
2. **Add --help flags** to existing Rust tools
3. **Create/update markdown documentation** in appropriate directories
4. **Build functionality into existing Rust binaries**

## � Focus on Documentation and Automation

We prioritize well-maintained documentation and automated tooling over ad-hoc
scripts. When encountering an issue:

1. First, check if our **automated tools** can solve it
   ([see tools in CONTRIBUTING.md](CONTRIBUTING.md#-automated-cleanup-tools-first))
2. Next, consult or update **documentation** rather than creating a script
3. If needed, extend our **Rust tooling** rather than writing a shell script

## Migration Strategy & Guidelines

We follow a structured approach to migrating functionality from shell scripts to
Rust. This ensures maintainability, type safety, and better error handling.

### What TO Migrate to Rust ✅

- Data processing scripts (text chunking, JSON formatting, validation)
- Application utilities (embedding tools, ingestion scripts)
- Business logic scripts (character processing, data transformation)
- Testing utilities that involve application logic

### What NOT to Migrate to Rust ❌

- Deployment scripts (orchestrating external tools like Docker, AWS CLI)
- Infrastructure scripts (system administration, environment setup)
- CI/CD pipeline scripts (git hooks, build orchestration)
- Simple file operations (backup, cleanup, monitoring)
- Scripts that primarily shell out to other tools

## Decision Framework

When evaluating whether to migrate a script or build new functionality:

**Migrate/Build in Rust if the script:**

- Contains complex application logic
- Processes data or performs calculations
- Benefits from type safety and error handling
- Is part of the core application functionality

**Keep/Create as shell script if it:**

- Primarily orchestrates external tools
- Handles system/infrastructure concerns
- Needs rapid iteration and deployment
- Is better served by shell's ecosystem integration

## Key Resources

- [Full Migration Guidelines](migration/SCRIPT-CLEANUP-PLAN.md)
- [Contributing Guidelines](CONTRIBUTING.md)
- [Script Migration Status](migration/SCRIPT-MIGRATION.md)
- [Build & Deploy Guide](../scripts/deploy/DEPLOY.md)

## Ideas Summary

 **Documentation over ad-hoc scripts** - Write clear documentation instead of
   creating new scripts
 **Rust over shell for application logic** - Use Rust's type safety and error
   handling for complex tasks
 **Automation over manual processes** - Invest in automated tooling that can
   be used by everyone
 **Reuse over recreation** - Extend existing tools rather than creating new
   ones
 **Testing over hoping** - Ensure all code has appropriate tests

Remember: **Documentation over shell scripts, automation over manual fixes.**
