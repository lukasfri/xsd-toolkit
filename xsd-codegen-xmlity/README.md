# XSD Codegen XMLity

Code generation engine that produces Rust code from XSD schemas using the XMLity serialization framework. This crate transforms XSD type definitions into strongly-typed Rust structures suitable for XML serialization and deserialization.

## Features

- Generate Rust structs and enums from XSD complex and simple types
- Support for XML Schema features including inheritance, restrictions, and extensions
- Integration with XMLity for XML serialization/deserialization
- Type augmentation system for custom derives and builder patterns
- Template-based code generation with customizable output

## Dependencies

This crate builds upon `xsd-fragments` to process schema definitions and relies on `syn`, `quote`, and `proc-macro2` for Rust code generation.
