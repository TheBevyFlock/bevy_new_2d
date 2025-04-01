// Rename template files.
file::rename(".github/workflows/release.yaml.template", ".github/workflows/release.yaml");
file::rename("Cargo.toml.template", "Cargo.toml");
file::rename("README.md.template", "README.md");
file::rename("src/main.rs.template", "src/main.rs");
file::rename("src/lib.rs.template", "src/lib.rs");
file::rename("src/audio.rs.template", "src/audio.rs");

// Generate `Cargo.lock`.
system::command("cargo", ["update", "--package", variable::get("project-name")]);
