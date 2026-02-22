# Changelog

All notable changes to MCZ Launcher will be documented in this file.

## [Unreleased]

### Features
- [ ] Mod manager UI for adding/removing mods
- [ ] Fabric loader support
- [ ] Instance management (multiple installations)
- [ ] Auto-update checker
- [ ] Crash reporter
- [ ] Server browser integration
- [ ] Profile switching
- [ ] Backup and restore functionality

### Improvements
- [ ] Better error messages and recovery
- [ ] Faster startup time
- [ ] Reduced memory footprint
- [ ] Dark/Light theme toggle
- [ ] Drag-and-drop modpack installation

### Bug Fixes
- [ ] Improve Java detection on Windows

---

## [0.1.0] - 2024-02-22

### Added
- **Initial Release**
  - Modern Windows 11-style UI using Iced
  - Modpack management system
  - Automatic Minecraft download
  - NeoForge loader installation
  - Mod auto-download from modpack list
  - Portable executable distribution
  - JSON-based configuration
  - Async multi-threaded downloads
  - RAM and Java arguments configuration
  - Pre-configured example modpacks (Vanilla Plus, Tech Modpack)

### Technical
- Built with Rust for performance and safety
- Tokio async runtime
- Iced GUI framework
- Cross-platform foundation (Windows stable, Linux/macOS experimental)

---

## Versioning
This project follows [Semantic Versioning](https://semver.org/) (MAJOR.MINOR.PATCH)

- **MAJOR**: Breaking changes
- **MINOR**: New features (backwards compatible)
- **PATCH**: Bug fixes and improvements

---

## Guidelines for Contributors

### Version Bumps
- Patch version: Bug fixes, minor improvements
- Minor version: New features, backwards compatible
- Major version: Breaking changes, major refactor

### Changelog Format
```markdown
## [Version] - YYYY-MM-DD

### Added
- Feature description

### Changed
- Change description

### Fixed
- Bug fix description

### Removed
- Removed feature description
```

---

## Release History

| Version | Date | Status | Notes |
|---------|------|--------|-------|
| 0.1.0 | 2024-02-22 | ✅ Released | Initial public release |

---

For detailed changes, see [git log](https://github.com/yourusername/MCZPlauncher/commits)
