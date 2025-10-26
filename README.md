# WinShaper 🔧

> A lightweight, open-source Windows customization utility

## 🚀 Development Setup

### Prerequisites
- ✅ Rust (installed)
- ✅ Visual Studio Build Tools 2022 (installed)
- 📦 Node.js (recommended for frontend development)

### Quick Start

1. **Build the project:**
   ```bash
   cd src-tauri
   cargo build
   ```

2. **Run in development mode:**
   ```bash
   cargo tauri dev
   ```

## 🎯 Project Goals

WinShaper aims to be a modular hub for Windows customization with:

- **Ultra-low resource footprint** (< 20MB RAM idle)
- **Toggleable modules** for customization
- **Open-source** community-driven development
- **Modern tech stack** (Rust + Tauri)

## 🔧 Features (MVP)

- 🪟 **Dynamic Window Splitting** - Custom layouts and snapping
- 🎨 **Taskbar Customizer** - Hide/show elements, themes
- 🖱️ **Mouse Action Mapper** - Gestures and shortcuts
- ⚡ **Process Controller** - Monitor and manage resources
- 📋 **Clipboard History** - Persistent clipboard with search

## 📁 Project Structure

```
WinShaper/
├── src/                 # Frontend (HTML/CSS/JS)
├── src-tauri/          # Rust backend
│   ├── src/
│   │   └── main.rs     # Main application entry
│   ├── Cargo.toml      # Rust dependencies
│   └── tauri.conf.json # Tauri configuration
├── .gitignore          # Git ignore rules
└── README.md          # This file
```

## 🛠️ Development Status

- ✅ Development environment setup
- ✅ Basic Tauri project structure
- ✅ Frontend layout created
- 🔄 Ready for feature development...

## 📝 License

MIT License - see [LICENSE](LICENSE) file for details.

---

**Built with ❤️ using Rust + Tauri**