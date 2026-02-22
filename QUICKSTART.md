# MCZ Launcher - Quick Start Guide

Get the MCZ Launcher running in 10 minutes!

## What is MCZ Launcher?

A modern, portable Minecraft launcher built in Rust for NeoForge 1.21.1, featuring:
- ✅ Windows 11-style beautiful UI
- ✅ Automatic Minecraft & NeoForge downloads
- ✅ Modpack management from modpack lists
- ✅ Server-side authentication system
- ✅ 100% portable (no installation needed)

## 5-Minute Setup

### Option 1: Download & Play (Easiest)

1. **Download**: Get latest release from [Releases](https://github.com/yourusername/MCZPlauncher/releases)
2. **Extract**: Unzip to any folder
3. **Run**: Double-click `MCZPlauncher.exe`
4. **Play**: Create account → Select modpack → Launch!

### Option 2: Build from Source (5 min)

```bash
# 1. Install Rust (2 min)
# Visit https://rustup.rs/ and follow instructions

# 2. Clone & Build (3 min)  
git clone https://github.com/yourusername/MCZPlauncher.git
cd MCZPlauncher
cargo build --release

# 3. Run
.\target\release\MCZPlauncher.exe
```

## First Launch

### 1. Create Account

```
Username: your_minecraft_name
Password: strong_password_8+ chars
Click: Register
```

### 2. Select Modpack

Choose from available modpacks:
- **Vanilla Plus** - Vanilla with quality improvements
- **Tech Modpack** - Tech and building mods

### 3. Select Server

Pick a server to connect to:
- **Local Server** - localhost:25565
- **Community Server** - play.example.com

### 4. Launch!

Click "Launch Game" - launcher will:
1. Download Minecraft 1.21.1
2. Download NeoForge 0.0.47
3. Download selected mods
4. Start the game

Game launches in ~2-5 minutes on first run, faster on subsequent launches.

## Features Overview

### 📦 Modpack Manager
- Pre-configured modpacks with mod lists
- Auto-download all mods from modpack list
- Add custom modpacks via JSON

### 🔐 Authentication
- Create secure accounts
- Server-side password validation
- Session tokens for game launch

### 🎮 Game Launcher
- Automatic Minecraft download & installation
- NeoForge 1.21.1 support
- Mod management
- Customizable RAM allocation (2-6GB recommended)

### ⚙️ Configuration
- Edit `%APPDATA%\MCZPlauncher\config.json`
- Customize RAM, Java path, and game settings
- Per-modpack settings

## Folder Structure

```
Your_Computer/
├── MCZPlauncher.exe          ← Run this
├── README.md                 ← Documentation
├── modpacks.json             ← Modpack definitions
└── %APPDATA%\MCZPlauncher/
    ├── config.json           ← Your settings
    ├── auth_config.json      ← Auth server URL
    ├── minecraft/            ← Game installation
    ├── neoforge/             ← Mod loader
    └── mods/                 ← Downloaded mods
```

## Common Tasks

### Change RAM Allocation

Edit `config.json`:
```json
{
  "default_ram_mb": 3072,
  "max_ram_mb": 6144
}
```

### Add Custom Server

Edit `servers.json` entry:
```json
{
  "address": "yourserver.com",
  "port": 25565,
  "name": "My Server",
  "auth_server_url": "https://auth.yourserver.com"
}
```

### Create Modpack

1. Create `custom_modpack.json`:
```json
{
  "name": "My Modpack",
  "mods": [
    {
      "name": "MyMod",
      "download_url": "https://example.com/mod.jar"
    }
  ]
}
```

2. Add to `modpacks.json`
3. Restart launcher

## Troubleshooting

### "Java not found"
- Install Java 21: https://java.com/
- Or set path: Edit config.json → `java_path`

### "Failed to download"
- Check internet connection
- Verify mod URLs are correct
- Check firewall

### "Game crashes"
- Increase RAM in config.json
- Check Java version (needs 21+)
- Check mod compatibility

### "Can't connect to server"
- Verify server address and port
- Check firewall allows port 25565
- Ensure auth server is accessible

## Getting Help

- 📖 [Full Documentation](./README.md)
- 🔐 [Auth Setup Guide](./AUTHENTICATION.md)
- 🎮 [NeoForge Setup](./NEOFORGE_SETUP.md)
- 📦 [Installation Guide](./INSTALLATION.md)
- 🐛 [Report Bugs](https://github.com/yourusername/MCZPlauncher/issues)

## Next Steps

1. ✅ Download/Build launcher
2. ✅ Create account and select modpack  
3. ✅ Launch game and enjoy!
4. 📖 Read full docs for advanced features
5. 🌐 Join community server
6. 💻 Create custom modpack

## System Requirements

| Item | Minimum | Recommended |
|------|---------|------------|
| OS | Windows 10 | Windows 11 |
| RAM | 8GB | 16GB |
| Disk | 15GB | 25GB+ |
| Java | 21 | Latest 21 |
| Internet | Required | Broadband |

## Pro Tips

💡 **Tip 1**: First launch takes 5-10 min (downloading assets), but subsequent launches are instant

💡 **Tip 2**: Set `auto_login: true` in config to skip login screen next time

💡 **Tip 3**: Close launcher with `keep_launcher_open: false` when game starts

💡 **Tip 4**: Keep modpacks.json updated for new mods and versions

💡 **Tip 5**: Use `-Xmx` value at 75% of your RAM for best performance

## System Information

Need help? Include this info with bug reports:
```
Windows Version: [Run: Win + R → winver]
Java Version: [Run: java -version]
Launcher Version: [Check: MCZPlauncher.exe properties]
Modpack: [Selected in launcher]
Error Message: [From launcher status]
```

---

**Ready to play? Launch MCZPlauncher.exe now!** 🚀

For support: Open an issue on GitHub or join our Discord community.
