# XSD Fragments

Schema fragment compilation system that transforms XSD definitions into structured, analyzable fragments. This crate compiles XSD schemas into an intermediate representation optimized for code generation and schema analysis.

## Features

- Compile XSD schemas into structured fragment representations
- Separate compilation of simple and complex types
- Namespace-aware fragment organization and indexing
- Top-level type identification and categorization
- Fragment-based schema querying and traversal

## Architecture

The crate organizes compiled schema information into fragments, with separate compilation pipelines for simple and complex types. This design enables efficient querying, transformation, and code generation from XSD schemas while maintaining type safety and namespace awareness.

## Usage

This crate serves as the core compilation layer between raw XSD parsing (from the `xsd` crate) and code generation (via `xsd-codegen-xmlity`), providing the structured intermediate representation that enables sophisticated schema transformations and analysis.
