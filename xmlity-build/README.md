# XMLity Build

Build-time code generation for transforming XSD files into [XMLity](https://github.com/lukasfri/xmlity)-compatible Rust data structures.

XMLity Build converts XSD schemas (local files or URLs) into Rust structs, enums, and types that work with the [XMLity](https://github.com/lukasfri/xmlity) XML serialization framework.

**⚠️ Note**: Code generation can take several seconds to minutes depending on schema complexity.

## Features

- Process XSD files from local files or URLs
- Automatic schema dependency resolution via network (configurable)
- Namespace binding and type customization
- Generate structs with bon builders, enum conversions, and custom derives
- Build script and standalone example integration

## Usage Scenarios

### Build Scripts (`build.rs`)

Generate code at compile time:

```rust
use syn::parse_quote;
use xmlity::XmlNamespace;

fn main() {
    // Tell Cargo to rerun if schema files change
    println!("cargo::rerun-if-changed=schemas/");

    let engine = xmlity_build::BuildEngine::builder()
        .allowed_files(vec!["schemas/**/*.xsd".to_string()])
        .allow_network_access(true)
        .bound_namespaces(vec![
            (XmlNamespace::XS, parse_quote!(xmlity_ns_xs)),
        ])
        .build();

    let engine = engine.start().unwrap();

    let out_dir = std::env::var("OUT_DIR").unwrap();
    let output_path = format!("{out_dir}/generated.rs").parse().unwrap();

    engine
        .generate_namespace(
            xmlity_build::GenerateNamespaceConfig::builder()
                .output_file(output_path)
                .namespace(XmlNamespace::XS)
                .bon_builders(true)
                .enum_from_impls(true)
                .struct_from_impls(true)
                .build(),
        )
        .unwrap();
}
```

### Standalone Examples

Generate code in examples for development or one-time use:

```rust
use syn::parse_quote;
use xmlity::XmlNamespace;

fn main() {
    println!("Starting schema processing...");
    let start_time = std::time::Instant::now();

    let engine = xmlity_build::BuildEngine::builder()
        .allowed_files(vec!["schemas/**/*.xsd".to_string()])
        .allow_network_access(true)
        .bound_namespaces(vec![(XmlNamespace::XML, parse_quote!(crate))])
        .build();

    let engine = engine.start().expect("Failed to start engine");
    println!("Engine startup took: {:?}", start_time.elapsed());

    // Generate code to a specific file
    engine
        .generate_namespace(
            xmlity_build::GenerateNamespaceConfig::builder()
                .output_file("src/generated.rs".parse().unwrap())
                .namespace(XmlNamespace::XML)
                .bon_builders(true)
                .enum_from_impls(true)
                .struct_from_impls(true)
                .build(),
        )
        .expect("Failed to generate code");

    println!("Total processing time: {:?}", start_time.elapsed());
}
```

## When to Use Each Approach

- **Build Scripts**: Automatic generation during builds, code regenerated when schemas change
- **Standalone Examples**: Development experimentation or one-time generation for committing to repository

## Performance

Processing includes schema parsing, dependency resolution, fragment generation, and code generation. Simple schemas take a few hundred milliseconds while complex schemas may take several seconds to minutes depending on the number of dependencies and complexity.
