use thiserror::Error;

#[derive(Error, Debug)]
pub enum MigrationError {
    #[error("Failed to backup Anchor.toml: {0}")]
    BackupFailed(String),

    #[error("Failed to read Anchor.toml: {0}")]
    ReadFailed(String),

    #[error("Failed to parse Anchor.toml: {0}")]
    TomlParseError(String),

    #[error("Failed to write Anchor.toml: {0}")]
    WriteFailed(String),

    #[error("Backup file not found at path: {0}")]
    BackupNotFound(String),

    #[error("Failed to restore from backup: {0}")]
    RestoreFailed(String),

    #[error("The specified path is not a valid Anchor project: {0}")]
    NotAnAnchorProject(String),

    #[error("Oracle detection failed: {0}")]
    OracleDetectionFailed(String),

    #[error("Failed to read oracle configuration: {0}")]
    OracleConfigError(String),

    #[error("Failed to generate oracle migration guide: {0}")]
    OracleGuideGenerationFailed(String),

    #[error("Unsupported oracle type detected: {0}")]
    UnsupportedOracleType(String),
}