# tauri-plugin-egui

🚧 **Work in Progress** 🚧

This plugin provides a simple way to render some `egui`-based Rust UI in a Tauri Window (alongside or without a Webview).


## Design / Architecture

1. Uses `egui`, `egui-wgpu` and `wgpu` dependencies
2. Expose some traits on Tauri's `Window`/`WebviewWindow` to set up `egui` context
3. Keeps "bridge" code as minimal as possible

## Development Guide

1. Most plugin source code is in `src/lib.rs` and `src/renderer.rs`
2. The API example usage is in `examples/vanilla/src-tauri/src/main.rs`
3. check all the rust code with `cargo check`
4. run the example app with `bun tauri dev` in the `examples/vanilla` directory

## Progress

- [x] create example app to explore API design
- [x] set up egui context
- [x] make gpu surface and connect to window
- [ ] render a basic egui UI
- [ ] add support for webview windows
- [ ] handle basics like input events, resizing, etc.
- [ ] make rendering backend (wgpu) swappable for glow, etc.


## Goals

For one of my Tauri apps, I need the ability to render `egui` in certain windows. As of today, other supported methods are either outdated/unmaintained or require too much boilerplate code.

[See also](https://github.com/clearlysid/egui-tao?tab=readme-ov-file#goals--motivations)


## References

- [egui + it's subcrates](https://github.com/emilk/egui)
- [official tauri plugins](https://github.com/tauri-apps/plugins-workspace)
- [tauri-egui integration (now unmaintained)](https://github.com/tauri-apps/tauri-egui)
