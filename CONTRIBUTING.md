# Contributing to MCZ Launcher

Thank you for your interest in contributing! This document provides guidelines and instructions for contributing.

## Code of Conduct

- Be respectful and inclusive
- Focus on the code, not the person
- Help each other learn and improve
- No harassment or discrimination

## Ways to Contribute

### Report Bugs
1. Check [existing issues](https://github.com/yourusername/MCZPlauncher/issues)
2. Create a detailed bug report including:
   - Steps to reproduce
   - Expected behavior
   - Actual behavior
   - System information (Windows version, Java version, etc.)
   - Error logs (from `%APPDATA%\MCZPlauncher\launcher.log`)

### Suggest Features
1. Open a [Discussion](https://github.com/yourusername/MCZPlauncher/discussions)
2. Describe the feature and use case
3. Discuss implementation ideas with maintainers

### Improve Documentation
- Fix typos
- Clarify instructions
- Add examples
- Update outdated info

### Submit Code Changes

#### Setup Development Environment

```bash
# Clone repository
git clone https://github.com/yourusername/MCZPlauncher.git
cd MCZPlauncher

# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Verify installation
cargo --version
rustc --version
```

#### Development Workflow

1. **Create a branch**
   ```bash
   git checkout -b feature/your-feature-name
   # or
   git checkout -b fix/issue-description
   ```

2. **Make changes**
   ```bash
   # Edit files as needed
   # Run tests frequently
   cargo test
   ```

3. **Format and lint**
   ```bash
   # Format code
   cargo fmt

   # Check for issues
   cargo clippy
   ```

4. **Commit changes**
   ```bash
   git add .
   git commit -m "Brief description of changes"
   # Use conventional commits: feat:, fix:, docs:, etc.
   ```

5. **Push and create PR**
   ```bash
   git push origin feature/your-feature-name
   ```
   Then create a Pull Request on GitHub

#### Commit Message Format

Follow [Conventional Commits](https://www.conventionalcommits.org/):

```
type(scope): description

[optional body]

[optional footer]
```

Types:
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation
- `style`: Code style (formatting)
- `refactor`: Code refactoring
- `perf`: Performance improvement
- `test`: Adding tests
- `chore`: Build, dependencies, etc.

Examples:
```
feat(ui): add settings panel for RAM configuration
fix(download): handle network timeouts gracefully
docs(readme): update installation instructions
```

---

## Development Guide

### Project Structure

```
src/
├── main.rs         # Application entry point
├── ui.rs           # UI components
├── download.rs     # Download manager
├── modpack.rs      # Modpack configuration
├── minecraft.rs    # Game launcher logic
└── config.rs       # Configuration management
```

### Key Files

| File | Purpose |
|------|---------|
| `Cargo.toml` | Dependencies and build config |
| `src/main.rs` | Main application loop |
| `modpacks.json` | Modpack definitions |

### Running Tests

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_name

# Run with output
cargo test -- --nocapture

# Run with multiple threads
cargo test -- --test-threads=4
```

### Building

```bash
# Debug build (faster compile, slower execution)
cargo build

# Release build (slower compile, faster execution)
cargo build --release

# Using build script
./build.ps1 -Release
```

### Running Locally

```bash
# From source
cargo run

# Released executable
./target/release/MCZPlauncher.exe

# With arguments
cargo run -- --debug
```

---

## Code Style

Follow Rust conventions:
- Use `cargo fmt` before committing
- Use `cargo clippy` for linting
- Write meaningful variable names
- Document public APIs with doc comments
- Keep functions focused and small

### Example

```rust
/// Downloads a mod from the provided URL
/// 
/// # Arguments
/// * `url` - The download URL
/// * `dest_path` - Destination file path
/// 
/// # Returns
/// `Ok(PathBuf)` on success, `Err` with description on failure
///
/// # Example
/// ```
/// download_mod("https://example.com/mod.jar", "./mods")?;
/// ```
pub async fn download_mod(
    &self,
    url: &str,
    dest_path: &PathBuf,
) -> Result<PathBuf, Box<dyn std::error::Error>> {
    // Implementation
}
```

---

## Pull Request Process

1. **Before submitting:**
   - [ ] Tests pass: `cargo test`
   - [ ] Code formatted: `cargo fmt`
   - [ ] No warnings: `cargo clippy`
   - [ ] Documentation updated
   - [ ] CHANGELOG.md updated

2. **PR Description:**
   - Explain what changes were made
   - Reference related issues (#123)
   - Include before/after screenshots for UI changes
   - List any breaking changes

3. **Review process:**
   - Maintainers will review within 7 days
   - Address feedback and update PR
   - CI/CD checks must pass
   - At least one approval required

4. **After merge:**
   - Your contribution is included in next release
   - You'll be credited in CHANGELOG

---

## Release Process

The maintainer handles releases:

1. Update version in `Cargo.toml`
2. Update `CHANGELOG.md`
3. Create and tag release commit
4. Build release binaries
5. Create GitHub release
6. Upload artifacts
7. Announce on social media

---

## Questions?

- 📖 [Documentation](./README.md)
- 💬 [GitHub Discussions](https://github.com/yourusername/MCZPlauncher/discussions)
- 🐛 [Issue Tracker](https://github.com/yourusername/MCZPlauncher/issues)

---

Thank you for contributing! 🎉
