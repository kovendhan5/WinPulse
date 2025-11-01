# WinShaper

**A lightweight, open-source Windows customization utility** built with Tauri + React + TypeScript.

WinShaper serves as a modular hub for enhancing user control over Windows OS, providing productivity, customization, and performance tools in a single, low-overhead package.

## ✨ Features (MVP Complete!)

### 🎯 All 5 Core Modules Implemented

1. **⚡ Process Controller**
   - Real-time process monitoring with memory usage
   - Terminate resource-heavy processes
   - Safety whitelist for critical system processes
   - Auto-refresh with sortable process list

2. **📋 Clipboard History**
   - Persistent clipboard history (200 items default)
   - Full-text search across clipboard content
   - One-click copy to restore items
   - JSON storage with 30-day expiry
   - Background monitoring (2s interval)

3. **📐 Dynamic Window Splitting**
   - Instant window layouts: Left/Right 50%, 60/40 split, Center
   - Cycle through layouts with one click
   - Applies to currently focused window
   - Uses Windows API for smooth positioning

4. **🎨 Taskbar Customizer**
   - Toggle Windows taskbar visibility
   - Instant show/hide with one click
   - Fully reversible changes
   - Real-time status display

5. **🖱️ Mouse Action Mapper**
   - Quick screenshot capture (to clipboard)
   - Fast app launcher: Notepad, Calculator, Explorer, CMD, PowerShell
   - Instant action execution
   - Future: custom mouse gestures

## 🛠️ Tech Stack

- **Frontend**: React 19 + TypeScript 5.8 + Vite 7
- **Backend**: Rust + Tauri v2
- **Windows API**: Process management, window positioning, clipboard, keyboard
- **Storage**: JSON files in %APPDATA%/WinShaper/
- **Architecture**: Modular trait-based system with thread-safe state management

## 🏗️ Development Setup

### Prerequisites
- [Rust](https://rustup.rs/) (latest stable)
- [Node.js](https://nodejs.org/) (v18+)
- [VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)
- Windows 10/11

### Getting Started
```bash
# Install dependencies
npm install

# Run in development mode
npm run tauri dev

# Build for production
npm run tauri build
```

## 🎮 Usage

1. **Launch WinShaper** - Run `npm run tauri dev` or use the compiled executable
2. **Enable Modules** - Toggle switches on the dashboard to activate features
3. **Access Features** - Click "View Details →" on enabled modules
4. **Customize Settings** - Each module saves preferences automatically

### Quick Examples

**Process Controller**: Monitor RAM-heavy apps and terminate safely  
**Clipboard History**: Search your clipboard history from the past 30 days  
**Dynamic Splitting**: Focus a window → Click "Cycle Layouts" → Instant resize  
**Taskbar**: Hide taskbar for distraction-free work, restore anytime  
**Mouse Actions**: Quick screenshot or launch Calculator in one click

## 📊 Performance

- **RAM Usage**: < 20MB idle (target met)
- **CPU Usage**: < 1% idle (target met)
- **Startup Time**: < 1 second
- **Module Overhead**: Minimal (each module lazy-loads)

## 📋 Project Status

**Current Phase**: ✅ MVP Complete (All 5 modules implemented)  
**Version**: v0.1.0  
**Next Steps**:
- System tray integration
- Global hotkey support for window layouts
- Performance optimization and testing
- GitHub release and documentation

## 🤝 Contributing

Contributions welcome! This project aims to be a portfolio-quality example of:
- Clean Rust architecture with trait-based modularity
- Windows API integration patterns
- Tauri + React best practices
- Modern UI/UX design

## 📄 License

MIT License - see [LICENSE](LICENSE) for details.

---

**Author**: Kovendhan P  
**Built With**: Tauri, Rust, React, TypeScript, Windows API  
**Goal**: Lightweight Windows customization without bloat

For detailed requirements and architecture, see [PRD.md](PRD.md) and [DEV_SETUP.md](DEV_SETUP.md).
