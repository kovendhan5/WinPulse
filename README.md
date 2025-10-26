# WinShaper

**A lightweight, open-source Windows customization utility** built with Tauri + React + TypeScript.

WinShaper serves as a modular hub for enhancing user control over Windows OS, providing productivity, customization, and performance tools in a single, low-overhead package.

## 🚀 Features (MVP)

- **Dynamic Window Splitting** - Custom screen layouts (60/40, L/T shapes) with hotkey cycling
- **Taskbar Customizer** - Hide/show elements, resize icons, theme selection
- **Mouse Action Mapper** - Map clicks/gestures to actions (screenshots, app launch)
- **Process Controller** - Monitor, suspend, or kill resource-heavy processes
- **Clipboard History** - Persistent history with search and quick paste

## 🛠️ Tech Stack

- **Frontend**: React + TypeScript + Vite
- **Backend**: Rust + Tauri
- **Architecture**: Modular design with toggleable components
- **Target**: Windows 10/11 (< 20MB RAM idle, < 1s startup)

## 🏗️ Development Setup

### Prerequisites
- [Rust](https://rustup.rs/)
- [Node.js](https://nodejs.org/)
- [VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)

### Getting Started
```bash
# Install dependencies
pnpm install

# Run in development mode
pnpm tauri dev

# Build for production
pnpm tauri build
```

## 📋 Project Status

**Current Phase**: MVP Development (v1.0)  
**Target Launch**: 4 weeks from project start  
**Goal**: 100+ GitHub stars in first month

## 📄 License

MIT License - see [LICENSE](LICENSE) for details.

---

For detailed requirements and architecture, see [PRD.md](PRD.md).
