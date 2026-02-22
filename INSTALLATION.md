# MCZ Launcher - Installation Guide

Complete setup instructions for the Minecraft NeoForge Launcher.

## Quick Start (5 minutes)

### For Pre-Built Releases
1. Download latest release ZIP from GitHub
2. Extract to desired location
3. Run `MCZPlauncher.exe`
4. Enjoy!

### For Linux/WSL Users
See [Linux Setup](#linux--wsl-setup) below.

---

## Detailed Setup

### Option 1: Using Pre-Built Executable (Recommended)

#### Step 1: Download
- Visit the [Releases](https://github.com/yourusername/MCZPlauncher/releases) page
- Download the latest `MCZPlauncher-release.zip`
- Extract to your preferred location (e.g., `C:\Games\MCZ`)

#### Step 2: First Launch
1. Run `MCZPlauncher.exe`
2. The launcher will automatically:
   - Create config directories
   - Load default modpacks
   - Detect your Java installation

#### Step 3: Configure (Optional)
Edit `config.json` to customize:
- RAM allocation
- Java path
- Download preferences

---

### Option 2: Build from Source

#### Prerequisites
- **Windows 7 SP1 or later** (Windows 10/11 recommended)
- **Rust 1.70+** (from [rustup.rs](https://rustup.rs/))
- **Java 11+** (from [java.com](https://www.java.com))
- **Git** (from [git-scm.com](https://git-scm.com))

#### Step 1: Install Rust
```powershell
# Download from https://rustup.rs/ or run:
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Restart PowerShell after installation
```

#### Step 2: Clone Repository
```powershell
git clone https://github.com/yourusername/MCZPlauncher.git
cd MCZPlauncher
```

#### Step 3: Build
```powershell
# Option A: Using build script (Recommended)
.\build.ps1 -Release

# Option B: Using cargo directly
cargo build --release

# Option C: Using batch file (simple)
.\build.bat release
```

#### Step 4: Run
```powershell
# After building:
.\target\release\MCZPlauncher.exe

# Or use build script to build and run:
.\build.ps1 -Release -Run
```

---

## Installation Paths

### Default Locations

| Component | Path |
|-----------|------|
| Game Files | `%LOCALAPPDATA%\MCZPlauncher` |
| Config | `%APPDATA%\MCZPlauncher\config.json` |
| Modpacks | `%LOCALAPPDATA%\MCZPlauncher\modpacks` |

### Custom Installation

1. Set `MINECRAFT_HOME` environment variable
2. Edit `config.json` and change `game_directory`
3. Restart launcher

---

## Java Setup

### Auto-Detection
The launcher automatically searches for Java in:
- System PATH
- Common installation directories
- Windows Registry

### Manual Configuration

If Java isn't detected:

1. Install Java from [java.com](https://www.java.com) or [Adoptium](https://adoptium.net/)
2. Open `config.json`
3. Set `java_path` to your Java executable:
   ```json
   {
     "java_path": "C:\\Program Files\\Java\\jdk-21\\bin\\java.exe"
   }
   ```

### Verifying Java Installation

```powershell
java -version
javac -version
```

---

## Linux / WSL Setup

### Prerequisites
```bash
# Ubuntu/Debian
sudo apt-get update
sudo apt-get install rustc cargo git build-essential libssl-dev pkg-config

# Fedora
sudo dnf install rust cargo git gcc libssl-devel pkg-config

# Arch
sudo pacman -S rust cargo git base-devel openssl pkg-config
```

### Build
```bash
git clone https://github.com/yourusername/MCZPlauncher.git
cd MCZPlauncher
cargo build --release
```

### Run (requires X11/Wayland)
```bash
# For WSL: Use VcXsrv or WSLg https://github.com/microsoft/wslg
./target/release/MCZPlauncher

# Or with GUI forwarding
DISPLAY=:0 ./target/release/MCZPlauncher
```

---

## macOS Setup

### Prerequisites
```bash
# Using Homebrew
brew install rust git

# Apple Silicon (M1/M2/M3)
rustup target add aarch64-apple-darwin
```

### Build
```bash
git clone https://github.com/yourusername/MCZPlauncher.git
cd MCZPlauncher
cargo build --release
```

### Run
```bash
./target/release/MCZPlauncher
```

---

## Troubleshooting

### "Rust not found"
Install from [rustup.rs](https://rustup.rs/) and restart terminal

### "Java not detected"
- Install Java: [java.com](https://www.java.com)
- Or set path manually in config.json

### "Build fails with 'iced' error"
Ensure system dependencies are installed:
```bash
# Ubuntu/Debian
sudo apt-get install libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev

# Or use GitHub Actions (cloud build)
```

### "Launcher crashes on start"
- Check `%APPDATA%\MCZPlauncher\launcher.log`
- Delete config.json to reset to defaults
- Ensure sufficient disk space (min 5GB)

### "Mods won't download"
- Check internet connection
- Verify mod URLs in modpacks.json
- Check firewall/proxy settings

---

## Building for Distribution

### Create Release Package

```powershell
# PowerShell
.\build.ps1 -Release -Package

# Or manually
cargo build --release
Compress-Archive -Path "build\" -DestinationPath "MCZPlauncher-release.zip"
```

This creates:
- Executable
- README
- modpacks.json
- LICENSE
- All dependencies bundled

### Sign Executable (Optional)
```powershell
# Using signtool (Windows SDK required)
signtool sign /f certificate.pfx /p password /t http://timestamp.server.com MCZPlauncher.exe
```

---

## System Requirements

### Minimum
- CPU: Intel/AMD dual-core 2.0+ GHz
- RAM: 8 GB
- Disk: 10 GB free
- Java: 11+
- Windows: 7 SP1+

### Recommended
- CPU: Intel i5/AMD Ryzen 5 or better
- RAM: 16 GB
- Disk: SSD with 15+ GB free
- Java: JDK 21
- Windows: 10/11

---

## Updating

### From Pre-Built Release
1. Download new release
2. Extract to new folder or overwrite old one
3. Run new executable
4. Old config preserved automatically

### From Source
```bash
cd MCZPlauncher
git pull origin main
cargo build --release
```

---

## Uninstallation

1. Delete MCZPlauncher folder
2. (Optional) Remove `%APPDATA%\MCZPlauncher`
3. (Optional) Remove `%LOCALAPPDATA%\MCZPlauncher` for save data

### Keep Save Data
Before uninstalling, backup:
- `%LOCALAPPDATA%\MCZPlauncher\mods`
- `%LOCALAPPDATA%\MCZPlauncher\saves`

---

## Support

- 📖 [Full Documentation](./README.md)
- 🐛 [Report Issues](https://github.com/yourusername/MCZPlauncher/issues)
- 💬 [Ask Questions](https://github.com/yourusername/MCZPlauncher/discussions)
- 📧 Email: support@example.com

---

**Ready to play?** Launch the application and select a modpack! 🎮
