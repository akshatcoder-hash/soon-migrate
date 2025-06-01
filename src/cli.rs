use clap::{Arg, ArgAction, Command};

#[derive(Debug)]
pub struct Config {
    pub path: String,
    pub dry_run: bool,
    pub verbose: bool,
    pub restore: bool,
    pub show_guide: bool,
    pub oracle_only: bool,
}

impl Config {
    pub fn new() -> Self {
        let matches = Command::new("soon-migrate")
            .version("0.2.0")
            .author("Akshat Sharma <akshatsharma0023@outlook.com>")
            .about("Migrates Solana Anchor projects to SOON Network with oracle detection")
            .arg(
                Arg::new("path")
                    .help("Path to the Anchor project")
                    .default_value(".")
                    .index(1),
            )
            .arg(
                Arg::new("dry-run")
                    .long("dry-run")
                    .help("Show changes without applying them")
                    .action(ArgAction::SetTrue),
            )
            .arg(
                Arg::new("verbose")
                    .long("verbose")
                    .short('v')
                    .help("Enable detailed logging")
                    .action(ArgAction::SetTrue),
            )
            .arg(
                Arg::new("restore")
                    .long("restore")
                    .help("Restore from backup")
                    .action(ArgAction::SetTrue),
            )
            .arg(
                Arg::new("show-guide")
                    .long("show-guide")
                    .help("Show detailed APRO integration guide")
                    .action(ArgAction::SetTrue),
            )
            .arg(
                Arg::new("oracle-only")
                    .long("oracle-only")
                    .help("Only scan for oracles, don't perform migration")
                    .action(ArgAction::SetTrue),
            )
            .get_matches();

        Config {
            path: matches.get_one::<String>("path").unwrap().to_string(),
            dry_run: matches.get_flag("dry-run"),
            verbose: matches.get_flag("verbose"),
            restore: matches.get_flag("restore"),
            show_guide: matches.get_flag("show-guide"),
            oracle_only: matches.get_flag("oracle-only"),
        }
    }
}