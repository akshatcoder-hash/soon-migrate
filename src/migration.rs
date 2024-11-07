use crate::cli::Config;
use crate::errors::MigrationError;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use colored::*;

#[derive(Deserialize, Serialize, Debug)]
struct Provider {
    cluster: String,
    wallet: String,
}

#[derive(Deserialize, Serialize, Debug)]
struct Programs {
    #[serde(rename = "localnet")]
    localnet: std::collections::HashMap<String, String>,
}

#[derive(Deserialize, Serialize, Debug)]
struct AnchorToml {
    provider: Provider,
    programs: Programs,
    #[serde(flatten)]
    extra: std::collections::HashMap<String, toml::Value>,
}

pub fn run_migration(config: &Config) -> Result<(), MigrationError> {
    validate_anchor_project(&config.path)?;

    let anchor_toml_path = Path::new(&config.path).join("Anchor.toml");

    // Backup original Anchor.toml
    let backup_path = anchor_toml_path.with_extension("toml.bak");
    fs::copy(&anchor_toml_path, &backup_path)
        .map_err(|e| MigrationError::BackupFailed(e.to_string()))?;

    if config.verbose {
        println!("{}", "Backup created successfully.".cyan());
    }

    // Read Anchor.toml
    let content = fs::read_to_string(&anchor_toml_path)
        .map_err(|e| MigrationError::ReadFailed(e.to_string()))?;

    // Parse TOML
    let mut toml_value: toml::Value = content.parse()
        .map_err(|e: toml::de::Error| MigrationError::TomlParseError(e.to_string()))?;

    // Update the cluster value in the provider section
    if let Some(provider) = toml_value.get_mut("provider") {
        if let Some(table) = provider.as_table_mut() {
            table.insert("cluster".to_string(), toml::Value::String("https://rpc.devnet.soo.network".to_string()));
        }
    }

    if config.verbose {
        println!("{}", "Cluster RPC URL updated.".cyan());
    }

    // Write back to Anchor.toml unless dry_run
    if !config.dry_run {
        let toml_string = toml::to_string_pretty(&toml_value)
            .map_err(|e| MigrationError::TomlParseError(e.to_string()))?;
            
        fs::write(&anchor_toml_path, toml_string)
            .map_err(|e| MigrationError::WriteFailed(e.to_string()))?;
            
        if config.verbose {
            println!("{}", "Anchor.toml written successfully.".cyan());
        }
    } else {
        println!("{}", "Dry run enabled. Changes not written.".yellow());
        println!("{}", toml::to_string_pretty(&toml_value)
            .map_err(|e| MigrationError::TomlParseError(e.to_string()))?
            .cyan());
    }

    Ok(())
}

pub fn restore_backup(path: &str) -> Result<(), MigrationError> {
    let anchor_toml_path = Path::new(path).join("Anchor.toml");
    let backup_path = anchor_toml_path.with_extension("toml.bak");

    if !backup_path.exists() {
        return Err(MigrationError::BackupNotFound(backup_path.to_string_lossy().into_owned()));
    }

    fs::copy(&backup_path, &anchor_toml_path)
        .map_err(|e| MigrationError::RestoreFailed(e.to_string()))?;

    Ok(())
}

fn validate_anchor_project(path: &str) -> Result<(), MigrationError> {
    let anchor_toml_path = Path::new(path).join("Anchor.toml");
    if !anchor_toml_path.exists() {
        return Err(MigrationError::NotAnAnchorProject(path.to_string()));
    }

    let cargo_toml_path = Path::new(path).join("Cargo.toml");
    if !cargo_toml_path.exists() {
        return Err(MigrationError::NotAnAnchorProject(path.to_string()));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    fn create_test_anchor_project() -> TempDir {
        let temp_dir = TempDir::new().unwrap();
        let anchor_toml_content = r#"
[toolchain]

[features]
resolution = true
skip-lint = false

[programs.localnet]
migration = "EtQdsPNDckBhME3gRjcj9Z4Z9tGEYAoHjWKv7aHJgBua"

[registry]
url = "https://api.apr.dev"

[provider]
cluster = "Localnet"
wallet = "~/.config/solana/id.json"

[scripts]
test = "yarn run ts-mocha -p ./tsconfig.json -t 1000000 tests/**/*.ts"
"#;

        fs::write(
            temp_dir.path().join("Anchor.toml"),
            anchor_toml_content,
        ).unwrap();

        fs::write(
            temp_dir.path().join("Cargo.toml"),
            "[package]\nname = \"test\"\nversion = \"0.1.0\"\n",
        ).unwrap();

        temp_dir
    }

    #[test]
    fn test_migration_dry_run() {
        let test_dir = create_test_anchor_project();
        let config = Config {
            path: test_dir.path().to_str().unwrap().to_string(),
            dry_run: true,
            verbose: false,
            restore: false,
        };

        let result = run_migration(&config);
        assert!(result.is_ok());
        
        // Verify original file wasn't changed
        let content = fs::read_to_string(test_dir.path().join("Anchor.toml")).unwrap();
        assert!(content.contains("cluster = \"Localnet\""));
    }

    #[test]
    fn test_migration_actual() {
        let test_dir = create_test_anchor_project();
        let config = Config {
            path: test_dir.path().to_str().unwrap().to_string(),
            dry_run: false,
            verbose: false,
            restore: false,
        };

        let result = run_migration(&config);
        assert!(result.is_ok());

        // Verify file was changed
        let content = fs::read_to_string(test_dir.path().join("Anchor.toml")).unwrap();
        assert!(content.contains("https://rpc.devnet.soo.network"));
        
        // Verify backup was created
        assert!(Path::new(&test_dir.path().join("Anchor.toml.bak")).exists());
    }

    #[test]
    fn test_restore_backup() {
        let test_dir = create_test_anchor_project();
        
        // First run migration
        let config = Config {
            path: test_dir.path().to_str().unwrap().to_string(),
            dry_run: false,
            verbose: false,
            restore: false,
        };
        run_migration(&config).unwrap();

        // Then restore
        let restore_result = restore_backup(test_dir.path().to_str().unwrap());
        assert!(restore_result.is_ok());

        // Verify content was restored
        let content = fs::read_to_string(test_dir.path().join("Anchor.toml")).unwrap();
        assert!(content.contains("cluster = \"Localnet\""));
    }

    #[test]
    fn test_invalid_path() {
        let config = Config {
            path: "/nonexistent/path".to_string(),
            dry_run: false,
            verbose: false,
            restore: false,
        };

        let result = run_migration(&config);
        assert!(matches!(result, Err(MigrationError::NotAnAnchorProject(_))));
    }
}