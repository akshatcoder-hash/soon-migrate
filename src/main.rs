use clap::{Arg, ArgAction, Command};
use colored::*;
use indicatif::{ProgressBar, ProgressStyle};
use std::process::exit;
use std::time::Duration;

mod cli;
mod errors;
mod migration;
mod utils;

use cli::Config;
use migration::{restore_backup, run_migration};

fn main() {
    let config = Config::new();

    if config.verbose {
        println!("{}", "Starting soon-migrate...".cyan());
    }

    let progress = ProgressBar::new_spinner();
    progress.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner} {msg}")
            .expect("Failed to create progress style") // Fixing the unwrap issue
            .tick_chars("/|\\- "),
    );

    if config.restore {
        progress.set_message("Restoring from backup...");
        progress.enable_steady_tick(Duration::from_millis(100)); // Using Duration
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

    progress.set_message("Migrating project...");
    progress.enable_steady_tick(Duration::from_millis(100)); // Using Duration

    match run_migration(&config) {
        Ok(_) => {
            progress.finish_with_message("Migration completed successfully.".green().to_string());
            println!("{}", "Migration successful!".green());
            println!("{}", "Next steps:".yellow());
            println!("1. Update your dependencies.");
            println!("2. Test your project.");
            println!("3. Deploy to SOON Network.");
        }
        Err(e) => {
            progress.finish_with_message("Migration failed.".red().to_string());
            eprintln!("{}", e.to_string().red());
            exit(1);
        }
    }
}
