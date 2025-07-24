# Development Principles & Migration History

**Created**: July 2025  
**Last Updated**: July 24, 2025  
**Version**: 2.0

This document outlines the canonical development principles and provides a brief
migration history for the HARALD project. For practical contribution guidance,
see [CONTRIBUTING.md](CONTRIBUTING.md).

---

## Part I: Development Principles

### 🚫 Core Rule: No New Shell Scripts

**We absolutely forbid creating new shell scripts for application logic.** This
is a fundamental architectural decision to ensure maintainability, type safety,
and better error handling.

#### ✅ Do This Instead

When you need to provide functionality or information:

1. **Update existing documentation** (README.md, relevant .md files)
2. **Add --help flags** to existing Rust tools
3. **Create/update markdown documentation** in appropriate directories
4. **Build functionality into existing Rust binaries**

#### 🚫 Never Do This

- Create new shell scripts for application logic
- Create "helper" or "reference" scripts
- Write shell scripts for data processing or business logic

### 🔧 Automation Over Manual Fixes

**Always use our automated tools first** before manually editing files:

```bash
# Fix JSON formatting and validation issues
./target/release/check_json --fix

# Fix Rust formatting, run clippy, and tests
./scripts/validation/check-rust.sh

# Fix Markdown formatting (line length, spacing, etc.)
./src/target/release/format_md

# Check and optionally fix naming convention problems
./src/target/release/validate_naming --fix --verbose
```

### 📚 Documentation Over Code

Prioritize well-maintained documentation over ad-hoc scripts. When encountering
an issue:

1. **First**: Check if our automated tools can solve it
2. **Next**: Consult or update documentation rather than creating a script
3. **If needed**: Extend our Rust tooling rather than writing a shell script

### 🦀 Rust vs Shell Decision Framework

**Migrate/Build in Rust if the functionality:**

- Contains complex application logic
- Processes data or performs calculations
- Benefits from type safety and error handling
- Is part of the core application functionality

**Keep/Create as shell if it:**

- Primarily orchestrates external tools (Docker, AWS CLI, git)
- Handles system/infrastructure concerns
- Manages CI/CD pipeline operations
- Performs simple file operations (backup, cleanup, monitoring)

#### What TO Migrate to Rust ✅

- Data processing scripts (text chunking, JSON formatting, validation)
- Application utilities (embedding tools, ingestion scripts)
- Business logic scripts (character processing, data transformation)
- Testing utilities that involve application logic

#### What NOT to Migrate to Rust ❌

- Deployment scripts (orchestrating external tools)
- Infrastructure scripts (system administration, environment setup)
- CI/CD pipeline scripts (git hooks, build orchestration)
- Scripts that primarily shell out to other tools
- Rust validation scripts (e.g., `check-rust.sh` must work even when Rust code
  has issues)

### 🎯 Key Principles Summary

- **Documentation over ad-hoc scripts** - Write clear documentation instead of
  creating new scripts
- **Rust over shell for application logic** - Use Rust's type safety and error
  handling for complex tasks
- **Automation over manual processes** - Invest in automated tooling that can be
  used by everyone
- **Reuse over recreation** - Extend existing tools rather than creating new
  ones
- **Testing over hoping** - Ensure all code has appropriate tests

---

## Part II: Migration History Summary

### Successfully Completed Migrations

The following shell scripts have been successfully migrated to Rust:

- ✅ `validate_naming.sh` → `src/target/release/validate_naming`
- ✅ `format-md.sh` → `src/target/release/format_md`
- ✅ `format_json.sh` → `src/target/release/format_json`
- ✅ `status.sh` → `src/target/release/status`
- ✅ `text_chunker.sh` → `src/target/release/text_chunker`
- ✅ `ingest_chunked.sh` → `src/target/release/ingest_chunked`
- ✅ `validate_json_schema.sh` → `src/target/release/validate_json_schema`
- ✅ `check-json.sh` → `src/target/release/check_json`

### Scripts That Intentionally Remain Shell Scripts

- **`check-rust.sh`** - Must work even when Rust code has issues
- **Deployment scripts** - Orchestrate external tools like AWS CLI
- **CI/CD pipeline scripts** - Build and deployment orchestration

### Migration Impact

This migration strategy has resulted in:

- **Improved maintainability** through type safety
- **Better error handling** with Rust's Result types
- **Consistent CLI interfaces** using the clap library
- **Reduced script proliferation** by consolidating functionality

### Documentation Archive

For complete historical details of the migration process, including step-by-step
plans, testing procedures, and decision rationales, see the migration archive:

📁 **[Migration Archive](migration/archive/)** - Contains detailed migration
plans and historical documents

---

## Quick Reference

**For Contributors**: See [CONTRIBUTING.md](CONTRIBUTING.md) for practical
guidance **For Migration Details**: See [migration/archive/](migration/archive/)
for complete historical documentation **For Tool Usage**: Run any tool with
`--help` for current options

Remember: **Documentation over shell scripts, automation over manual fixes, Rust
over shell for application logic.**
