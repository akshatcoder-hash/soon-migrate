# soon-migrate

🚀 `soon-migrate` is a CLI tool that helps developers migrate their Solana Anchor projects to the SOON Network. It modifies the `Anchor.toml` configuration file and updates the cluster RPC URL to point to the SOON Network, among other tasks. This tool is designed to simplify the process of upgrading and migrating existing Solana Anchor projects.

## Features

- 🛠 **Automatic Migration**: Updates the `Anchor.toml` file to migrate from standard Solana clusters to SOON Network.
- 📾 **Backup & Restore**: Automatically backs up the existing `Anchor.toml` to ensure you can restore it if needed.
- 🔍 **Dry Run Option**: See what changes would be made without applying them.
- 🗑 **Verbose Logging**: Provides detailed output to help you understand the migration process.

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

3. **Verify** the updated `Anchor.toml` and run your project's tests.

4. If something went wrong, **restore from the backup**:

   ```bash
   soon-migrate --restore
   ```

## How it Works

`soon-migrate` performs the following tasks:

1. **Validation**: Ensures that the specified directory is a valid Anchor project with `Anchor.toml` and `Cargo.toml`.
2. **Backup**: Creates a backup of `Anchor.toml` before making changes.
3. **Modification**: Updates the RPC URL in `Anchor.toml` to point to the SOON Network:
   ```
   https://rpc.devnet.soo.network/rpc
   ```
4. **Logging**: Provides detailed progress, error messages, and final instructions.

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

## License

Licensed under the **MIT License** ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT).

## Contact

- **Author**: Akshat Sharma (akshatsharma0023@outlook.com)
- **GitHub**: [akshatcoder-hash](https://github.com/akshatcoder-hash)

If you have any questions or suggestions, feel free to reach out!

---

Give `soon-migrate` a ⭐ on GitHub if you find it useful!
