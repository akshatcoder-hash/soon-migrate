//! # Soon Migrate
//!
//! A CLI tool for migrating Solana Anchor projects to the SOON Network.
//! 
//! This library provides the core functionality for the `soon-migrate` binary,
//! including configuration management, oracle detection, and migration logic.
//!
//! ## Features
//! - Migration of Solana Anchor project configurations
//! - Oracle detection and analysis
//! - Backup and restore functionality
//! - Detailed reporting and recommendations

#![warn(missing_docs)]
#![forbid(unsafe_code)]
#![warn(rust_2018_idioms)]
#![warn(clippy::all, clippy::pedantic)]

/// Command-line interface configuration and argument parsing.
///
/// This module handles the definition and parsing of command-line arguments,
/// providing a clean interface for the rest of the application to access
/// user-provided configuration.
pub mod cli;

/// Error types used throughout the crate.
///
/// This module defines the error types and error handling utilities used
/// across the application, ensuring consistent error reporting and handling.
pub mod errors;

/// Core migration logic and functionality.
///
/// This module contains the main migration logic, including configuration
/// file parsing, backup/restore functionality, and the core migration process.
pub mod migration;

/// Oracle detection and analysis.
///
/// This module provides functionality to detect and analyze oracle usage
/// in Solana programs, with support for various oracle providers like
/// Pyth, Switchboard, and Chainlink.
pub mod oracle;

/// Re-export commonly used items for easier access
pub use cli::*;
pub use errors::MigrationError;
pub use migration::*;
pub use oracle::*;

/// The current version of the soon-migrate crate.
///
/// This is automatically set from the `Cargo.toml` version field.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
