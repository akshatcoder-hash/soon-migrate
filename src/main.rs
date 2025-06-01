use colored::*;
use indicatif::{ProgressBar, ProgressStyle};
use soon_migrate::{
    cli::Config,
    migration::{restore_backup, run_migration, run_oracle_scan_only},
    oracle::OracleDetector,
    MigrationError,
};
use std::process::exit;
use std::time::Duration;

fn main() {
    let config = Config::new();

    if config.verbose {
        println!("{}", "Starting soon-migrate v2.0...".cyan());
        println!("{}", format!("Project path: {}", config.path).dimmed());
    }

    let progress = ProgressBar::new_spinner();
    progress.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner} {msg}")
            .expect("Failed to create progress style")
            .tick_chars("/|\\- "),
    );

    // Handle restore command
    if config.restore {
        progress.set_message("Restoring from backup...");
        progress.enable_steady_tick(Duration::from_millis(100));
        match restore_backup(&config.path) {
            Ok(_) => {
                progress.finish_with_message("Backup restored successfully.".green().to_string());
                println!("{}", "Restore complete.".green());
            }
            Err(e) => {
                progress.finish_with_message("Restore failed.".red().to_string());
                eprintln!("{}", e.to_string().red());
                exit(1);
            }
        }
        return;
    }

    // Handle oracle-only scan
    if config.oracle_only {
        progress.set_message("Scanning for oracle usage...");
        progress.enable_steady_tick(Duration::from_millis(100));

        match run_oracle_scan_only(&config) {
            Ok(oracle_report) => {
                progress.finish_with_message("Oracle scan completed.".green().to_string());
                println!();

                OracleDetector::print_report(&oracle_report, config.verbose);

                if config.show_guide {
                    println!();
                    println!("{}", "=== APRO Integration Guide ===".bold().cyan());
                    OracleDetector::print_integration_guide(&oracle_report);
                }
            }
            Err(e) => {
                progress.finish_with_message("Oracle scan failed.".red().to_string());
                eprintln!("{}", e.to_string().red());
                exit(1);
            }
        }
        return;
    }

    // Handle full migration (with oracle detection)
    progress.set_message("Running migration with oracle detection...");
    progress.enable_steady_tick(Duration::from_millis(100));

    match run_migration(&config) {
        Ok(result) => {
            progress.finish_with_message("Migration completed successfully.".green().to_string());

            println!();
            println!("{}", "=== Migration Summary ===".bold().green());

            if result.config_updated {
                println!("{} Anchor.toml configuration updated for SOON Network", "✅".green());
            } else if config.dry_run {
                println!("{} Dry run completed - no files were modified", "ℹ️".blue());
            } else {
                println!("{} No configuration changes needed", "ℹ️".blue());
            }

            // Show oracle report if available
            if let Some(ref oracle_report) = result.oracle_report {
                println!();
                OracleDetector::print_report(oracle_report, config.verbose);

                if config.show_guide && oracle_report.apro_integration_guide.is_some() {
                    println!();
                    println!("{}", "=== APRO Integration Guide ===".bold().cyan());
                    OracleDetector::print_integration_guide(oracle_report);
                }
            }

            // Show warnings if any
            if !result.warnings.is_empty() {
                println!();
                println!("{}", "=== Warnings ===".bold().yellow());
                for warning in &result.warnings {
                    println!("{} {}", "⚠️".yellow(), warning);
                }
            }

            // Show next steps
            println!();
            println!("{}", "=== Next Steps ===".bold().cyan());
            for (i, step) in result.next_steps.iter().enumerate() {
                println!("{}. {}", i + 1, step);
            }

            // Additional helpful information
            if let Some(ref oracle_report) = result.oracle_report {
                if !oracle_report.detected_oracles.is_empty() && !config.show_guide {
                    println!();
                    println!("{}", "💡 Tip: Run with --show-guide to see detailed APRO oracle integration examples".yellow());
                }
            }

            println!();
            println!("{}", "🎉 Migration to SOON Network complete!".bold().green());
            println!("   Visit https://docs.soo.network for more information");
        }
        Err(e) => {
            progress.finish_with_message("Migration failed.".red().to_string());
            eprintln!();
            eprintln!("{}", "❌ Migration failed:".bold().red());
            eprintln!("   {}", e.to_string().red());

            // Provide helpful suggestions based on error type
            match &e {
                MigrationError::NotAnAnchorProject(_) => {
                    eprintln!();
                    eprintln!(
                        "{}",
                        "💡 Make sure you're in an Anchor project directory with:".yellow()
                    );
                    eprintln!("   • Anchor.toml file");
                    eprintln!("   • Cargo.toml file");
                }
                MigrationError::OracleDetectionFailed(_) => {
                    eprintln!();
                    eprintln!(
                        "{}",
                        "💡 Try running with --oracle-only to isolate oracle detection issues"
                            .yellow()
                    );
                }
                _ => {}
            }

            exit(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_info() {
        // Basic compilation test to ensure main module works
        assert_eq!(std::env!("CARGO_PKG_NAME"), "soon-migrate");
        assert_eq!(std::env!("CARGO_PKG_VERSION"), "0.2.0");
    }
}
