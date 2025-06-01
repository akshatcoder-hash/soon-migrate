# soon-migrate

[![Rust Report Card](https://rust-reportcard.xuri.me/badge/github.com/akshatcoder-hash/soon-migrate)](https://rust-reportcard.xuri.me/report/github.com/akshatcoder-hash/soon-migrate)
[![Crates.io](https://img.shields.io/crates/v/soon-migrate)](https://crates.io/crates/soon-migrate)
[![Downloads](https://img.shields.io/crates/d/soon-migrate)](https://crates.io/crates/soon-migrate)
[![License](https://img.shields.io/crates/l/soon-migrate)](https://crates.io/crates/soon-migrate)
[![Documentation](https://docs.rs/soon-migrate/badge.svg)](https://docs.rs/soon-migrate)

🚀 `soon-migrate` is a CLI tool that helps developers migrate their Solana Anchor projects to the SOON Network. It modifies the `Anchor.toml` configuration file, updates the cluster RPC URL, and provides oracle detection and migration guidance. This tool simplifies the process of upgrading and migrating existing Solana Anchor projects to the SOON ecosystem.

## Features

- 🛠 **Automatic Migration**: Updates the `Anchor.toml` file to migrate from standard Solana clusters to SOON Network.
- 🔍 **Oracle Detection**: Automatically detects and provides migration guidance for various oracle providers (Pyth, Switchboard, Chainlink, etc.).
- 📦 **APRO Integration**: Generates comprehensive guides for integrating with APRO (SOON's oracle solution).
- 📊 **Configuration Management**: Handles cluster and program configurations seamlessly.
- 📦 **Backup & Restore**: Automatically backs up the existing `Anchor.toml` to ensure you can restore it if needed.
- 🔍 **Dry Run Option**: Preview changes before applying them.
- 🗑 **Verbose Logging**: Detailed output to help you understand the migration process.

## Installation

You can install `soon-migrate` using Cargo:

```bash
cargo install soon-migrate
```

## Usage

### Basic Usage

Navigate to the root directory of your Anchor project and run:

```bash
soon-migrate
```

This will migrate your project by modifying `Anchor.toml` and updating the cluster RPC URL to the SOON Network. The tool will also create a backup (`Anchor.toml.bak`) before making any changes.

### Running with a Specific Path

You can specify the path to your Anchor project explicitly:

```bash
soon-migrate /path/to/your/anchor-project
```

### Dry Run Mode (Recommended First Step)

If you want to preview the changes that will be made without modifying the actual files, use the `--dry-run` flag:

```bash
soon-migrate --dry-run
```

This will print out the changes that would be made to `Anchor.toml` without making any modifications.

### Verbose Mode

For more detailed logging about the migration process, use the `--verbose` flag:

```bash
soon-migrate --verbose
```

### Restore from Backup

If you need to revert the changes made by `soon-migrate`, you can restore the backup using the `--restore` flag:

```bash
soon-migrate --restore
```

### Full Command Reference

- **Basic Migration**:
  ```bash
  soon-migrate
  ```
- **Specify Path**:
  ```bash
  soon-migrate /path/to/project
  ```
- **Dry Run**:
  ```bash
  soon-migrate --dry-run
  ```
- **Verbose Logging**:
  ```bash
  soon-migrate --verbose
  ```
- **Restore Backup**:
  ```bash
  soon-migrate --restore
  ```

## Example Workflow

1. **Run a Dry Run** to see what changes will be made:
   ```bash
   soon-migrate --dry-run
   ```

2. **Run the Actual Migration** after reviewing the dry run output:
   ```bash
   soon-migrate
   ```

3. **For Oracle Migration**, review the generated guide:
   ```bash
   soon-migrate --oracle
   ```

4. **Verify** the updated `Anchor.toml` and run your project's tests.

5. If something went wrong, **restore from the backup**:
   ```bash
   soon-migrate --restore
   ```

## How it Works

`soon-migrate` performs the following tasks:

1. **Project Validation**: Ensures the directory is a valid Anchor project with `Anchor.toml` and `Cargo.toml`.
2. **Backup Creation**: Creates a backup of `Anchor.toml` before making any changes.
3. **Network Migration**: Updates the RPC URL in `Anchor.toml` to point to the SOON Network.
4. **Oracle Detection**: Scans your project for oracle usage and provides migration guidance.
5. **Configuration Updates**: Handles program IDs and cluster configurations specific to SOON Network.
6. **Documentation**: Generates detailed migration guides for detected oracles.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request or open an Issue for suggestions, improvements, or bug reports.

### Development Setup

1. **Clone the Repository**:
   ```bash
   git clone https://github.com/akshatcoder-hash/soon-migrate.git
   cd soon-migrate
   ```

2. **Build the Project**:
   ```bash
   cargo build
   ```

3. **Run Tests**:
   ```bash
   cargo test
   ```

4. **Run Linting**:
   ```bash
   cargo clippy -- -D warnings
   cargo fmt --check
   ```

5. **Generate Documentation**:
   ```bash
   cargo doc --no-deps --open
   ```

## License

Licensed under the **MIT License** ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT).

## Contact

- **Author**: Akshat Sharma (akshatsharma0023@outlook.com)
- **GitHub**: [akshatcoder-hash](https://github.com/akshatcoder-hash)

If you have any questions or suggestions, feel free to reach out!

---

Give `soon-migrate` a ⭐ on GitHub if you find it useful!
