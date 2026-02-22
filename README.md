# MCZ Launcher - Minecraft NeoForge Modpack Manager

A portable, Windows 11-styled Minecraft launcher built in Rust that automatically downloads and manages Minecraft, NeoForge, and modpacks.

## Features

✨ **Modern Windows 11 UI** - Clean, native-looking interface built with Iced
📦 **Automatic Downloads** - Auto-downloads Minecraft, NeoForge, and mods from modpack lists
🎮 **Modpack Management** - Easy selection and configuration of modpacks
⚙️ **Easy Configuration** - JSON-based modpack configuration system
🧵 **Multi-threaded** - Asynchronous downloads and operations
💾 **Portable** - Self-contained application for easy distribution
🔧 **Customizable** - RAM settings, Java arguments, and game configuration

## System Requirements

- Windows 10 or later
- Java 8+ (will auto-detect)
- At least 8 GB disk space for a modpack
- 512 MB RAM minimum for launcher

## Installation

### Option 1: Release Binary (Easiest)
1. Download the latest release from GitHub
2. Extract the ZIP file
3. Run `MCZPlauncher.exe`

### Option 2: Build from Source
1. Install Rust from [rustup.rs](https://rustup.rs/)
2. Clone this repository:
   ```bash
   git clone https://github.com/yourusername/MCZPlauncher.git
   cd MCZPlauncher
   ```
3. Build the launcher:
   ```bash
   cargo build --release
   ```
4. Run the launcher:
   ```bash
   cargo run --release
   ```

## First-Time Setup

1. **Launch the application** - MCZ Launcher will automatically:
   - Create necessary directories
   - Load the default configuration
   - Initialize the modpack list

2. **Select a Modpack** - Click on an available modpack from the list:
   - Vanilla Plus - Vanilla with quality-of-life mods
   - Tech Modpack - Advanced tech and building mods

3. **Configure Settings** (Optional):
   - Adjust RAM allocation (default: 2GB min, 4GB max)
   - Add custom Java arguments
   - Select Java version if multiple are installed

4. **Launch the Game** - Click "Launch Game" to:
   - Download Minecraft (if needed)
   - Download NeoForge
   - Download all required mods
   - Start the game automatically

## Configuration

### Modpack Configuration

Edit or create `modpacks.json` to add custom modpacks:

```json
{
  "modpacks": [
    {
      "name": "My Modpack",
      "version": "1.0.0",
      "minecraft_version": "1.20.1",
      "neoforge_version": "0.0.47",
      "description": "My custom modpack",
      "author": "Your Name",
      "mods": [
        {
          "name": "Mod Name",
          "download_url": "https://example.com/mod.jar",
          "version": "1.0.0",
          "required": true,
          "filename": "mod-1.0.0.jar"
        }
      ],
      "settings": {
        "ram_min": 2048,
        "ram_max": 4096,
        "java_args": "-XX:+UseG1GC",
        "custom_args": null
      }
    }
  ]
}
```

### Launcher Configuration

The launcher creates a config file at:
- `%APPDATA%\AppData\Local\MCZPlauncher\config.json`

Configure:
- RAM allocation
- Java path
- Auto-download preferences
- UI theme and language

## Directory Structure

```
MCZPlauncher/
├── %APPDATA%\Local\MCZPlauncher/
│   ├── config.json              # Launcher configuration
│   ├── minecraft/               # Minecraft installation
│   ├── neoforge/                # NeoForge loader
│   ├── mods/                    # Installed mods
│   ├── libraries/               # Game libraries
│   ├── assets/                  # Game assets
│   └── versions/                # Game versions
└── modpacks.json                # Modpack definitions
```

## Modpack Sources

### CurseForge
- Use CurseForge project IDs to auto-generate download URLs
- Format: `https://www.curseforge.com/api/v1/mods/{project_id}/files`

### Modrinth
- Alternative to CurseForge
- Format: `https://api.modrinth.com/v2/project/{slug}/version`

### GitHub Releases
- Directly link to GitHub release JAR files
- Format: `https://github.com/{owner}/{repo}/releases/download/{tag}/{file}`

## RAM Configuration Guide

- **Minimum RAM**: 1024 MB (not recommended)
- **Low-end Modpacks**: 2048 MB (Vanilla+)
- **Medium Modpacks**: 3072-4096 MB (Tech mods)
- **Heavy Modpacks**: 6144+ MB (100+ mods)

> **Tip**: Set `-Xmx` to 75-80% of your system RAM for optimal performance

## Java Arguments Default

```
-XX:+UseG1GC                      # Use G1GC garbage collector
-XX:+ParallelRefProcEnabled       # Parallel reference processing
-XX:MaxGCPauseMillis=200          # Max GC pause time
-XX:InitiatingHeapOccupancyPercent=35  # GC trigger threshold
-DJ2D.opengl=true                 # Enable OpenGL for better graphics
```

## Troubleshooting

### Issue: "Java not found"
**Solution**: 
- Install Java from [java.com](https://www.java.com)
- Manually set Java path in settings

### Issue: "Failed to download mods"
**Solution**:
- Check internet connection
- Verify mod URLs are correct in modpacks.json
- Check CurseForge API is accessible

### Issue: Game crashes on launch
**Solution**:
- Increase allocated RAM
- Check Java version compatibility
- Review crash log in game directory

### Issue: Slow download speeds
**Solution**:
- Check internet connection
- Use CDN-cached mod sources
- Try again later (API rate limits)

## Advanced Usage

### Create a Custom Modpack

1. Create a JSON file with your mod list
2. Add to `modpacks.json`
3. Include download URLs for all mods
4. Set appropriate RAM requirements

### Portable Installation

1. Build or download release
2. Copy entire MCZPlauncher folder to USB drive
3. Run `MCZPlauncher.exe` - creates local config directory
4. Use on any Windows machine

### Batch Operations

Planned features:
- Bulk mod updates
- Modpack version management
- Automatic cleanup of old versions

## Known Limitations

- Server downloads only (single-player worlds require manual copy)
- NeoForge loader only (no Fabric support currently)
- Windows only (Linux/Mac support planned)
- Manual mod deletion required (no mod manager UI yet)

## Contributing

Contributions welcome! Please:
1. Fork the repository
2. Create a feature branch
3. Submit a pull request

### Development Setup

```bash
# Install dependencies
rustup update

# Run tests
cargo test

# Format code
cargo fmt

# Check code quality
cargo clippy
```

## License

MIT License - See LICENSE file for details

## Credits

- Built with [Iced](https://github.com/iced-rs/iced) GUI framework
- Uses [Tokio](https://tokio.rs/) for async operations
- Inspired by popular launchers like MultiMC and Prism Launcher

## Support

- 📖 [Documentation](./README.md)
- 🐛 [Report Issues](https://github.com/yourusername/MCZPlauncher/issues)
- 💬 [Discussions](https://github.com/yourusername/MCZPlauncher/discussions)

---

**Happy Gaming!** 🎮
