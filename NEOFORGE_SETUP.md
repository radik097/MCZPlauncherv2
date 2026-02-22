# MCZ Launcher - NeoForge 1.21.1 Server Setup Guide

Complete guide for setting up and connecting to a NeoForge 1.21.1 server with the MCZ Launcher authentication system.

## Table of Contents

1. [Prerequisites](#prerequisites)
2. [Server Setup](#server-setup)
3. [Authentication Mod Installation](#authentication-mod-installation)
4. [Launcher Configuration](#launcher-configuration)
5. [Testing](#testing)
6. [Troubleshooting](#troubleshooting)

---

## Prerequisites

### System Requirements

- **Java**: Java 21 (required for NeoForge 1.21.1)
- **RAM**: Minimum 4GB for server, 2GB client
- **Disk Space**: 5GB+ for server, 10GB+ for client with mods
- **Network**: Stable internet connection

### Java Installation

#### Windows
```powershell
# Using WinGet (Windows 11)
winget install Oracle.JDK.21

# Or download from https://www.oracle.com/java/technologies/downloads/
```

#### Linux
```bash
# Ubuntu/Debian
sudo apt-get install openjdk-21-jdk

# Fedora
sudo dnf install java-21-openjdk-devel

# Arch
sudo pacman -S jdk21-openjdk
```

#### macOS
```bash
# Using Homebrew
brew install openjdk@21
```

Verify installation:
```bash
java -version
# Should output: openjdk version "21.x.x" or similar
```

---

## Server Setup

### Step 1: Download NeoForge Installer

1. Visit [NeoForge Official Site](https://neoforged.net/)
2. Select version **1.21.1**
3. Download the **Installer** (not Mdk)

### Step 2: Install Server

```bash
# On Windows or Linux
java -jar neoforge-1.21.1-installer.jar --installServer

# You should see:
# [INFO] Installing libraries...
# [INFO] Installing client libraries...
# [INFO] Installing forge...
```

### Step 3: Initial Server Run

```bash
# Windows
run_server.bat

# Linux/Mac
./run.sh
```

First run will generate `server.properties` and `eula.txt`

### Step 4: Accept EULA

Edit `eula.txt`:
```
eula=true
```

### Step 5: Configure Server

Edit `server.properties`:

```properties
#MCZ Server Configuration
server-name=MCZ_NeoForge_Server
server-port=25565

# CRITICAL: Set online-mode to false for custom authentication
online-mode=false

# Performance
difficulty=2
max-players=20
max-world-size=29999984
view-distance=10
simulation-distance=10

# Network
server-port=25565
enable-query=true
query.port=25565

# Features
spawn-protection=16
allow-nether=true
allow-flight=false
pvp=true

# Logging
log-ips=true
```

### Step 6: Start Server

```bash
# Windows
start_server.bat

# Linux
./start.sh

# macOS
./start.sh
```

Server startup output:
```
[...]
[Network] Local query port set to: 25565
[Main/INFO]: Done (X.XXXs)! For help, type "help"
```

---

## Authentication Mod Installation

### Creating the Authentication Mod

Create a lightweight NeoForge mod for authentication handling:

#### Directory Structure
```
MCZAuthMod/
├── src/
│   └── main/
│       ├── java/
│       │   └── com/
│       │       └── mcz/
│       │           └── auth/
│       │               ├── MCZAuthMod.java
│       │               ├── auth/
│       │               │   ├── AuthHandler.java
│       │               │   └── SessionManager.java
│       │               └── event/
│       │                   └── LoginHandler.java
│       └── resources/
│           ├── META-INF/
│           │   └── mods.toml
│           └── pack.mcmeta
├── build.gradle
└── gradle.properties
```

#### mods.toml
```toml
modLoader="javafxmod"
loaderVersion="[1,)"
license="MIT"

[[mods]]
modId="mczauth"
version="1.0.0"
displayName="MCZ Authentication"
description="Server-side authentication for MCZ Launcher"
authors=["MCZ Team"]
credits="MCZ Launcher Contributors"
logoFile="logo.png"

[[dependencies.mczauth]]
    modId="minecraft"
    mandatory=true
    versionRange="[1.21.1,)"
    ordering="NONE"
    side="SERVER"
```

#### MCZAuthMod.java
```java
package com.mcz.auth;

import net.neoforged.api.distmarker.Dist;
import net.neoforged.bus.api.IEventBus;
import net.neoforged.fml.ModLoadingContext;
import net.neoforged.fml.common.Mod;
import net.neoforged.fml.config.ModConfig;
import net.neoforged.fml.event.lifecycle.FMLCommonSetupEvent;

@Mod("mczauth")
public class MCZAuthMod {
    public static final String MODID = "mczauth";

    public MCZAuthMod(IEventBus modBus) {
        modBus.addListener(this::commonSetup);
    }

    private void commonSetup(final FMLCommonSetupEvent event) {
        // Initialize auth system
        SessionManager.init();
    }
}
```

#### AuthHandler.java (Server-side)
```java
package com.mcz.auth.event;

import net.neoforged.api.distmarker.Dist;
import net.neoforged.bus.api.SubscribeEvent;
import net.neoforged.fml.common.Mod;
import net.neoforged.neoforge.event.entity.player.PlayerEvent;
import net.minecraft.server.level.ServerPlayer;

@Mod.EventBusSubscriber(modid = "mczauth", bus = Mod.EventBusSubscriber.Bus.FORGE)
public class LoginHandler {
    
    @SubscribeEvent
    public static void onPlayerLogin(PlayerEvent.PlayerLoggedInEvent event) {
        if (event.getEntity() instanceof ServerPlayer player) {
            String sessionToken = readSessionToken(player);
            if (!validateWithAuthServer(sessionToken)) {
                player.connection.disconnect(Component.literal("Authentication failed"));
            }
        }
    }

    private static String readSessionToken(ServerPlayer player) {
        // Read from launcher config file or NBT data
        // Implementation details
        return "";
    }

    private static boolean validateWithAuthServer(String token) {
        // Call auth backend to verify token
        // Implementation details
        return true;
    }
}
```

### Building the Mod

```bash
# Windows
gradlew.bat build

# Linux/Mac
./gradlew build
```

Output: `build/libs/MCZAuth-1.0.0.jar`

### Installing the Mod

1. Create `mods` folder in server directory
2. Copy `MCZAuth-1.0.0.jar` to `mods/`
3. Restart server

---

## Launcher Configuration

### Update servers.json

Create or edit `%APPDATA%\MCZPlauncher\servers.json`:

```json
{
  "servers": [
    {
      "address": "localhost",
      "port": 25565,
      "name": "Local Server",
      "description": "Local NeoForge server",
      "requires_authentication": true,
      "auth_server_url": "http://localhost:8080",
      "neoforge_version": "0.0.47",
      "minecraft_version": "1.21.1"
    },
    {
      "address": "play.example.com",
      "port": 25565,
      "name": "Community Server",
      "description": "Public MCZ server",
      "requires_authentication": true,
      "auth_server_url": "https://auth.example.com",
      "neoforge_version": "0.0.47",
      "minecraft_version": "1.21.1"
    }
  ]
}
```

### Update auth_config.json

Create or edit `%APPDATA%\MCZPlauncher\auth_config.json`:

```json
{
  "auth_server_url": "http://localhost:8080",
  "server_address": "localhost",
  "server_port": 25565,
  "session_token": null,
  "last_username": null,
  "auto_login": false
}
```

---

## Testing

### Test 1: Server Connectivity

```bash
# Test connection from launcher machine
ping localhost
# or
ping play.example.com
```

### Test 2: Auth Endpoint

```bash
# Register user
curl -X POST http://localhost:8080/api/register \
  -H "Content-Type: application/json" \
  -d '{
    "username": "testplayer",
    "password": "SecurePass123"
  }'

# Expected response:
# {"success": true, "user_uuid": "550e8400-...", "message": "Account created"}
```

### Test 3: Login Endpoint

```bash
# Login
curl -X POST http://localhost:8080/api/login \
  -H "Content-Type: application/json" \
  -d '{
    "username": "testplayer",
    "password": "SecurePass123"
  }'

# Expected response:
# {"success": true, "session_token": "eyJhbGc...", "user_uuid": "550e8400-..."}
```

### Test 4: Launch Game Through Launcher

1. Open MCZ Launcher
2. Click Login tab
3. Enter credentials: `testplayer` / `SecurePass123`
4. Click Login
5. Select modpack and click Launch Game
6. Check if Minecraft starts and connects to server

### Test 5: Check Server Logs

```bash
# Windows - grep equivalent (in PowerShell)
Get-Content latest.log | Select-String "Authentication"

# Linux/Mac
tail -f logs/latest.log | grep Authentication
```

---

## Troubleshooting

### "Failed to connect to server"

**Cause**: Server not running or port blocked

**Solution**:
```bash
# Check server is running
netstat -an | findstr :25565  # Windows
lsof -i :25565  # Linux/Mac

# Check firewall
# Port 25565 must be open (TCP)
```

### "Authentication failed"

**Cause**: Session token invalid or expired

**Solution**:
1. Try logging in again in launcher
2. Check auth server is running
3. Verify auth_config.json has correct URL

### "Player kicked with 'Authentication failed'"

**Cause**: Auth mod not installed on server

**Solution**:
1. Copy MCZAuth mod to server `mods/` folder
2. Restart server
3. Check server logs for mod loading

### "Launcher crashes on startup"

**Cause**: Missing dependencies

**Solution**:
```bash
# Verify Java 21
java -version

# Verify Minecraft launcher installed
ls %LOCALAPPDATA%\MCZPlauncher\minecraft\  # Windows
ls ~/.minecraft/  # Linux/Mac
```

### "Mods not loading"

**Cause**: Wrong NeoForge version

**Solution**:
1. Verify server NeoForge: 1.21.1
2. Verify launcher NeoForge: 1.21.1
3. Check mod compatibility

---

## Advanced Configuration

### Multi-Server Setup

```json
{
  "servers": [
    {
      "address": "server1.example.com",
      "port": 25565,
      "name": "SkyFactory",
      "auth_server_url": "https://auth1.example.com"
    },
    {
      "address": "server2.example.com",
      "port": 25566,
      "name": "Tech Server",
      "auth_server_url": "https://auth2.example.com"
    }
  ]
}
```

### SSH Tunnel for Remote Auth

```bash
# On launcher machine
ssh -L 8080:auth-server:8080 username@server.example.com

# Then connect to localhost:8080
```

### Docker Deployment

```yaml
version: '3'
services:
  neoforge-server:
    image: openjdk:21-jdk
    ports:
      - "25565:25565"
    volumes:
      - ./server:/app/server
    working_dir: /app/server
    command: java -Xmx4G -Xms4G -jar forge-server.jar nogui

  auth-server:
    build: ./auth-backend
    ports:
      - "8080:8080"
    environment:
      DATABASE_URL: postgresql://...
```

---

## Security Notes

⚠️ **Important**: Never share session tokens or passwords

- Store auth_config.json in secure location
- Use HTTPS for auth server in production
- Enable firewall rules for port access
- Regularly update NeoForge and mods
- Monitor server logs for suspicious activity

---

## Support

- 📖 [NeoForge Documentation](https://docs.neoforged.net/)
- 🐛 [Report Issues](https://github.com/yourusername/MCZPlauncher/issues)
- 💬 [Discord Community](https://discord.gg/neoforge)

---

**Happy Gaming on NeoForge 1.21.1!** 🎮
