# Contributing to soon-migrate

Thank you for your interest in contributing to soon-migrate! We appreciate your time and effort in helping improve this tool.

## Code of Conduct

By participating in this project, you agree to abide by our [Code of Conduct](CODE_OF_CONDUCT.md). Please read it before making any contributions.

## Getting Started

### Prerequisites

- Rust (latest stable version)
- Cargo (Rust's package manager)
- Git

### Setting Up the Development Environment

1. Fork the repository on GitHub
2. Clone your fork locally:
   ```bash
   git clone https://github.com/your-username/soon-migrate.git
   cd soon-migrate
   ```
3. Build the project:
   ```bash
   cargo build
   ```
4. Run tests:
   ```bash
   cargo test
   ```

## How to Contribute

### Reporting Bugs

1. Check if the issue has already been reported in the [GitHub Issues](https://github.com/akshatcoder-hash/soon-migrate/issues)
2. If not, create a new issue with a clear title and description
3. Include steps to reproduce the issue and any relevant logs

### Feature Requests

1. Check if the feature has already been requested
2. Open an issue with a clear description of the feature and its benefits
3. Include any relevant use cases or examples

### Pull Requests

1. Fork the repository and create a new branch for your feature/fix:
   ```bash
   git checkout -b feature/your-feature-name
   ```
2. Make your changes following the code style guidelines
3. Add tests for your changes
4. Update the documentation if necessary
5. Run the test suite and ensure all tests pass:
   ```bash
   cargo test
   ```
6. Commit your changes with a descriptive commit message
7. Push your branch and open a pull request

## Code Style

- Follow the [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Run `cargo fmt` before committing
- Run `cargo clippy` to catch common mistakes and improve your code
- Keep lines under 100 characters when possible

## Testing

- Write unit tests for new functionality
- Ensure all tests pass before submitting a PR
- Add integration tests for new features when appropriate

## Documentation

- Update the README.md for significant changes
- Add documentation comments for all public items
- Keep the CHANGELOG.md up to date

## Review Process

1. A maintainer will review your PR as soon as possible
2. Be prepared to address any feedback or requested changes
3. Once approved, a maintainer will merge your PR

## Areas Needing Help

If you're looking to contribute but not sure where to start, here are some areas that could use attention:

1. **Testing**: More test coverage, especially for edge cases
2. **Documentation**: Improving API documentation and examples
3. **Error Handling**: Enhancing error messages and recovery
4. **CI/CD**: Setting up GitHub Actions for automated testing and releases
5. **Performance**: Optimizing the code for better performance
6. **New Features**: Check the issues labeled "help wanted" or "good first issue"

## License

By contributing to this project, you agree that your contributions will be licensed under its [MIT License](LICENSE).

Thank you for contributing to soon-migrate! 🚀
