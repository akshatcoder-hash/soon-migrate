use crate::errors::MigrationError;
use colored::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OracleDetection {
    pub oracle_type: OracleType,
    pub confidence: ConfidenceLevel,
    pub locations: Vec<DetectionLocation>,
    pub migration_suggestion: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OracleType {
    Pyth,
    Switchboard,
    Chainlink,
    DIA,
    RedStone,
    APRO,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConfidenceLevel {
    High,    // Found in dependencies + code usage
    Medium,  // Found in dependencies OR clear code patterns
    Low,     // Found in comments or weak patterns
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetectionLocation {
    pub file_path: String,
    pub line_number: Option<usize>,
    pub pattern_matched: String,
    pub context: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OracleReport {
    pub detected_oracles: Vec<OracleDetection>,
    pub migration_recommendations: Vec<String>,
    pub apro_integration_guide: Option<String>,
}

pub struct OracleDetector;

impl OracleDetector {
    pub fn scan_project(path: &str, verbose: bool) -> Result<OracleReport, MigrationError> {
        if verbose {
            println!("{}", "Scanning project for oracle usage...".cyan());
        }

        let mut detected_oracles = Vec::new();
        
        // Scan Cargo.toml for oracle dependencies
        let cargo_detections = Self::scan_cargo_toml(path)?;
        detected_oracles.extend(cargo_detections);

        // Scan Rust files for oracle imports and usage
        let code_detections = Self::scan_rust_files(path)?;
        detected_oracles.extend(code_detections);

        // Merge and deduplicate detections
        let merged_detections = Self::merge_detections(detected_oracles);
        
        // Generate migration recommendations
        let recommendations = Self::generate_recommendations(&merged_detections);
        
        // Generate APRO integration guide if oracles detected
        let apro_guide = if !merged_detections.is_empty() {
            Some(Self::generate_apro_guide(&merged_detections))
        } else {
            None
        };

        Ok(OracleReport {
            detected_oracles: merged_detections,
            migration_recommendations: recommendations,
            apro_integration_guide: apro_guide,
        })
    }

    fn scan_cargo_toml(path: &str) -> Result<Vec<OracleDetection>, MigrationError> {
        let cargo_path = Path::new(path).join("Cargo.toml");
        if !cargo_path.exists() {
            return Ok(Vec::new());
        }

        let content = fs::read_to_string(&cargo_path)
            .map_err(|e| MigrationError::OracleDetectionFailed(format!("Failed to read Cargo.toml: {}", e)))?;

        let mut detections = Vec::new();

        // Pyth detection
        if content.contains("pyth-solana-receiver-sdk") {
            detections.push(OracleDetection {
                oracle_type: OracleType::Pyth,
                confidence: ConfidenceLevel::High,
                locations: vec![DetectionLocation {
                    file_path: "Cargo.toml".to_string(),
                    line_number: Self::find_line_number(&content, "pyth-solana-receiver-sdk"),
                    pattern_matched: "pyth-solana-receiver-sdk".to_string(),
                    context: "Cargo.toml dependency".to_string(),
                }],
                migration_suggestion: "Consider migrating to APRO oracle for SOON Network compatibility. APRO provides similar price feed functionality with better performance.".to_string(),
            });
        }

        // Switchboard detection
        if content.contains("switchboard-v2") || content.contains("switchboard_on_demand") {
            let pattern = if content.contains("switchboard-v2") { "switchboard-v2" } else { "switchboard_on_demand" };
            detections.push(OracleDetection {
                oracle_type: OracleType::Switchboard,
                confidence: ConfidenceLevel::High,
                locations: vec![DetectionLocation {
                    file_path: "Cargo.toml".to_string(),
                    line_number: Self::find_line_number(&content, pattern),
                    pattern_matched: pattern.to_string(),
                    context: "Cargo.toml dependency".to_string(),
                }],
                migration_suggestion: "APRO oracle offers customizable data feeds similar to Switchboard with enhanced performance on SOON Network.".to_string(),
            });
        }

        // Chainlink detection
        if content.contains("chainlink_solana") {
            detections.push(OracleDetection {
                oracle_type: OracleType::Chainlink,
                confidence: ConfidenceLevel::High,
                locations: vec![DetectionLocation {
                    file_path: "Cargo.toml".to_string(),
                    line_number: Self::find_line_number(&content, "chainlink_solana"),
                    pattern_matched: "chainlink_solana".to_string(),
                    context: "Cargo.toml dependency".to_string(),
                }],
                migration_suggestion: "APRO oracle provides reliable price feeds compatible with Chainlink's API patterns for seamless migration.".to_string(),
            });
        }

        Ok(detections)
    }

    fn scan_rust_files(path: &str) -> Result<Vec<OracleDetection>, MigrationError> {
        let mut detections = Vec::new();
        
        // Walk through all .rs files
        Self::walk_directory(Path::new(path), &mut |file_path| {
            if let Some(extension) = file_path.extension() {
                if extension == "rs" {
                    if let Ok(content) = fs::read_to_string(file_path) {
                        detections.extend(Self::scan_rust_content(&content, file_path)?);
                    }
                }
            }
            Ok(())
        })?;

        Ok(detections)
    }

    fn scan_rust_content(content: &str, file_path: &Path) -> Result<Vec<OracleDetection>, MigrationError> {
        let mut detections = Vec::new();
        let file_path_str = file_path.to_string_lossy().to_string();

        // Pyth patterns
        let pyth_patterns = [
            "use pyth_solana_receiver_sdk",
            "PriceUpdateV2",
            "get_price_no_older_than",
            "pyth_solana_receiver_sdk::",
        ];

        for pattern in &pyth_patterns {
            if content.contains(pattern) {
                detections.push(OracleDetection {
                    oracle_type: OracleType::Pyth,
                    confidence: ConfidenceLevel::High,
                    locations: vec![DetectionLocation {
                        file_path: file_path_str.clone(),
                        line_number: Self::find_line_number(content, pattern),
                        pattern_matched: pattern.to_string(),
                        context: format!("Rust code usage: {}", pattern),
                    }],
                    migration_suggestion: "Replace Pyth price feeds with APRO oracle integration. See APRO documentation for migration guide.".to_string(),
                });
                break; // Only add one detection per oracle type per file
            }
        }

        // Switchboard patterns
        let switchboard_patterns = [
            "use switchboard_v2",
            "use switchboard_on_demand",
            "AggregatorAccountData",
            "get_result",
            "switchboard_v2::",
        ];

        for pattern in &switchboard_patterns {
            if content.contains(pattern) {
                detections.push(OracleDetection {
                    oracle_type: OracleType::Switchboard,
                    confidence: ConfidenceLevel::High,
                    locations: vec![DetectionLocation {
                        file_path: file_path_str.clone(),
                        line_number: Self::find_line_number(content, pattern),
                        pattern_matched: pattern.to_string(),
                        context: format!("Rust code usage: {}", pattern),
                    }],
                    migration_suggestion: "Replace Switchboard aggregators with APRO oracle data feeds for SOON Network compatibility.".to_string(),
                });
                break;
            }
        }

        // Chainlink patterns
        let chainlink_patterns = [
            "use chainlink_solana",
            "latest_round_data",
            "chainlink_solana::",
            "chainlink::",
        ];

        for pattern in &chainlink_patterns {
            if content.contains(pattern) {
                detections.push(OracleDetection {
                    oracle_type: OracleType::Chainlink,
                    confidence: ConfidenceLevel::High,
                    locations: vec![DetectionLocation {
                        file_path: file_path_str.clone(),
                        line_number: Self::find_line_number(content, pattern),
                        pattern_matched: pattern.to_string(),
                        context: format!("Rust code usage: {}", pattern),
                    }],
                    migration_suggestion: "Migrate Chainlink price feeds to APRO oracle for enhanced performance on SOON Network.".to_string(),
                });
                break;
            }
        }

        // DIA patterns (weaker detection)
        let dia_patterns = [
            "CoinInfo",
            "// DIA",
            "dia oracle",
            "DIA Oracle",
        ];

        for pattern in &dia_patterns {
            if content.to_lowercase().contains(&pattern.to_lowercase()) {
                detections.push(OracleDetection {
                    oracle_type: OracleType::DIA,
                    confidence: ConfidenceLevel::Low,
                    locations: vec![DetectionLocation {
                        file_path: file_path_str.clone(),
                        line_number: Self::find_line_number(content, pattern),
                        pattern_matched: pattern.to_string(),
                        context: format!("Potential DIA usage: {}", pattern),
                    }],
                    migration_suggestion: "If using DIA oracle, consider migrating to APRO for better SOON Network integration.".to_string(),
                });
                break;
            }
        }

        // RedStone patterns (weaker detection)
        let redstone_patterns = [
            "redstone",
            "RedStone",
            "// RedStone",
            "wormhole",
        ];

        for pattern in &redstone_patterns {
            if content.contains(pattern) {
                detections.push(OracleDetection {
                    oracle_type: OracleType::RedStone,
                    confidence: ConfidenceLevel::Low,
                    locations: vec![DetectionLocation {
                        file_path: file_path_str.clone(),
                        line_number: Self::find_line_number(content, pattern),
                        pattern_matched: pattern.to_string(),
                        context: format!("Potential RedStone usage: {}", pattern),
                    }],
                    migration_suggestion: "If using RedStone oracle, APRO provides similar RWA oracle capabilities for SOON Network.".to_string(),
                });
                break;
            }
        }

        Ok(detections)
    }

    fn walk_directory<F>(dir: &Path, callback: &mut F) -> Result<(), MigrationError>
    where
        F: FnMut(&Path) -> Result<(), MigrationError>,
    {
        if dir.is_dir() {
            let entries = fs::read_dir(dir)
                .map_err(|e| MigrationError::OracleDetectionFailed(format!("Failed to read directory: {}", e)))?;
            
            for entry in entries {
                let entry = entry
                    .map_err(|e| MigrationError::OracleDetectionFailed(format!("Failed to read directory entry: {}", e)))?;
                let path = entry.path();
                
                if path.is_dir() {
                    // Skip common directories that won't contain oracle code
                    if let Some(dir_name) = path.file_name() {
                        let dir_str = dir_name.to_string_lossy();
                        if dir_str == "target" || dir_str == "node_modules" || dir_str == ".git" {
                            continue;
                        }
                    }
                    Self::walk_directory(&path, callback)?;
                } else {
                    callback(&path)?;
                }
            }
        }
        Ok(())
    }

    fn find_line_number(content: &str, pattern: &str) -> Option<usize> {
        content
            .lines()
            .enumerate()
            .find(|(_, line)| line.contains(pattern))
            .map(|(i, _)| i + 1)
    }

    fn merge_detections(detections: Vec<OracleDetection>) -> Vec<OracleDetection> {
        let mut oracle_map: HashMap<String, OracleDetection> = HashMap::new();
        
        for detection in detections {
            let key = format!("{:?}", detection.oracle_type);
            
            if let Some(existing) = oracle_map.get_mut(&key) {
                // Merge locations
                existing.locations.extend(detection.locations);
                // Use highest confidence
                if matches!(detection.confidence, ConfidenceLevel::High) {
                    existing.confidence = ConfidenceLevel::High;
                } else if matches!(detection.confidence, ConfidenceLevel::Medium) && matches!(existing.confidence, ConfidenceLevel::Low) {
                    existing.confidence = ConfidenceLevel::Medium;
                }
            } else {
                oracle_map.insert(key, detection);
            }
        }
        
        oracle_map.into_values().collect()
    }

    fn generate_recommendations(detections: &[OracleDetection]) -> Vec<String> {
        let mut recommendations = Vec::new();
        
        if detections.is_empty() {
            recommendations.push("No oracle usage detected. Your project should migrate smoothly to SOON Network.".to_string());
            return recommendations;
        }

        recommendations.push("🔍 Oracle usage detected in your project. Consider these migration steps:".to_string());
        recommendations.push("".to_string());

        for detection in detections {
            let confidence_icon = match detection.confidence {
                ConfidenceLevel::High => "🔴",
                ConfidenceLevel::Medium => "🟡",
                ConfidenceLevel::Low => "🟢",
            };
            
            recommendations.push(format!("{} {:?} Oracle detected with {} confidence", 
                confidence_icon, detection.oracle_type, 
                format!("{:?}", detection.confidence).to_lowercase()));
            
            for location in &detection.locations {
                if let Some(line_num) = location.line_number {
                    recommendations.push(format!("   📁 {}:{} - {}", location.file_path, line_num, location.pattern_matched));
                } else {
                    recommendations.push(format!("   📁 {} - {}", location.file_path, location.pattern_matched));
                }
            }
            recommendations.push(format!("   💡 {}", detection.migration_suggestion));
            recommendations.push("".to_string());
        }

        recommendations.push("📚 Next steps:".to_string());
        recommendations.push("1. Review the APRO oracle integration guide generated below".to_string());
        recommendations.push("2. Update your dependencies to use APRO oracle SDK".to_string());
        recommendations.push("3. Replace oracle-specific code with APRO equivalents".to_string());
        recommendations.push("4. Test your price feed integrations on SOON devnet".to_string());

        recommendations
    }

    fn generate_apro_guide(detections: &[OracleDetection]) -> String {
        let mut guide = String::new();
        
        guide.push_str("# APRO Oracle Integration Guide for SOON Network\n\n");
        guide.push_str("## Overview\n");
        guide.push_str("APRO has chosen SOON as their first SVM chain to support oracle services. ");
        guide.push_str("This guide will help you migrate your existing oracle integrations.\n\n");
        
        guide.push_str("## Program IDs\n");
        guide.push_str("```\n");
        guide.push_str("Devnet:  4Mvy4RKRyJMf4PHavvGUuTj9agoddUZ9atQoFma1tyMY\n");
        guide.push_str("Mainnet: 4Mvy4RKRyJMf4PHavvGUuTj9agoddUZ9atQoFma1tyMY\n");
        guide.push_str("```\n\n");
        
        guide.push_str("## API Endpoints\n");
        guide.push_str("```\n");
        guide.push_str("Devnet:  https://live-api-test.apro.com\n");
        guide.push_str("Mainnet: https://live-api.apro.com\n");
        guide.push_str("```\n\n");

        // Add specific migration examples based on detected oracles
        for detection in detections {
            match detection.oracle_type {
                OracleType::Pyth => {
                    guide.push_str("## Migrating from Pyth\n");
                    guide.push_str("Replace your Pyth price feed calls:\n");
                    guide.push_str("```rust\n");
                    guide.push_str("// Before (Pyth)\n");
                    guide.push_str("use pyth_solana_receiver_sdk::PriceUpdateV2;\n");
                    guide.push_str("let price = price_update.get_price_no_older_than(&clock, max_age)?;\n\n");
                    guide.push_str("// After (APRO)\n");
                    guide.push_str("use oracle_sdk::load_price_feed_from_account_info;\n");
                    guide.push_str("let price_feed = load_price_feed_from_account_info(&price_account)?;\n");
                    guide.push_str("let price = price_feed.benchmark_price;\n");
                    guide.push_str("```\n\n");
                }
                OracleType::Switchboard => {
                    guide.push_str("## Migrating from Switchboard\n");
                    guide.push_str("Replace your Switchboard aggregator calls:\n");
                    guide.push_str("```rust\n");
                    guide.push_str("// Before (Switchboard)\n");
                    guide.push_str("use switchboard_v2::AggregatorAccountData;\n");
                    guide.push_str("let result = aggregator.get_result()?;\n\n");
                    guide.push_str("// After (APRO)\n");
                    guide.push_str("use oracle_sdk::load_price_feed_from_account_info;\n");
                    guide.push_str("let price_feed = load_price_feed_from_account_info(&price_account)?;\n");
                    guide.push_str("let result = price_feed.benchmark_price;\n");
                    guide.push_str("```\n\n");
                }
                OracleType::Chainlink => {
                    guide.push_str("## Migrating from Chainlink\n");
                    guide.push_str("Replace your Chainlink price feed calls:\n");
                    guide.push_str("```rust\n");
                    guide.push_str("// Before (Chainlink)\n");
                    guide.push_str("use chainlink_solana as chainlink;\n");
                    guide.push_str("let round_data = chainlink::latest_round_data(ctx, &feed_account)?;\n\n");
                    guide.push_str("// After (APRO)\n");
                    guide.push_str("use oracle_sdk::load_price_feed_from_account_info;\n");
                    guide.push_str("let price_feed = load_price_feed_from_account_info(&price_account)?;\n");
                    guide.push_str("let price = price_feed.benchmark_price;\n");
                    guide.push_str("```\n\n");
                }
                _ => {}
            }
        }

        guide.push_str("## Available Price Feeds (Devnet)\n");
        guide.push_str("- BTC/USD: 0x0003665949c883f9e0f6f002eac32e00bd59dfe6c34e92a91c37d6a8322d6489\n");
        guide.push_str("- ETH/USD: 0x0003555ace6b39aae1b894097d0a9fc17f504c62fea598fa206cc6f5088e6e45\n");
        guide.push_str("- SOL/USD: 0x000343ec7f6691d6bf679978bab5c093fa45ee74c0baac6cc75649dc59cc21d3\n");
        guide.push_str("- USDT/USD: 0x00039a0c0be4e43cacda1599ac414205651f4a62b614b6be9e5318a182c33eb0\n");
        guide.push_str("- USDC/USD: 0x00034b881a0c0fff844177f881a313ff894bfc6093d33b5514e34d7faa41b7ef\n\n");

        guide.push_str("## Getting Started\n");
        guide.push_str("1. Contact APRO BD team for authorization:\n");
        guide.push_str("   - Email: bd@apro.com\n");
        guide.push_str("   - Telegram: Head of Business Development\n");
        guide.push_str("2. Add APRO oracle SDK to your Cargo.toml\n");
        guide.push_str("3. Update your price feed integration code\n");
        guide.push_str("4. Test on SOON devnet before mainnet deployment\n\n");

        guide.push_str("For detailed integration examples, see the complete APRO documentation.\n");

        guide
    }

    pub fn print_report(report: &OracleReport, verbose: bool) {
        println!("{}", "=== Oracle Detection Report ===".bold().cyan());
        println!();

        if report.detected_oracles.is_empty() {
            println!("{} No oracle usage detected", "✅".green());
            println!("Your project should migrate smoothly to SOON Network without oracle changes.");
            return;
        }

        println!("{} {} oracle(s) detected:", "🔍".yellow(), report.detected_oracles.len());
        println!();

        for detection in &report.detected_oracles {
            let confidence_color = match detection.confidence {
                ConfidenceLevel::High => "red",
                ConfidenceLevel::Medium => "yellow", 
                ConfidenceLevel::Low => "green",
            };

            println!("{} {:?} Oracle ({})", 
                match detection.confidence {
                    ConfidenceLevel::High => "🔴",
                    ConfidenceLevel::Medium => "🟡",
                    ConfidenceLevel::Low => "🟢",
                },
                detection.oracle_type,
                format!("{:?} confidence", detection.confidence).color(confidence_color)
            );

            if verbose {
                for location in &detection.locations {
                    if let Some(line) = location.line_number {
                        println!("  📁 {}:{} - {}", location.file_path, line, location.pattern_matched);
                    } else {
                        println!("  📁 {} - {}", location.file_path, location.pattern_matched);
                    }
                }
            }

            println!("  💡 {}", detection.migration_suggestion);
            println!();
        }

        println!("{}", "=== Migration Recommendations ===".bold().cyan());
        for rec in &report.migration_recommendations {
            if rec.starts_with("🔍") || rec.starts_with("📚") {
                println!("{}", rec.bold());
            } else if rec.is_empty() {
                println!();
            } else {
                println!("{}", rec);
            }
        }

        if report.apro_integration_guide.is_some() {
            println!();
            println!("{}", "💡 Run with --show-guide to see the complete APRO integration guide".yellow());
        }
    }

    pub fn print_integration_guide(report: &OracleReport) {
        if let Some(guide) = &report.apro_integration_guide {
            println!("{}", guide);
        } else {
            println!("No oracle integration guide available.");
        }
    }
}

impl std::fmt::Display for OracleType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OracleType::Pyth => write!(f, "Pyth"),
            OracleType::Switchboard => write!(f, "Switchboard"), 
            OracleType::Chainlink => write!(f, "Chainlink"),
            OracleType::DIA => write!(f, "DIA"),
            OracleType::RedStone => write!(f, "RedStone"),
            OracleType::APRO => write!(f, "APRO"),
            OracleType::Unknown => write!(f, "Unknown"),
        }
    }
}