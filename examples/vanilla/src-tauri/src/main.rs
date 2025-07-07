// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{WebviewUrl, WebviewWindow};
use tauri_plugin_egui::WindowEguiExt;
// use tauri_plugin_egui::WebviewWindowEguiExt;

use tauri_plugin_egui::egui; // just a re-export for convenience

fn main() {
  tauri::Builder::default()
    .plugin(tauri_plugin_egui::init())
    .setup(|app| {
      // create a window without webview (requires `unstable` crate feature in tauri)

      // create/obtain a WebviewWindow from Tauri
      let window = WebviewWindow::builder(app, "main", WebviewUrl::App("index.html".into()))
        .inner_size(600.0, 400.0)
        .title("tauri-plugin-egui demo")
        .title_bar_style(tauri::TitleBarStyle::Overlay)
        .transparent(true)
        .build()?;

      // once we have a Window (or WebviewWindow), pass in a
      // closure that will be called to render the egui UI.
      window.make_egui(|ctx| {
        egui::CentralPanel::default().show(ctx, |ui| {
          ui.add_space(28.0);
          ui.heading("Hello from Egui!");
          ui.label("This is rendered natively with egui!");
          ui.separator();

          if ui.button("Click me").clicked() {
            println!("Egui button clicked!");
          }

          ui.horizontal(|ui| {
            ui.label("Counter:");
            // Note: This is just for demo - in a real app you'd want persistent state
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
