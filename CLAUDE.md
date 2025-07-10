# CLAUDE.md

## Project Overview
This is a Tauri plugin that integrates egui (immediate mode GUI) with Tauri applications. It allows rendering native egui interfaces directly in Tauri windows using WGPU for hardware-accelerated rendering.

## Development Commands

### Checking the Plugin
```bash
cargo check
```

### Running the Example
```bash
cd examples/vanilla/src-tauri
bun tauri dev
```

## Architecture

### Core Components

**Plugin Structure (`src/lib.rs`):**
- `EguiWindowConfig`: Configuration for egui windows
- `WindowEguiExt`: Extension trait that adds `.make_egui()` method to Tauri windows
- `init()`: Plugin initialization function

**Rendering Pipeline (`src/renderer.rs`):**
- `Renderer`: Main rendering coordinator that manages GPU resources and egui rendering
- `Gpu`: WGPU abstraction handling surface, device, queue, and configuration
- Uses `egui_wgpu::Renderer` for the actual egui-to-GPU translation

### Key Architecture Concepts

1. **Window Extension Pattern**: The plugin extends Tauri windows with egui capabilities via the `WindowEguiExt` trait
2. **GPU Rendering**: Uses WGPU for hardware-accelerated rendering with egui-wgpu as the bridge

### Dependencies
- `egui`: Core immediate mode GUI library
- `egui-wgpu`: Bridge between egui and WGPU
- `wgpu`: Modern graphics API abstraction
- `tauri`: Cross-platform app framework
- `tauri-runtime` and `tauri-runtime-wry`: Low-level event access

## Example Usage Pattern
See `examples/vanilla/src-tauri/src/main.rs` for usage pattern where a window is created and then converted to egui rendering.

## Reference Crates and Documentation
- For a comprehensive complete integration, refer to [egui-winit](https://github.com/emilk/egui/tree/main/crates/egui-winit)
- It supports resizing, input handling, and continuous rendering loops.

## To-do's that are yet to be addressed
- Only single-frame rendering (no continuous render loop)
- No input handling
- No window resizing support
