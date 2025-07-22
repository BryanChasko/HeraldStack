# HARALD Configuration

This directory contains configuration files for the HARALD project.

## Files

- `default.json` - Default configuration settings
- `ethics/` - Ethical guidelines including Laws of Robotics
- `models/` - Model configuration files like Modelfile
- `schemas/` - JSON schemas for validation

## Best Practices

1. Keep sensitive information in environment variables, not in config files
2. Use kebab-case for configuration file names
3. Document all configuration options
4. Provide sensible defaults for all settings
5. Validate configuration against schemas on application startup
