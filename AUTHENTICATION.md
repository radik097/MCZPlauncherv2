# MCZ Launcher - Authentication System

Complete guide for setting up server-side authentication for the MCZ Launcher with NeoForge 1.21.1 support.

## Overview

The MCZ Launcher uses a client-server authentication architecture:

```
┌─────────────────────┐
│  MCZ Launcher       │
│  (Rust Client)      │
└──────────┬──────────┘
           │ HTTP/REST
           │
┌──────────▼──────────┐
│  Auth Backend       │
│  (Rust/Node/Python) │
└──────────┬──────────┘
           │
┌──────────▼──────────┐
│  Database           │
│  (PostgreSQL, etc)  │
└─────────────────────┘
```

## Authentication Flow

### 1. Registration
```
User enters credentials
        ↓
Client validates locally
        ↓
POST /api/register
        ↓
Server hashes password (bcrypt)
        ↓
Server stores user in database
        ↓
Return user_uuid
```

### 2. Login
```
User enters credentials
        ↓
POST /api/login
        ↓
Server retrieves user by username
        ↓
Server verifies password hash
        ↓
Server generates JWT session token
        ↓
Return session_token + expiry
```

### 3. Game Launch
```
Session verified
        ↓
Generate login credentials file
        ↓
  Launch Minecraft with:
  - username
  - session_token
  - server_address
  ↓
NeoForge mod intercepts login
        ↓
Mod sends session_token to server
        ↓
Server validates token
        ↓
Allow player to join
```

---

## API Endpoints

### POST /api/register
Register a new account

**Request:**
```json
{
  "username": "player_name",
  "password": "secure_password",
  "email": "email@example.com"
}
```

**Response (Success):**
```json
{
  "success": true,
  "message": "Account created successfully",
  "user_uuid": "550e8400-e29b-41d4-a716-446655440000"
}
```

**Response (Error):**
```json
{
  "success": false,
  "message": "Username already exists",
  "user_uuid": null
}
```

**Status Codes:**
- 201: Created
- 400: Invalid input
- 409: Username exists

---

### POST /api/login
Authenticate user and receive session token

**Request:**
```json
{
  "username": "player_name",
  "password": "secure_password"
}
```

**Response (Success):**
```json
{
  "success": true,
  "session_token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
  "error": null,
  "user_uuid": "550e8400-e29b-41d4-a716-446655440000"
}
```

**Response (Error):**
```json
{
  "success": false,
  "session_token": null,
  "error": "Invalid credentials",
  "user_uuid": null
}
```

**Status Codes:**
- 200: OK
- 401: Unauthorized
- 404: User not found

---

### POST /api/verify
Verify session token validity

**Headers:**
```
Authorization: Bearer <session_token>
```

**Response (Valid):**
```json
{
  "valid": true,
  "username": "player_name",
  "expires_at": 1708612800
}
```

**Response (Invalid):**
```json
{
  "valid": false,
  "message": "Token expired or invalid"
}
```

**Status Codes:**
- 200: Valid
- 401: Invalid/Expired

---

### POST /api/logout
Invalidate session token

**Headers:**
```
Authorization: Bearer <session_token>
```

**Response:**
```json
{
  "success": true,
  "message": "Logged out successfully"
}
```

---

## Configuration

### Launcher Config (.minecraft/auth_config.json)

```json
{
  "auth_server_url": "https://auth.example.com",
  "server_address": "play.example.com",
  "server_port": 25565,
  "session_token": null,
  "last_username": "player_name",
  "auto_login": false
}
```

### Environment Variables

```bash
# Auth server
AUTH_SERVER=http://localhost:8080
DATABASE_URL=postgresql://user:pass@localhost/mcz_auth
JWT_SECRET=your_secret_key_here

# Server info
MINECRAFT_SERVER_PORT=25565
NEOFORGE_VERSION=0.0.47
```

---

## Example Backend Implementation

### Rust Backend (Axum)

```rust
use axum::{
    extract::Json,
    http::StatusCode,
    routing::post,
    Router,
};
use serde::{Deserialize, Serialize};
use bcrypt::{hash, verify};

#[derive(Debug, Serialize, Deserialize)]
struct LoginRequest {
    username: String,
    password: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct LoginResponse {
    success: bool,
    session_token: Option<String>,
    error: Option<String>,
}

async fn login(
    Json(payload): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, StatusCode> {
    // 1. Query user from database
    let user = find_user(&payload.username)
        .await
        .ok_or(StatusCode::NOT_FOUND)?;

    // 2. Verify password
    let valid = verify(&payload.password, &user.password_hash)
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    if !valid {
        return Ok(Json(LoginResponse {
            success: false,
            session_token: None,
            error: Some("Invalid credentials".to_string()),
        }));
    }

    // 3. Generate JWT token
    let token = generate_jwt(&user.uuid)?;

    Ok(Json(LoginResponse {
        success: true,
        session_token: Some(token),
        error: None,
    }))
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/api/login", post(login));

    axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
```

### Database Schema (PostgreSQL)

```sql
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    uuid UUID UNIQUE NOT NULL,
    username VARCHAR(16) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    email VARCHAR(255),
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE sessions (
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL REFERENCES users(id),
    token VARCHAR(500) UNIQUE NOT NULL,
    expires_at TIMESTAMP NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_users_username ON users(username);
CREATE INDEX idx_sessions_token ON sessions(token);
CREATE INDEX idx_sessions_expires_at ON sessions(expires_at);
```

---

## NeoForge Mod Integration

### Client-Side: Intercept Login

The NeoForge mod intercepts the player login process and validates the session using your auth backend:

```java
@Mod.EventBusSubscriber(modid = MOD_ID, bus = Mod.EventBusSubscriber.Bus.FORGE)
public class AuthenticationHandler {
    @SubscribeEvent
    public static void onServerConnected(ServerChatEvent event) {
        // Get session token from launcher config
        String sessionToken = getSessionToken();
        
        // Send to server for validation
        validateSession(sessionToken);
        
        // If validation fails, kick player
        if (!isValid) {
            event.getPlayer().disconnect();
        }
    }
}
```

### Server-Side: Validate Session

The NeoForge server mod receives the session token and validates it:

```java
ServerPlayConnectionEvents.INIT.register((handler, server) -> {
    String username = handler.getPlayer().getName().getString();
    String sessionToken = getTokenFromClient(username);
    
    if (!validateWithAuthServer(sessionToken)) {
        handler.disconnect(Component.literal("Authentication failed"));
    }
});
```

---

## Security Best Practices

### Password Security
- ✅ Hash all passwords with bcrypt (minimum cost factor 12)
- ✅ Never store plaintext passwords
- ✅ Use HTTPS for all API communication
- ✅ Validate passwords on server-side only

### Token Security
- ✅ Use JWT with RS256 (asymmetric) for tokens
- ✅ Set reasonable expiration times (12-24 hours)
- ✅ Rotate tokens periodically
- ✅ Invalidate tokens on logout
- ✅ Use secure, httpOnly cookies if possible

### API Security
- ✅ Implement rate limiting (prevent brute force)
- ✅ Add CORS restrictions
- ✅ Validate all input data
- ✅ Use API keys for mod authentication
- ✅ Log all authentication attempts

### Account Security
- ✅ Implement account lockout after failed attempts
- ✅ Add optional 2FA support
- ✅ Force password change on first login
- ✅ Send login notifications via email
- ✅ Allow account recovery via email

---

## Troubleshooting

### "Login Failed"
1. Check auth server is running
2. Verify connectivity: `curl http://auth-server/api/health`
3. Check database connection
4. Review server logs

### "Session Token Invalid"
1. Verify token format in request headers
2. Check token expiration time
3. Ensure correct JWT secret is used
4. Check token hasn't been revoked

### "Player Kicked: Authentication Failed"
1. Verify session token is valid
2. Check NeoForge mod is installed on server
3. Verify mod can reach auth server
4. Check firewall doesn't block requests

### "Username Already Exists"
1. Allow users to recover lost accounts
2. Implement username change feature
3. Consider email-based login as alternative

---

## Testing

### Test Login Endpoint

```bash
# Register
curl -X POST http://localhost:8080/api/register \
  -H "Content-Type: application/json" \
  -d '{"username":"testuser","password":"SecurePass123"}'

# Login
curl -X POST http://localhost:8080/api/login \
  -H "Content-Type: application/json" \
  -d '{"username":"testuser","password":"SecurePass123"}'

# Verify Session
curl -X POST http://localhost:8080/api/verify \
  -H "Authorization: Bearer <token_here>"
```

---

## Deployment

### Docker Setup

```dockerfile
FROM rust:latest

WORKDIR /app
COPY . .

RUN cargo build --release

EXPOSE 8080
CMD ["./target/release/mcz-auth-server"]
```

### Docker Compose

```yaml
version: '3'
services:
  postgres:
    image: postgres:15
    environment:
      POSTGRES_DB: mcz_auth
      POSTGRES_USER: mcz
      POSTGRES_PASSWORD: secure_password
    volumes:
      - postgres_data:/var/lib/postgresql/data

  auth-server:
    build: .
    ports:
      - "8080:8080"
    environment:
      DATABASE_URL: postgresql://mcz:secure_password@postgres/mcz_auth
      JWT_SECRET: your_secret_here
    depends_on:
      - postgres

volumes:
  postgres_data:
```

---

## Next Steps

1. **Set up auth backend** using provided Rust template
2. **Configure database** with schema provided
3. **Test API endpoints** using curl commands
4. **Install NeoForge mods** for authentication
5. **Configure launcher** with auth server URL
6. **Deploy to production** using Docker

---

For questions or support, open an issue on GitHub or check the main README.md
