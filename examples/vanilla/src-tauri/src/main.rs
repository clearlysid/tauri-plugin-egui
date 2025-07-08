// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{WebviewUrl, WebviewWindow, Window};
use tauri_plugin_egui::{egui, EguiPluginBuilder, WindowEguiExt};

fn main() {
  tauri::Builder::default()
    .setup(|app| {
      // First: register the plugin as a `wry_plugin`.
      app.wry_plugin(EguiPluginBuilder::new(app.handle().to_owned()));

      // Second: create/obtain a Tauri `WebviewWindow`
      // let window = WebviewWindow::builder(app, "main", WebviewUrl::App("index.html".into()))
      //   .inner_size(600.0, 400.0)
      //   .title("tauri-plugin-egui demo [with webview]")
      //   .transparent(true)
      //   .build()?;

      // A webview-less `Window` can be made w the `unstable` crate feature
      let only_window = Window::builder(app, "main2")
        .inner_size(600.0, 400.0)
        .title("tauri-plugin-egui demo")
        .transparent(true)
        .build()?;

      // Third: on your Window/WebviewWindow,
      // 1. call `.egui()` on it with a closure that takes an `egui::Context`
      // 2. build your UI using `egui` APIs
      only_window.egui(|ctx| {
        egui::CentralPanel::default()
          // .frame(egui::Frame::none().fill(egui::Color32::default()))
          .show(ctx, |ui| {
            ui.add_space(28.0);
            ui.heading("Hello from Egui!");
            ui.label("This is rendered natively with egui!");
            ui.separator();

            if ui.button("Click me").clicked() {
              println!("Egui button clicked!");
            }

            ui.horizontal(|ui| {
              ui.label("Counter:");
              // Note: just for demo, in a real app you'd want persistent state
              static mut COUNTER: i32 = 0;
              unsafe {
                if ui.button("+").clicked() {
                  COUNTER += 1;
                }
                ui.label(format!("{}", COUNTER));
                if ui.button("-").clicked() {
                  COUNTER -= 1;
                }
              }
            });
          });
      })?;

      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
