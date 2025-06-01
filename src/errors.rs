use thiserror::Error;

/// Represents all possible errors that can occur during migration.
#[derive(Error, Debug)]
pub enum MigrationError {
    /// Failed to create a backup of the Anchor.toml file.
    ///
    /// This typically occurs when there are permission issues or the file is locked.
    #[error("Failed to backup Anchor.toml: {0}")]
    BackupFailed(String),

    /// Failed to read the Anchor.toml file.
    ///
    /// This can happen if the file doesn't exist or is not accessible.
    #[error("Failed to read Anchor.toml: {0}")]
    ReadFailed(String),

    /// Failed to parse the Anchor.toml file.
    ///
    /// This indicates the file exists but contains invalid TOML.
    #[error("Failed to parse Anchor.toml: {0}")]
    TomlParseError(String),

    /// Failed to write to the Anchor.toml file.
    ///
    /// This can occur due to permission issues or disk space problems.
    #[error("Failed to write Anchor.toml: {0}")]
    WriteFailed(String),

    /// The specified backup file was not found.
    ///
    /// This occurs when trying to restore from a non-existent backup.
    #[error("Backup file not found at path: {0}")]
    BackupNotFound(String),

    /// Failed to restore from a backup.
    ///
    /// This can happen if the backup file is corrupted or inaccessible.
    #[error("Failed to restore from backup: {0}")]
    RestoreFailed(String),

    /// The specified path is not a valid Anchor project.
    ///
    /// This error is returned when the expected Anchor project structure is not found.
    #[error("The specified path is not a valid Anchor project: {0}")]
    NotAnAnchorProject(String),

    /// Oracle detection failed.
    ///
    /// This error occurs when there are issues detecting or analyzing oracles.
    #[error("Oracle detection failed: {0}")]
    OracleDetectionFailed(String),
}