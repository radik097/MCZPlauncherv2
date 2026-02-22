# MCZ Launcher - Index & Navigation Guide

Welcome to the MCZ Launcher project! This guide will help you navigate all the files and documentation.

## 🎯 Quick Navigation

### 👤 For End Users
1. **Start Here**: [QUICKSTART.md](./QUICKSTART.md) - Get running in 10 minutes
2. **Detailed Setup**: [INSTALLATION.md](./INSTALLATION.md) - Full installation guide
3. **Main Docs**: [README.md](./README.md) - Complete feature documentation
4. **Having Issues?**: See Troubleshooting section in [README.md](./README.md)

### 👨‍💻 For Developers
1. **Project Overview**: [PROJECT_SUMMARY.md](./PROJECT_SUMMARY.md) - Technical architecture
2. **Contributing**: [CONTRIBUTING.md](./CONTRIBUTING.md) - Development guidelines
3. **Code**: [src/](./src/) - All Rust source code
4. **Build**: Use `cargo build --release`

### 🔐 For Authentication Setup
1. **Architecture**: [AUTHENTICATION.md](./AUTHENTICATION.md) - How auth works
2. **API Specs**: See endpoint documentation in [AUTHENTICATION.md](./AUTHENTICATION.md)
3. **Backend**: Database schema and code examples provided

### 🎮 For Server Setup
1. **Server Guide**: [NEOFORGE_SETUP.md](./NEOFORGE_SETUP.md) - NeoForge 1.21.1 setup
2. **Mod Installation**: Instructions for auth mod in [NEOFORGE_SETUP.md](./NEOFORGE_SETUP.md)
3. **Configuration**: server.properties template included

### 📦 For Deployment
1. **Delivery Package**: [DELIVERY.md](./DELIVERY.md) - What's included and ready
2. **Version Info**: [CHANGELOG.md](./CHANGELOG.md) - Current version 0.1.0
3. **License**: [LICENSE](./LICENSE) - MIT License

---

## 📁 File Structure

```
MCZPlauncher/
│
├─ 📚 DOCUMENTATION (Read These First!)
│  ├─ README.md              ← Start here for overview
│  ├─ QUICKSTART.md          ← 10-minute setup guide
│  ├─ INSTALLATION.md        ← Detailed installation
│  ├─ AUTHENTICATION.md      ← Auth API & architecture
│  ├─ NEOFORGE_SETUP.md      ← Server setup guide
│  ├─ CONTRIBUTING.md        ← Development guide
│  ├─ CHANGELOG.md           ← Version history
│  ├─ PROJECT_SUMMARY.md     ← Technical overview
│  ├─ DELIVERY.md            ← What's included
│  └─ LICENSE                ← MIT License
│
├─ 💻 SOURCE CODE
│  └─ src/
│     ├─ main.rs            (500 lines) - App entry & state
│     ├─ ui_views.rs        (400 lines) - UI screens
│     ├─ auth.rs            (350 lines) - Authentication
│     ├─ server.rs          (250 lines) - Server connection
│     ├─ minecraft.rs       (300 lines) - Game launcher
│     ├─ modpack.rs         (250 lines) - Modpack management
│     ├─ config.rs          (200 lines) - Configuration
│     ├─ download.rs        (150 lines) - Downloads
│     └─ ui.rs              (150 lines) - UI components
│
├─ ⚙️ BUILD & CONFIG
│  ├─ Cargo.toml            - Dependencies & metadata
│  ├─ Cargo.lock            - Locked versions
│  ├─ build.ps1             - PowerShell build script
│  └─ build.bat             - Batch build script
│
├─ 📋 CONFIGURATION TEMPLATES
│  ├─ modpacks.json         - Modpack definitions
│  └─ servers.json          - Server configurations
│
└─ 📂 BUILD OUTPUT (After building)
   └─ target/
      ├─ debug/MCZPlauncher.exe           - Debug build
      └─ release/MCZPlauncher.exe         - Optimized build
```

---

## 🚀 Getting Started Paths

### Path 1: User Setup (Just Want to Play)
```
1. Download/Extract
2. Read: QUICKSTART.md
3. Run: MCZPlauncher.exe
4. Create account → Play!
```
⏱️ **Time**: ~15 minutes

### Path 2: Developer Setup (Want to Modify)
```
1. Install Rust: https://rustup.rs/
2. Clone Repository
3. Read: PROJECT_SUMMARY.md
4. Build: cargo build --release
5. Modify: Edit src/ files
6. Test locally
```
⏱️ **Time**: ~1-2 hours

### Path 3: Server Setup (Want Own Server)
```
1. Read: NEOFORGE_SETUP.md
2. Install Java 21
3. Download NeoForge 1.21.1
4. Install Auth Mod
5. Configure server.properties
6. Start server
7. Configure launcher
```
⏱️ **Time**: ~2-3 hours

### Path 4: Deployment (Want to Release)
```
1. Read: DELIVERY.md
2. Build: cargo build --release
3. Package: Add to ZIP
4. Deploy: Share with users
5. Distribute: Upload to hosting
```
⏱️ **Time**: ~30 minutes

---

## 📖 Documentation Overview

| Document | Length | Audience | Key Topics |
|----------|--------|----------|-----------|
| README.md | 15KB | Everyone | Features, setup, config |
| QUICKSTART.md | 8KB | Users | 10-min quick start |
| INSTALLATION.md | 20KB | Developers | Build from source |
| AUTHENTICATION.md | 25KB | Backend devs | Auth system, API |
| NEOFORGE_SETUP.md | 22KB | Server admins | Server setup, mods |
| CONTRIBUTING.md | 15KB | Contributors | Dev guidelines |
| PROJECT_SUMMARY.md | 18KB | Developers | Architecture, modules |
| DELIVERY.md | 12KB | Deployers | What's ready |
| CHANGELOG.md | 5KB | Everyone | Version history |

**Total Documentation**: ~15,000 words

---

## 🔍 Find What You Need

### Looking for...

**Setup Instructions**
→ [QUICKSTART.md](./QUICKSTART.md) or [INSTALLATION.md](./INSTALLATION.md)

**Feature List**
→ [README.md](./README.md)

**How Authentication Works**
→ [AUTHENTICATION.md](./AUTHENTICATION.md)

**Server Configuration**
→ [NEOFORGE_SETUP.md](./NEOFORGE_SETUP.md)

**Code Architecture**
→ [PROJECT_SUMMARY.md](./PROJECT_SUMMARY.md)

**How to Contribute**
→ [CONTRIBUTING.md](./CONTRIBUTING.md)

**What's Ready to Deploy**
→ [DELIVERY.md](./DELIVERY.md)

**Source Code**
→ [src/](./src/) directory

**Building Instructions**
→ [INSTALLATION.md](./INSTALLATION.md) or [README.md](./README.md)

**Troubleshooting**
→ [README.md](./README.md) - Troubleshooting section

**API Documentation**
→ [AUTHENTICATION.md](./AUTHENTICATION.md) - API Endpoints section

---

## 🎯 Common Tasks

### "I want to run the launcher"
1. Read [QUICKSTART.md](./QUICKSTART.md)
2. Download/Extract
3. Run MCZPlauncher.exe

### "I want to build from source"
1. Install Rust from https://rustup.rs/
2. Clone repository
3. Run: `cargo build --release`
4. Run: `target/release/MCZPlauncher.exe`

### "I want to customize modpacks"
1. Edit `modpacks.json`
2. Add your mod URLs
3. Restart launcher

### "I want to set up my own server"
1. Follow [NEOFORGE_SETUP.md](./NEOFORGE_SETUP.md)
2. Install NeoForge 1.21.1
3. Install auth mod
4. Configure server properties

### "I want to deploy authentication"
1. Read [AUTHENTICATION.md](./AUTHENTICATION.md)
2. Set up database (PostgreSQL recommended)
3. Deploy auth backend (provided template)
4. Update auth_config.json in launcher

### "I want to contribute code"
1. Read [CONTRIBUTING.md](./CONTRIBUTING.md)
2. Fork repository
3. Make changes in new branch
4. Submit pull request

### "I found a bug"
1. Check [README.md](./README.md) Troubleshooting
2. Open issue on GitHub
3. Include error message and system info

### "I want to understand the code"
1. Read [PROJECT_SUMMARY.md](./PROJECT_SUMMARY.md)
2. Look at [src/main.rs](./src/main.rs)
3. Explore other src/ files
4. Check code comments

---

## 📚 Reading Order Recommendations

### For Non-Technical Users
1. [README.md](./README.md) - Features & overview
2. [QUICKSTART.md](./QUICKSTART.md) - Setup guide
3. Direct to launcher

### For Technical Users
1. [README.md](./README.md) - Overview
2. [INSTALLATION.md](./INSTALLATION.md) - Build guide
3. [PROJECT_SUMMARY.md](./PROJECT_SUMMARY.md) - Architecture
4. Source code in [src/](./src/)

### For Server Administrators
1. [README.md](./README.md) - Features
2. [NEOFORGE_SETUP.md](./NEOFORGE_SETUP.md) - Server setup
3. [AUTHENTICATION.md](./AUTHENTICATION.md) - Auth system
4. Configure and deploy

### For Developers Extending Project
1. [CONTRIBUTING.md](./CONTRIBUTING.md) - Guidelines
2. [PROJECT_SUMMARY.md](./PROJECT_SUMMARY.md) - Architecture
3. [src/](./src/) - Read through modules
4. Look at tests and docs
5. Make changes following patterns

---

## 🔗 External Resources

### Official Documentation
- [Rust Book](https://doc.rust-lang.org/book/)
- [Iced GUI Framework](https://github.com/iced-rs/iced)
- [Tokio Async Runtime](https://tokio.rs/)
- [NeoForge](https://neoforged.net/)
- [Minecraft Launcher API](https://wiki.vg/)

### Tools You'll Need
- [Rust & Cargo](https://rustup.rs/)
- [Git](https://git-scm.com/)
- [Java 21](https://www.oracle.com/java/technologies/downloads/)

### Community
- [GitHub Issues](https://github.com/yourusername/MCZPlauncher/issues)
- [GitHub Discussions](https://github.com/yourusername/MCZPlauncher/discussions)
- [NeoForge Discord](https://discord.gg/neoforge)

---

## 💡 Tips

### For Faster Reading
- **TL;DR sections** available in most docs
- **Troubleshooting** sections have quick answers
- **Code examples** show practical usage

### For Better Understanding
- Read architecture section in [PROJECT_SUMMARY.md](./PROJECT_SUMMARY.md)
- Look at related source files
- Check comments in code
- Test locally as you learn

### For Fastest Setup
- Use [QUICKSTART.md](./QUICKSTART.md) (10 minutes)
- Pre-built binary is fastest
- Building from source takes ~3 minutes

---

## ✅ Checklist for First-Time Users

- [ ] Read [README.md](./README.md) for overview
- [ ] Download or build launcher
- [ ] Run MCZPlauncher.exe
- [ ] See login screen
- [ ] Create test account
- [ ] View main launcher
- [ ] Read configuration section
- [ ] Try launching (will download assets)
- [ ] Verify game launches
- [ ] Customize modpack if desired

---

## 🆘 Getting Help

### If you're stuck:
1. Check [README.md](./README.md) Troubleshooting
2. Read relevant guide (INSTALLATION, AUTHENTICATION, etc.)
3. Search GitHub issues
4. Open new GitHub issue with:
   - What you tried
   - What happened
   - What you expected
   - System info (Windows version, Java version, etc.)

### For specific topics:
- **Setup**: [QUICKSTART.md](./QUICKSTART.md)
- **Errors**: [README.md](./README.md) → Troubleshooting
- **Auth**: [AUTHENTICATION.md](./AUTHENTICATION.md)
- **Server**: [NEOFORGE_SETUP.md](./NEOFORGE_SETUP.md)
- **Code**: [PROJECT_SUMMARY.md](./PROJECT_SUMMARY.md)

---

## 📞 Support Channels

1. **Documentation** - These 10+ files with 15,000+ words
2. **Code Comments** - Well-commented source
3. **GitHub Issues** - Bug reports and features
4. **GitHub Discussions** - Questions and ideas
5. **README.md** - Troubleshooting section

---

## 🎉 You're Ready!

Everything you need is here:
- ✅ **Code** - Complete and ready
- ✅ **Documentation** - Comprehensive
- ✅ **Examples** - In modpacks.json and code
- ✅ **Guides** - Step-by-step instructions

**Pick your path above and get started!**

---

## 📄 Document Sizes & Reading Times

| Document | Size | Read Time |
|----------|------|-----------|
| README.md | 15KB | 20 min |
| QUICKSTART.md | 8KB | 10 min |
| INSTALLATION.md | 20KB | 25 min |
| AUTHENTICATION.md | 25KB | 30 min |
| NEOFORGE_SETUP.md | 22KB | 30 min |
| CONTRIBUTING.md | 15KB | 20 min |
| PROJECT_SUMMARY.md | 18KB | 25 min |
| DELIVERY.md | 12KB | 15 min |
| CHANGELOG.md | 5KB | 5 min |

**Total**: 140KB / ~180 minutes (3 hours)

Choose which docs are relevant to you!

---

## 🏁 TL;DR (Too Long; Didn't Read)

**I just want to play:**
→ Run MCZPlauncher.exe (read [QUICKSTART.md](./QUICKSTART.md) first)

**I want to build it:**
→ Follow [INSTALLATION.md](./INSTALLATION.md) → Run `cargo build --release`

**I want to set up a server:**
→ Follow [NEOFORGE_SETUP.md](./NEOFORGE_SETUP.md)

**I want to modify the code:**
→ Read [PROJECT_SUMMARY.md](./PROJECT_SUMMARY.md) then explore [src/](./src/)

**I want to contribute:**
→ Follow [CONTRIBUTING.md](./CONTRIBUTING.md)

---

**Start here**: Pick your path above and dive in! 🚀
