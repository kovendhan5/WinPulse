# WinShaper Development Setup

## 🚀 Quick Start

### Prerequisites
- **Rust** (latest stable) - [Install from rustup.rs](https://rustup.rs/)
- **Node.js** (v18+) - [Download from nodejs.org](https://nodejs.org/)
- **pnpm** - Already installed globally

### First Run
```bash
# 1. Navigate to project directory
cd K:\Devops\WinShaper

# 2. Install frontend dependencies
npm install

# 3. Run in development mode
npm run tauri dev
```

## 🏗️ Project Architecture

### Frontend (React + TypeScript)
- **Dashboard UI** - Main control panel with module toggles
- **Configuration Management** - Saves settings to %APPDATA%/WinShaper/config.json
- **Modern Design** - Dark theme with responsive grid layout

### Backend (Rust + Tauri)
- **Modular System** - Each feature is a separate module
- **Windows API Integration** - Low-level system access
- **Performance Focused** - Target: <20MB RAM, <1% CPU idle

## 📋 MVP Modules Status

| Module | Status | Description |
|--------|--------|-------------|
| 🔧 **Process Controller** | ✅ Foundation | Monitor/suspend/kill resource hogs |
| 📐 **Dynamic Splitting** | 🏗️ Structure | Custom window layouts (60/40, L/T) |
| 🎨 **Taskbar Customizer** | 🏗️ Structure | Hide/show elements, themes |
| 🖱️ **Mouse Action Mapper** | 🏗️ Structure | Map gestures to actions |
| 📋 **Clipboard History** | 🏗️ Structure | Persistent clipboard with search |

## 🔧 Development Commands

```bash
# Frontend development
npm run dev                 # Start Vite dev server
npm run build               # Build frontend for production
npm run lint                # Run ESLint

# Tauri development  
npm run tauri dev           # Run app in development mode
npm run tauri build         # Build production executable
npm run tauri build --debug # Build with debug symbols

# Rust development
cd src-tauri
cargo check                 # Fast syntax check
cargo test                  # Run unit tests
cargo clippy                # Rust linter
```

## 📁 Key Files

### Configuration
- `src-tauri/tauri.conf.json` - Tauri app configuration
- `src-tauri/Cargo.toml` - Rust dependencies
- `package.json` - Frontend dependencies

### Core Implementation
- `src/App.tsx` - Main dashboard UI
- `src/App.css` - Modern dark theme styling
- `src-tauri/src/lib.rs` - Main Rust entry point
- `src-tauri/src/modules/` - Module implementations

### Documentation
- `PRD.md` - Product Requirements Document
- `README.md` - Project overview
- `DEV_SETUP.md` - This file

## 🎯 Next Development Steps

### Phase 1: Core Infrastructure ✅
- [x] Project setup with Tauri + React + TypeScript
- [x] Module system architecture
- [x] Configuration management
- [x] Modern dashboard UI

### Phase 2: MVP Implementation (Current)
- [ ] Process Controller - Windows API integration
- [ ] Dynamic Splitting - Window management hooks
- [ ] System Tray - Minimize to tray functionality
- [ ] Settings persistence and validation

### Phase 3: Advanced Features
- [ ] Hotkey system
- [ ] Mouse gesture recognition
- [ ] Taskbar API integration
- [ ] Clipboard monitoring

## 🐛 Troubleshooting

### Build Issues
```bash
# Clear cache and rebuild
npm run tauri build --debug
```

### Rust Compilation
```bash
# Update Rust toolchain
rustup update

# Clean and rebuild
cd src-tauri
cargo clean
cargo build
```

### Windows API Issues
- Ensure running with administrator privileges for system-level features
- Some modules require elevated permissions

## 📊 Performance Targets

Based on PRD requirements:
- **RAM Usage**: <20MB idle, <50MB active
- **CPU Usage**: <1% idle
- **Startup Time**: <1 second
- **File Size**: <10MB executable

## 🔒 Security Notes

- Admin elevation only when necessary
- No persistent data without user consent
- Secure config storage in %APPDATA%
- Windows Security compliance

---

**Ready to start coding!** 🚀

Next: Run `npm run tauri dev` to see the dashboard in action.