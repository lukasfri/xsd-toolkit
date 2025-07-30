# XMLity Build

Build-time code generation system for creating Rust modules from XSD schemas. This crate provides a high-level API for integrating XSD-to-Rust code generation into build scripts and procedural macros.

## Features

- Batch processing of XSD files from glob patterns and URLs
- Network resolution of schema dependencies
- Namespace binding and type customization
- Integration with build scripts for compile-time generation
- Support for bon builders, enum conversions, and custom derives

## Usage

This crate is designed for use in build scripts where XSD schemas need to be converted to Rust code at compile time. It orchestrates the entire pipeline from schema loading through fragment compilation to final code generation, providing a streamlined interface for build-time schema processing.
