use clap::{Arg, ArgAction, Command};

#[derive(Debug)]
pub struct Config {
    pub path: String,
    pub dry_run: bool,
    pub verbose: bool,
    pub restore: bool,
}

impl Config {
    pub fn new() -> Self {
        let matches = Command::new("soon-migrate")
            .version("0.1.0")
            .author("Your Name <youremail@example.com>")
            .about("Migrates Solana Anchor projects to SOON Network")
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
                    .help("Enable detailed logging")
                    .action(ArgAction::SetTrue),
            )
            .arg(
                Arg::new("restore")
                    .long("restore")
                    .help("Restore from backup")
                    .action(ArgAction::SetTrue),
            )
            .get_matches();

        Config {
            path: matches.get_one::<String>("path").unwrap().to_string(),
            dry_run: matches.get_flag("dry-run"),
            verbose: matches.get_flag("verbose"),
            restore: matches.get_flag("restore"),
        }
    }
}