# GitHub Setup & Workflow

This document outlines how the HeraldStack project uses GitHub for development,
issue tracking, and project management.

<!-- filepath: docs/GITHUB.md -->

## Labels System

We use a color-blind friendly labeling system to categorize
issues and pull requests:

### Core Technical Areas

| Label | Description | Color |
|-------|-------------|-------|
| `rust` | Rust codebase implementation | #0052CC |
| `ingest` | File ingestion and indexing pipeline | #006644 |
| `query` | Search and retrieval functionality | #5319E7 |
| `embed` | Vector embedding generation and processing | #E05D44 |
| `memory` | Memory storage and retrieval systems | #F9A03F |

### Issue Types

| Label | Description | Color |
|-------|-------------|-------|
| `bug` | Functionality issues requiring fixes | #D93F0B |
| `enhancement` | New features and improvements | #0E8A16 |
| `refactor` | Code restructuring without behavior change | #1D76DB |
| `documentation` | Documentation updates and improvements | #FFC01F |
| `testing` | Test coverage and infrastructure | #8250DF |

### Priority & Status

| Label | Description | Color |
|-------|-------------|-------|
| `critical` | Requires immediate attention | #B60205 |
| `high` | High priority for current sprint | #D93F0B |
| `medium` | Standard priority task | #FBCA04 |
| `low` | Nice to have, not time-sensitive | #C5DEF5 |
| `in-progress` | Actively being worked on | #0E8A16 |
| `blocked` | Waiting on dependencies or decisions | #D876E3 |

### Architecture Components

| Label | Description | Color |
|-------|-------------|-------|
| `entity-system` | Entity framework and routing | #6F42C1 (Purple) |
| `vector-store` | Pinecone and vector storage integration | #1A73E8 (Google Blue) |
| `infrastructure` | AWS and deployment infrastructure | #FF6D00 (Dark Orange) |
| `cli` | Command-line interface | #795548 (Brown) |
| `security` | Security and authentication concerns | #EE0701 (Bright Red) |

### Housekeeping

| Label | Description | Color |
|-------|-------------|-------|
| `duplicate` | Issue already exists elsewhere | #CCCCCC (Light Grey) |
| `invalid` | Issue doesn't apply or is incorrect | #E4E669 (Pale Yellow) |
| `question` | Requires clarification or discussion | #D876E3 (Pink) |
| `wontfix` | Decision made not to fix or implement | #FBBF24 (Gold) |

## Branch Strategy

We follow a simplified GitHub flow:

1. `main` branch is always deployable
2. Feature branches named `feature/<description>` branch off from `main`
3. Bug fix branches named `fix/<issue-number>-<description>` branch off from `main`
4. Pull requests merge back to `main` after review

## Pull Request Process

1. Create a branch for your changes
2. Make your changes with descriptive commits
3. Open a pull request with:

- Clear description of changes
- Reference to related issues
- Screenshots if UI changes are involved

4. Request review from appropriate team members
5. Address any review comments
6. Merge when approved (squash commits)

## Issue Templates

We use issue templates for common types:

- Bug reports
- Feature requests
- Documentation updates

## Project Boards

Our development is organized into project boards:

- **Rust Migration MVP**: Core functionality migration from Python to Rust
- **Entity Framework**: Development of the entity system
- **Infrastructure**: AWS and deployment configuration

## Automation

We use GitHub Actions for:

- Continuous Integration testing
- Documentation generation
- Weekly status reports

## Using GitHub CLI

Common commands for working with our repository:

```bash
# Create a new issue
gh issue create --title "Issue title" --body "Description" --label "rust,bug"

# Check out a PR
gh pr checkout 123

# Create a PR
gh pr create --title "PR title" --body "Description" --label "enhancement"

# Apply labels
gh issue edit 123 --add-label "high,in-progress"

# View project status
gh project view "Rust Migration MVP"
```

## Weekly Reviews

Every Monday, we conduct a GitHub review:

1. Triage new issues
2. Update priority labels
3. Close completed items
4. Prioritize, define, and plan upcoming work
