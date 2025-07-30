# XSD Toolkit

A comprehensive Rust toolkit for parsing, transforming, and generating code from XML Schema Definition (XSD) files. This toolkit provides a complete pipeline from XSD parsing to strongly-typed Rust code generation, with support for complex schema transformations and XMLity-based serialization.

## Overview

The XSD Toolkit consists of multiple interconnected crates that work together to process XSD schemas:

1. **Parse** XSD files into structured representations
2. **Fragment** schemas into analyzable components
3. **Transform** fragments for optimization and normalization
4. **Generate** type-safe Rust code with XMLity integration
5. **Query** compiled schemas dynamically at runtime

## Core Crates

### [`xsd`](./xsd)

Core library for parsing and manipulating XSD files. Provides the foundation for schema loading, navigation, and basic manipulation.

### [`xsd-fragments`](./xsd-fragments)

Schema fragment compilation system that transforms XSD definitions into structured, analyzable fragments optimized for code generation.

### [`xsd-fragments-transformer`](./xsd-fragments-transformer)

Schema transformation engine that applies structural modifications to normalize, flatten, and optimize schema structures before code generation.

### [`xsd-codegen-xmlity`](./xsd-codegen-xmlity)

Code generation engine that produces strongly-typed Rust code from XSD schemas using the XMLity serialization framework.

### [`xsd-dynamic-query`](./xsd-dynamic-query)

Runtime query system for examining compiled schema fragments, enabling dynamic inspection and traversal of schema structures.

### [`xmlity-build`](./xmlity-build)

Build-time code generation system providing high-level APIs for integrating XSD-to-Rust generation into build scripts and procedural macros.

### [`xsd-builtin`](./xsd-builtin)

Implementation of XML Schema built-in types and facet constraints as defined in the W3C XML Schema specification.

## Schema Namespace Crates

Pre-generated Rust bindings for standard XML namespaces:

- **[`xmlity-ns`](./schemas/xmlity-ns)** - Core namespace utilities and common functionality
- **[`xmlity-ns-xml`](./schemas/xmlity-ns-xml)** - XML namespace bindings (`http://www.w3.org/XML/1998/namespace`)
- **[`xmlity-ns-xs`](./schemas/xmlity-ns-xs)** - XML Schema namespace bindings (`http://www.w3.org/2001/XMLSchema`)
- **[`xmlity-ns-xsi`](./schemas/xmlity-ns-xsi)** - XML Schema Instance namespace bindings (`http://www.w3.org/2001/XMLSchema-instance`)
- **[`xmlity-ns-xhtml`](./schemas/xmlity-ns-xhtml)** - XHTML 1.0 Strict namespace bindings

## Features

- **Complete XSD Support**: Parse and process complex XSD schemas with inheritance, restrictions, extensions, and substitution groups
- **Type-Safe Code Generation**: Generate strongly-typed Rust structures with compile-time validation
- **XMLity Integration**: Seamless XML serialization/deserialization through the XMLity framework
- **Build-Time Generation**: Integrate schema processing into Rust build pipelines
- **Runtime Queries**: Dynamically inspect and traverse compiled schema structures
- **Namespace Aware**: Full support for XML namespaces and schema modularity
- **Transformation Pipeline**: Optimize and normalize schemas for better code generation

## License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in XMLity by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
</sub>
