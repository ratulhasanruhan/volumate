# 🎚️ Volumate

Control your Windows PC's volume from any device on your network - your phone, tablet, or another computer!

## 📥 Download & Installation

**[Download Volumate](#)** *(download link will be added here)*

1. Download `volumate.exe`
2. Double-click to run
3. That's it! No installation needed.

> **Note**: Windows may show a security warning since this is a new app. Click "More info" and "Run anyway" to proceed.

## 🚀 How to Use

### Step 1: Start Volumate
- Run `volumate.exe` on your Windows PC
- A window will appear showing your local network address (e.g., `http://192.168.1.100:8080`)

### Step 2: Connect from Any Device
- On your phone, tablet, or another computer, open a web browser
- Type in the address shown in the Volumate window
- Make sure your device is on the same WiFi network as your PC

### Step 3: Control Your Volume!
- **Slide** to adjust volume precisely
- **Quick buttons** (0%, 25%, 50%, 75%) for instant presets
- **Mute/Unmute** buttons for quick silence
- Changes happen instantly!

## 💡 Use Cases

- 🛋️ Control your PC from the couch while watching movies
- 🎮 Adjust game volume from your phone without alt-tabbing
- 🎵 Change music volume from across the room
- 📺 Quick mute during video calls

## ❓ Troubleshooting

### Can't connect from my phone?
- Make sure both devices are on the **same WiFi network**
- Check that Windows Firewall isn't blocking the connection
- Try typing the IP address exactly as shown in the Volumate window

### "No audio sessions found" error?
- Start playing some audio on your PC (YouTube, Spotify, etc.)
- Then try using Volumate again

### Windows blocks the app?
- This is normal for new apps
- Click "More info" → "Run anyway"
- Volumate is safe and doesn't require special permissions

---

## 👨‍💻 For Developers

### Building from Source

**Prerequisites:**
- Rust (latest stable)
- Windows OS
- Cargo (included with Rust)

**Build Commands:**
```bash
# Development build
cargo build

# Optimized release build
cargo build --release

# Run directly
cargo run --release
```

The executable will be in `target/release/volumate.exe`

### Project Structure

```
volumate/
├── src/
│   ├── main.rs        # Server & GUI logic
│   └── volume.rs      # Windows audio control
├── static/
│   └── index.html     # Web interface
└── Cargo.toml         # Dependencies
```

### Tech Stack

- **Backend**: Rust with Actix-web
- **Audio Control**: winmix (Windows Audio Session API)
- **GUI**: native-windows-gui
- **Web UI**: Vanilla HTML/CSS/JavaScript

### API Reference

#### Get Current Volume
```http
GET /api/volume
```

**Response:**
```json
{
  "success": true,
  "volume": 0.5,
  "muted": false,
  "message": null
}
```

#### Set Volume
```http
POST /api/volume
Content-Type: application/json

{
  "level": 0.75
}
```

**Parameters:**
- `level` (float): Volume level from 0.0 to 1.0

#### Set Mute
```http
POST /api/mute
Content-Type: application/json

{
  "muted": true
}
```

**Parameters:**
- `muted` (boolean): true to mute, false to unmute

### Configuration

The server runs on port `8080` by default. To change this, modify the port in `src/main.rs`:

```rust
HttpServer::new(...)
    .bind(("0.0.0.0", 8080))?  // Change port here
```

### Dependencies

Key dependencies (see `Cargo.toml` for full list):
- `actix-web` - Web framework
- `actix-files` - Static file serving
- `winmix` - Windows audio control
- `native-windows-gui` - GUI framework
- `serde` & `serde_json` - JSON serialization

### Contributing

Contributions are welcome! Some ideas:
- [ ] System tray icon with minimize to tray
- [ ] Start with Windows option
- [ ] Per-application volume control
- [ ] Dark mode toggle
- [ ] Custom port configuration in GUI
- [ ] Password protection option
- [ ] Volume lock feature
- [ ] Mobile app versions

### Building for Distribution

```bash
# Build optimized release
cargo build --release

# The executable will be at:
# target/release/volumate.exe
```

For smaller file size, add to `Cargo.toml`:
```toml
[profile.release]
opt-level = "z"
lto = true
codegen-units = 1
strip = true
```

### License

Open source project using the following main dependencies:
- winmix (Windows audio control)
- actix-web (Web framework)
- native-windows-gui (GUI framework)

See individual dependency licenses for details.

---

**Made with 🎵 for easy remote volume control**
