# Test Fixtures

This directory contains test data files used for testing the HARALD system.

## Files

- `test_single_character.json` - Sample character data for testing ingest
  functionality
- `vision.jsonl` - Generated JSONL output for Vision character testing (created
  by `single_character_ingest.rs`)

## Usage

These fixtures are intended for unit and integration testing. They provide
controlled, consistent data for verifying that system components work as
expected.

When adding new test fixtures:

1. Use descriptive names that indicate the purpose of the fixture
2. Include sample data that exercises specific functionality
3. Document the structure and purpose of the fixture
4. Keep fixtures small and focused on specific test scenarios
