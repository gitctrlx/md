# Contributing to Markdown Viewer

Thank you for your interest in contributing to the Markdown Viewer project! This guide will help you get started with development and contribution.

## 🚀 Getting Started

### Prerequisites

- [Rust](https://rustup.rs/) (latest stable version)
- [Node.js](https://nodejs.org/) (v18 or later)
- [Deno](https://deno.land/) (latest version)
- [Tauri CLI](https://tauri.app/v1/guides/getting-started/prerequisites)

### Installation

1. Clone the repository:

   ```bash
   git clone <repository-url>
   cd md
   ```

2. Install dependencies:

   ```bash
   deno install
   ```

3. Install Tauri CLI:

   ```bash
   cargo install tauri-cli
   ```

## 🛠️ Development

### Running the Development Server

```bash
deno task tauri dev
```

This will start both the Vite development server and the Tauri application.

### Building for Production

```bash
deno task tauri build
```

## 🎨 Customizing Application Icons

To update the application icons, follow these steps:

### 1. Prepare Your Icon

- Create a high-quality PNG icon (recommended: 1024x1024 pixels)
- Ensure the icon has a transparent background
- Save it as `static/app-icon.png` in the project root

### 2. Generate All Icon Sizes

Use the Tauri CLI to automatically generate all required icon formats and sizes:

```bash
cargo tauri icon static/app-icon.png
```

This command will:

- Generate all platform-specific icon formats (.ico, .icns, .png)
- Create icons for Windows, macOS, and Linux
- Generate icons for different DPI settings
- Update the `src-tauri/icons/` directory with all necessary files

### 3. Rebuild the Application

After generating new icons, rebuild the application:

```bash
deno task tauri build
```

Thank you for contributing to make Markdown Viewer better for everyone! 🎉
