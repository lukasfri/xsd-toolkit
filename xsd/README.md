# XSD

Core library for parsing and manipulating XML Schema Definition (XSD) files. This crate provides the foundation for the XSD toolkit by offering parsing capabilities, schema manipulation utilities, and fundamental data structures for working with XSD documents.

## Features

- Parse XSD files into structured Rust representations
- Navigate schema compositions, includes, and imports
- Query elements, attributes, and types within schemas
- Extract and manipulate namespace information
- Handle schema linking and URL resolution

## Usage

This crate serves as the foundational layer for other crates in the toolkit, particularly `xsd-fragments` and `xsd-codegen-xmlity`. It provides essential functionality for loading and interpreting XSD schemas before they are processed for code generation or analysis.
