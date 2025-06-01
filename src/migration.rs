//! Migration module for handling the migration of Solana Anchor projects to the SOON Network.
//!
//! This module provides functionality to migrate Anchor.toml configuration files,
//! detect oracles, and handle backup/restore operations.

use crate::cli::Config;
use crate::errors::MigrationError;
use crate::oracle::{OracleDetector, OracleReport};
use colored::*;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

/// Represents the provider configuration in Anchor.toml
#[derive(Deserialize, Serialize, Debug)]
struct Provider {
    /// The cluster URL or name (e.g., "mainnet-beta", "testnet", "devnet")
    cluster: String,
    /// The wallet path or configuration
    wallet: String,
}

/// Represents the programs section in Anchor.toml
#[derive(Deserialize, Serialize, Debug)]
struct Programs {
    /// Program IDs for the localnet environment
    #[serde(rename = "localnet")]
    localnet: std::collections::HashMap<String, String>,
}

/// Represents the structure of an Anchor.toml file
#[derive(Deserialize, Serialize, Debug)]
struct AnchorToml {
    /// Provider configuration
    provider: Provider,
    /// Program configurations
    programs: Programs,
    /// Any additional fields that aren't explicitly defined
    #[serde(flatten)]
    extra: std::collections::HashMap<String, toml::Value>,
}

/// The result of a migration operation
#[derive(Debug)]
pub struct MigrationResult {
    /// Whether the configuration was updated
    pub config_updated: bool,
    /// Optional report about detected oracles
    pub oracle_report: Option<OracleReport>,
    /// Any warnings that occurred during migration
    pub warnings: Vec<String>,
    /// Recommended next steps for the user
    pub next_steps: Vec<String>,
}

fn map_cluster_to_soon(cluster: &str) -> &'static str {
    match cluster.to_lowercase().as_str() {
        "mainnet-beta" | "mainnet" => "https://rpc.mainnet.soo.network/rpc",
        "testnet" => "https://rpc.testnet.soo.network/rpc", 
        "devnet" | _ => "https://rpc.devnet.soo.network/rpc",
    }
}

/// Run the migration process for an Anchor project
///
/// # Arguments
/// * `config` - Configuration for the migration
///
/// # Returns
/// A `Result` containing the migration result or an error
///
/// # Errors
/// Returns `MigrationError` if any step of the migration fails
pub fn run_migration(config: &Config) -> Result<MigrationResult, MigrationError> {
    validate_anchor_project(&config.path)?;

    let mut result = MigrationResult {
        config_updated: false,
        oracle_report: None,
        warnings: Vec::new(),
        next_steps: Vec::new(),
    };

    // Always run oracle detection first
    if config.verbose {
        println!("{}", "Running oracle detection...".cyan());
    }
    
    let oracle_report = OracleDetector::scan_project(&config.path, config.verbose)?;
    result.oracle_report = Some(oracle_report);

    // If oracle-only mode, skip Anchor.toml migration
    if config.oracle_only {
        if config.verbose {
            println!("{}", "Oracle-only mode: skipping Anchor.toml migration".yellow());
        }
        return Ok(result);
    }

    // Proceed with Anchor.toml migration
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
    let mut toml_value: toml::Value = content
        .parse()
        .map_err(|e: toml::de::Error| MigrationError::TomlParseError(e.to_string()))?;

    let mut config_changed = false;

    // Update the cluster value in the provider section
    if let Some(provider) = toml_value.get_mut("provider") {
        if let Some(table) = provider.as_table_mut() {
            // Store cluster value first before modifying table
            let cluster_value = table.get("cluster")
                .and_then(|c| c.as_str())
                .map(|c| c.to_string());
            
            if let Some(cluster) = cluster_value {
                let soon_rpc = map_cluster_to_soon(&cluster);
                table.insert("cluster".to_string(), toml::Value::String(soon_rpc.to_string()));
                
                if config.verbose {
                    println!("{}", format!("Updating cluster from '{}' to '{}'", cluster, soon_rpc).cyan());
                }
                config_changed = true;
            }
        }
    }

    // Determine the appropriate network name before modifying programs section
    let network_name = {
        if let Some(provider) = toml_value.get("provider") {
            if let Some(cluster) = provider.get("cluster").and_then(|c| c.as_str()) {
                if cluster.contains("mainnet") {
                    "mainnet"
                } else if cluster.contains("testnet") {
                    "testnet"
                } else {
                    "devnet"
                }
            } else {
                "devnet"
            }
        } else {
            "devnet"
        }
    };

    // Update programs section: change programs.localnet to appropriate network
    if let Some(programs) = toml_value.get_mut("programs") {
        if let Some(table) = programs.as_table_mut() {
            if let Some(localnet) = table.remove("localnet") {
                table.insert(network_name.to_string(), localnet);
                if config.verbose {
                    println!("{}", format!("Updated programs.localnet to programs.{}", network_name).cyan());
                }
                config_changed = true;
            }
        }
    }

    // Add oracle-related warnings if oracles detected
    if let Some(ref oracle_report) = result.oracle_report {
        if !oracle_report.detected_oracles.is_empty() {
            result.warnings.push("Oracle usage detected in your project. Review the oracle migration recommendations.".to_string());
            
            // Add specific warnings for high-confidence detections
            for detection in &oracle_report.detected_oracles {
                if matches!(detection.confidence, crate::oracle::ConfidenceLevel::High) {
                    result.warnings.push(format!("{:?} oracle detected - migration required for SOON compatibility", detection.oracle_type));
                }
            }
        }
    }

    // Generate next steps
    result.next_steps.push("1. Update your dependencies if using oracles".to_string());
    result.next_steps.push("2. Test your project on SOON devnet".to_string());
    result.next_steps.push("3. Review oracle integration if detected".to_string());
    result.next_steps.push("4. Deploy to SOON Network".to_string());

    if config.verbose {
        println!("{}", "Configuration updated successfully.".cyan());
    }

    // Write back to Anchor.toml unless dry_run
    if !config.dry_run && config_changed {
        let toml_string = toml::to_string_pretty(&toml_value)
            .map_err(|e| MigrationError::TomlParseError(e.to_string()))?;

        fs::write(&anchor_toml_path, toml_string)
            .map_err(|e| MigrationError::WriteFailed(e.to_string()))?;

        if config.verbose {
            println!("{}", "Anchor.toml written successfully.".cyan());
        }
        result.config_updated = true;
    } else if config.dry_run {
        if config.verbose {
            println!("{}", "Dry run enabled. Changes not written.".yellow());
            println!(
                "{}",
                toml::to_string_pretty(&toml_value)
                    .map_err(|e| MigrationError::TomlParseError(e.to_string()))?
                    .cyan()
            );
        }
    } else if !config_changed {
        if config.verbose {
            println!("{}", "No changes needed to Anchor.toml".green());
        }
    }

    Ok(result)
}

/// Scan the project for oracles without performing any migrations
///
/// # Arguments
/// * `config` - Configuration for the oracle scan
///
/// # Returns
/// A `Result` containing the oracle report or an error
///
/// # Errors
/// Returns `MigrationError` if the oracle detection fails
pub fn run_oracle_scan_only(config: &Config) -> Result<OracleReport, MigrationError> {
    validate_anchor_project(&config.path)?;
    OracleDetector::scan_project(&config.path, config.verbose)
}

/// Restore the Anchor.toml file from a backup
///
/// # Arguments
/// * `path` - Path to the backup file
///
/// # Returns
/// A `Result` indicating success or an error
///
/// # Errors
/// Returns `MigrationError` if the backup cannot be restored
pub fn restore_backup(path: &str) -> Result<(), MigrationError> {
    let anchor_toml_path = Path::new(path).join("Anchor.toml");
    let backup_path = anchor_toml_path.with_extension("toml.bak");

    if !backup_path.exists() {
        return Err(MigrationError::BackupNotFound(
            backup_path.to_string_lossy().into_owned(),
        ));
    }

    fs::copy(&backup_path, &anchor_toml_path)
        .map_err(|e| MigrationError::RestoreFailed(e.to_string()))?;

    if Path::new(&backup_path).exists() {
        fs::remove_file(backup_path)
            .map_err(|e| MigrationError::RestoreFailed(e.to_string()))?;
    }

    Ok(())
}

/// Validate that the given path contains a valid Anchor project
///
/// # Arguments
/// * `path` - Path to validate as an Anchor project
///
/// # Returns
/// A `Result` indicating if the path is valid or an error
///
/// # Errors
/// Returns `MigrationError::NotAnAnchorProject` if validation fails
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
cluster = "devnet"
wallet = "~/.config/solana/id.json"

[scripts]
test = "yarn run ts-mocha -p ./tsconfig.json -t 1000000 tests/**/*.ts"
"#;

        let cargo_toml_content = r#"
[package]
name = "test"
version = "0.1.0"

[dependencies]
anchor-lang = "0.28.0"
"#;

        fs::write(temp_dir.path().join("Anchor.toml"), anchor_toml_content).unwrap();
        fs::write(temp_dir.path().join("Cargo.toml"), cargo_toml_content).unwrap();

        temp_dir
    }

    fn create_test_anchor_project_with_oracle() -> TempDir {
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
cluster = "devnet"
wallet = "~/.config/solana/id.json"

[scripts]
test = "yarn run ts-mocha -p ./tsconfig.json -t 1000000 tests/**/*.ts"
"#;

        let cargo_toml_content = r#"
[package]
name = "test"
version = "0.1.0"

[dependencies]
anchor-lang = "0.28.0"
pyth-solana-receiver-sdk = "0.2.0"
"#;

        let rust_code = r#"
use anchor_lang::prelude::*;
use pyth_solana_receiver_sdk::PriceUpdateV2;

pub fn get_price() -> Result<()> {
    // Get price from Pyth
    let price = price_update.get_price_no_older_than(&clock, 60)?;
    Ok(())
}
"#;

        fs::write(temp_dir.path().join("Anchor.toml"), anchor_toml_content).unwrap();
        fs::write(temp_dir.path().join("Cargo.toml"), cargo_toml_content).unwrap();
        
        // Create src directory and price.rs file
        fs::create_dir_all(temp_dir.path().join("src")).unwrap();
        fs::write(temp_dir.path().join("src").join("price.rs"), rust_code).unwrap();

        temp_dir
    }

    #[test]
    fn test_basic_migration() {
        let test_dir = create_test_anchor_project();
        let config = Config {
            path: test_dir.path().to_str().unwrap().to_string(),
            dry_run: false,
            verbose: false,
            restore: false,
            show_guide: false,
            oracle_only: false,
        };

        let result = run_migration(&config).unwrap();
        
        // Verify config was updated
        assert!(result.config_updated);
        
        // Verify file was changed
        let content = fs::read_to_string(test_dir.path().join("Anchor.toml")).unwrap();
        assert!(content.contains("https://rpc.devnet.soo.network/rpc"));
        assert!(content.contains("[programs.devnet]"));
    }

    #[test]
    fn test_migration_with_oracle_detection() {
        let test_dir = create_test_anchor_project_with_oracle();
        let config = Config {
            path: test_dir.path().to_str().unwrap().to_string(),
            dry_run: false,
            verbose: false,
            restore: false,
            show_guide: false,
            oracle_only: false,
        };

        let result = run_migration(&config).unwrap();
        
        // Verify oracle was detected
        assert!(result.oracle_report.is_some());
        let oracle_report = result.oracle_report.unwrap();
        assert!(!oracle_report.detected_oracles.is_empty());
        
        // Verify Pyth was detected
        let has_pyth = oracle_report.detected_oracles.iter()
            .any(|d| matches!(d.oracle_type, crate::oracle::OracleType::Pyth));
        assert!(has_pyth);

        // Verify config was updated
        assert!(result.config_updated);
        
        // Verify warnings about oracle usage
        assert!(!result.warnings.is_empty());
    }

    #[test]
    fn test_oracle_only_mode() {
        let test_dir = create_test_anchor_project_with_oracle();
        let config = Config {
            path: test_dir.path().to_str().unwrap().to_string(),
            dry_run: false,
            verbose: false,
            restore: false,
            show_guide: false,
            oracle_only: true,
        };

        let result = run_migration(&config).unwrap();
        
        // Verify oracle was detected
        assert!(result.oracle_report.is_some());
        
        // Verify config was NOT updated in oracle-only mode
        assert!(!result.config_updated);
        
        // Verify original file wasn't changed
        let content = fs::read_to_string(test_dir.path().join("Anchor.toml")).unwrap();
        assert!(content.contains("cluster = \"devnet\""));
    }

    #[test]
    fn test_dry_run_mode() {
        let test_dir = create_test_anchor_project();
        let config = Config {
            path: test_dir.path().to_str().unwrap().to_string(),
            dry_run: true,
            verbose: false,
            restore: false,
            show_guide: false,
            oracle_only: false,
        };

        let result = run_migration(&config).unwrap();
        
        // Verify config was NOT updated in dry-run mode
        assert!(!result.config_updated);
        
        // Verify original file wasn't changed
        let content = fs::read_to_string(test_dir.path().join("Anchor.toml")).unwrap();
        assert!(content.contains("cluster = \"devnet\""));
    }

    #[test]
    fn test_network_mapping() {
        assert_eq!(map_cluster_to_soon("mainnet-beta"), "https://rpc.mainnet.soo.network/rpc");
        assert_eq!(map_cluster_to_soon("testnet"), "https://rpc.testnet.soo.network/rpc");
        assert_eq!(map_cluster_to_soon("devnet"), "https://rpc.devnet.soo.network/rpc");
        // Default fallback to devnet for unknown clusters
        assert_eq!(map_cluster_to_soon("unknown"), "https://rpc.devnet.soo.network/rpc");
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
            show_guide: false,
            oracle_only: false,
        };
        run_migration(&config).unwrap();

        // Then restore
        let restore_result = restore_backup(test_dir.path().to_str().unwrap());
        assert!(restore_result.is_ok());

        // Verify content was restored
        let content = fs::read_to_string(test_dir.path().join("Anchor.toml")).unwrap();
        assert!(content.contains("cluster = \"devnet\""));
    }

    #[test]
    fn test_invalid_path() {
        let config = Config {
            path: "/nonexistent/path".to_string(),
            dry_run: false,
            verbose: false,
            restore: false,
            show_guide: false,
            oracle_only: false,
        };

        let result = run_migration(&config);
        assert!(matches!(result, Err(MigrationError::NotAnAnchorProject(_))));
    }
}