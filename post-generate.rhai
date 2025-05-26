// Rename template files.
file::rename(".github/workflows/release.yaml.template", ".github/workflows/release.yaml");
file::rename("Cargo.toml.template", "Cargo.toml");
file::rename("README.md.template", "README.md");
file::rename("src/main.rs.template", "src/main.rs");

// Generate `Cargo.lock`.
system::command("cargo", ["update", "--package", variable::get("project-name")]);
