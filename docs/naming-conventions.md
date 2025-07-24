# HARALD Project Naming Conventions

**Created**: July 2025  
**Last Updated**: July 24, 2025  
**Version**: 1.0

This document defines the naming conventions for directories and files in the
HARALD project, with specific guidelines for Rust, Markdown, and JSON files.
Following these conventions ensures consistency, readability, and adheres to
best practices for each file type.

## Directory Structure

### General Directory Naming

- Use `kebab-case` for multi-word directory names (e.g., `vector-search`)
- Use singular form for categorical directories (e.g., `integration-guide`,
  `script`, `doc`)
- Use plural only when the directory contains multiple instances of the same
  type (e.g., `ai-entities`, `personality-archetypes`)
- Avoid abbreviations unless widely recognized (e.g., `aws` for Amazon Web
  Services)
- Prefer descriptive names that indicate purpose or content

### Top-Level Directories

```
/ai-entities        # AI entity definitions
/data               # Data storage and vector indices
/datasets           # Training and testing datasets
/docs               # Documentation files
/infrastructure     # Cloud and deployment configuration
/integration-guides # External service integration documentation
/personality-archetypes # Personality profiles and definitions
/scripts            # Utility and automation scripts
/workflows          # Workflow definitions and documentation
```

### Sub-Directory Organization

- Group related files in specialized subdirectories
- Limit directory depth to 3-4 levels where possible
- Use categorization subdirectories only when you have 5+ files in a category

## File Naming Conventions

### General Rules

1. File names should clearly indicate content and purpose
2. Avoid special characters except hyphens and underscores
3. Use consistent naming patterns within each category of files
4. Prefer explicit names over abbreviations

### Rust Files (.rs)

- Use `snake_case` for file names (e.g., `embed.rs`, `ingest.rs`)
- Match file names with the module names they define
- Use descriptive verbs or nouns that represent the module's functionality
- Main entry files follow standard Rust conventions (`main.rs`, `lib.rs`)

#### Example Structure

```
/rust_ingest/src/
  - main.rs      # Main entry point
  - lib.rs       # Library exports
  - embed.rs     # Embedding functionality
  - ingest.rs    # Data ingestion module
  - query.rs     # Query handling module
```

### Markdown Files (.md)

- Use `kebab-case` for technical documentation (e.g.,
  `character-based-chunking.md`)
- Use `TitleCase` for entity descriptions and persona-related files (e.g.,
  `Harald.md`, `Liora.md`)
- Standard documentation files use uppercase (e.g., `README.md`,
  `CONTRIBUTING.md`)
- Include a semantic prefix for categorization when helpful (e.g.,
  `hnsw-best-practices.md`)

#### Example Structure

```
/docs/vector-search/
  - character-based-chunking.md
  - ollama-embedding-limits.md
  - jsonl-ingestion.md

/ai-entities/
  - harald.md
  - liora.md
  - stratia.md
```

### JSON Files (.json)

- Use `kebab-case` for configuration and data files (e.g.,
  `vector-stores-registry.json`)
- Use `TitleCase` for entity definitions and personality archetypes (e.g.,
  `Heralds.json`, `VictorMancha.json`)
- Use `snake_case` for schema definition files (e.g., `entity_context.json`,
  `emotion_vectors.json`)
- Config files should include `config` in the name (e.g.,
  `embedding-config.json`)

#### Example Structure

```
/config/schemas/
  - entity-context.json
  - emotion-vectors.json
  - conversation-metadata.json

/personality-archetypes/
  - Heralds.json
  - MarvelAIs.json
```

## File Content Conventions

### Rust Files

1. **Module Documentation**: Begin each file with module documentation using
   `//!`

   ```rust
   //! Module name and brief description.
   //!
   //! More detailed explanation about the module's purpose.
   ```

2. **Struct/Function Naming**:
   - `PascalCase` for types, traits, and enums (e.g., `IngestConfig`)
   - `snake_case` for functions, methods, and variables
   - `SCREAMING_SNAKE_CASE` for constants

3. **Documentation Format**:
   - Document all public items with `///` comments
   - Include example usage for public API functions
   - Explain parameters and return values

### Markdown Files

1. **Structure**:
   - Begin with a clear H1 title using `#`
   - Include a brief overview paragraph
   - Use H2 `##` for major sections
   - Use H3 `###` for subsections
   - Maintain 80 character line width

2. **Formatting**:
   - Use backticks for code, commands, and file paths
   - Use code blocks with language specification
   - Separate paragraphs with a blank line
   - Use bullet points for lists, numbered points for sequences

### JSON Files

1. **Structure**:
   - Consistent indentation (2 spaces recommended)
   - Clear hierarchical organization
   - Logical property ordering
   - Descriptive property names

2. **Property Naming**:
   - Use `camelCase` for property names
   - Be descriptive but concise
   - Avoid abbreviations except for widely recognized ones
   - Use plural form for array properties

3. **Schema Files**:
   - Include a `$schema` property for JSON Schema files
   - Add `description` for each property
   - Specify `required` properties explicitly
   - Include `type` information for all properties

## Enforcement and Tools

- Use `rustfmt` to enforce Rust style conventions
- Use `markdownlint` to maintain Markdown formatting standards
- Use `prettier` for JSON formatting
- Incorporate file naming validation in CI/CD pipelines
- Use the JSON formatting scripts in `/scripts/json_tools/` for JSON validation

## Conversion Guide for Legacy Files

When updating existing files to match these conventions:

1. Create an issue in the tracking system
2. Update file names using git moves to preserve history
3. Update imports and references
4. Run appropriate formatters
5. Test to ensure functionality is preserved
6. Document changes in commit messages

## References

- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- [Google Markdown Style Guide](https://google.github.io/styleguide/docguide/style.html)
- [Google JSON Style Guide](https://google.github.io/styleguide/jsoncstyleguide.xml)
