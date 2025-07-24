# HARALD Configuration

This directory contains all configuration files for the HARALD project,
centralized per our development standards.

## Structure

- `.markdownlint.json` - Markdown linting configuration
- `vector-stores-registry.json` - Vector store configuration registry
- `ethics/` - Ethical guidelines including Laws of Robotics
- `models/` - Model configuration files (Ollama Modelfile, etc.)
- `schemas/` - JSON schemas for validation and data structure definitions

## Configuration Files

### Core Configuration

- `vector-stores-registry.json` - Registry of all vector store configurations
- `.markdownlint.json` - Markdown linting rules and formatting standards

### Ethics Configuration

- `ethics/LawsOfRobotics.json` - Asimov's Laws implementation for AI safety

### Model Configuration

- `models/Modelfile` - Ollama model configuration and parameters

### Schema Definitions

- `schemas/conversation-metadata.json` - Conversation data structure schema
- `schemas/emotion-vectors.json` - Emotion representation schema
- `schemas/entity-context.json` - Entity context data schema
- `schemas/narrative-arc.json` - Narrative structure schema

## Best Practices

1. **Centralization**: All configuration files belong in this directory
2. **Security**: Keep sensitive information in environment variables, not config
   files
3. **Naming**: Use kebab-case for configuration file names
4. **Documentation**: Document all configuration options and their purposes
5. **Defaults**: Provide sensible defaults for all settings
6. **Validation**: Validate configuration against schemas on application startup
7. **Version Control**: Configuration files should be version controlled for
   consistency
