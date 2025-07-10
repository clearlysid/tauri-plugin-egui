# tauri-plugin-egui

🚧 **Work in Progress** 🚧

This plugin offers a simple way to render some `egui` UI in a Tauri Window (alongside or without a Webview).

<img width="1294" alt="Screenshot 2025-07-07 at 7 19 42 PM" src="https://github.com/user-attachments/assets/c56dcc60-6698-44f5-8941-ff6881e79d93" />

### Example Usage

```rust
fn main() {
  tauri::Builder::default()
    .setup(|app| {
      // First: register the plugin as a `wry_plugin`.
      app.wry_plugin(EguiPluginBuilder::new(app.handle().to_owned()));

      // Second: create a Tauri `WebviewWindow` / `Window`
      Window::builder(app, "main")
        .inner_size(600.0, 400.0)
        .title("tauri-plugin-egui demo")
        .build()?;

      // Third: mark window as an `egui` target using it's label
      // Pass in a closure that gets called to render egui.
      app.handle().start_egui_for_window(
        "main",
        Box::new(|ctx| {
          egui::CentralPanel::default()
            .show(ctx, |ui| {
              ui.heading("Hello from Egui!");
            });
        }),
      )?;

      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
```

## Development Guide

Most of the source code is in `/src`. We use `egui`, `egui-wgpu`, `wgpu` and (currently) a custom Tauri fork to access some lower-level internals. Effort is being made to merge this into the main Tauri codebase. There's an example app in `examples/vanilla` for demonstrating API usage and easy testing.

```shell
# to check and verify everything
cargo check

# to run example app (needs bun installed)
cd examples/vanilla
bun tauri dev
```


### Design / Architecture Notes

1. Uses Tauri's `wry_plugin` system to hook into the event loop (diff from regular Tauri plugins)
2. Tauri maintains control over the Windowing system, `egui` is only used to draw within them.
3. Uses `wgpu` for hardware-accelerated rendering (can potentially add `glow` support later).
4. Minimize maintaining "soft forks" and trying to keep the bridge code as small as possible.

### How it works:

1. Plugin tracks all the windows "marked" as `egui` targets in a thread-safe HashMap.
2. For each such Tauri Window:
  1. we create an egui context, a renderer and a GPU surface.
  2. we intercept all relevant events (like `RedrawRequested`) to drive egui.


## Progress

This is still a very crude prototype. I will continue working on it as I need this for my own app, [Helmer](https://www.helmer.app). Further improvements will be on a best-effort basis.

- [x] create example app to explore API design
- [x] set up egui context
- [x] make gpu surface and connect to window
- [x] render a basic egui UI
- [x] add support for webview windows
- [x] handle basics like input events, resizing, etc.
- [ ] make rendering backend (wgpu) swappable for glow, etc.


## Goals

For one of my Tauri apps, I needed to render `egui` in certain windows. As of today, other methods are either outdated/unmaintained or require too much boilerplate code.

[See also](https://github.com/clearlysid/egui-tao?tab=readme-ov-file#goals--motivations)


## References

- [egui + it's subcrates](https://github.com/emilk/egui)
- [official tauri plugins](https://github.com/tauri-apps/plugins-workspace)
- [tauri-egui integration (now unmaintained)](https://github.com/tauri-apps/tauri-egui)
