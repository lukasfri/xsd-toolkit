# XSD Fragments Transformer

Schema transformation engine that applies structural modifications to XSD fragments before code generation. This crate implements various transformation passes to normalize, flatten, and optimize schema structures for better code generation output.

## Features

- Complex type transformations including extension and restriction expansion
- Sequence and choice flattening for improved type structure
- Attribute declaration expansion and prohibited attribute removal
- Simple type restriction expansion
- Transformation change tracking and dependency management

## Transformations

### Complex Type Transformations

- Expand extension fragments into concrete implementations
- Expand restriction fragments with proper inheritance
- Flatten nested sequences and choices for cleaner output
- Convert single-choice elements to sequences where appropriate
- Expand shortform complex types into full definitions

### Simple Type Transformations

- Expand simple type restrictions with facet information

This crate works in conjunction with `xsd-fragments` to prepare schema data for efficient code generation by `xsd-codegen-xmlity`.
