//! Theme generation script
//! Run with: cargo run --manifest-path ../../rustscript/crates/compiler/schema-codegen/Cargo.toml --example generate_theme

use rsc_schema_codegen::{generate_css_from_theme, generate_rust_from_theme};

fn main() {
    let theme_path = "configs/theme.yaml";
    let css_output = "styles/generated/theme.css";
    let rust_output = "src/generated/theme.rs";

    println!("Generating theme from {}", theme_path);

    match generate_css_from_theme(theme_path, css_output) {
        Ok(_) => println!("✓ Generated {}", css_output),
        Err(e) => eprintln!("✗ Failed to generate CSS: {}", e),
    }

    match generate_rust_from_theme(theme_path, rust_output) {
        Ok(_) => println!("✓ Generated {}", rust_output),
        Err(e) => eprintln!("✗ Failed to generate Rust: {}", e),
    }
}
