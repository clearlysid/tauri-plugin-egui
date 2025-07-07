// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Window;
use tauri_plugin_egui::WindowEguiExt;

use tauri_plugin_egui::egui; // just a re-export for convenience

fn main() {
  tauri::Builder::default()
    .plugin(tauri_plugin_egui::init())
    .setup(|app| {
      // create a window without webview (for now)
      // doing this atm just to avoid visual conflicts but
      // there's no reason why we can't have a webview too
      // note: this requires the `unstable` crate feature in tauri

      let window = Window::builder(app, "main")
        .inner_size(600.0, 400.0)
        .title("egui-tauri demo")
        .build()?;

      // once we have a Window (or WebviewWindow), pass in a
      // closure that will be called to render the egui UI.
      window.make_egui(|ctx| {
        egui::Window::new("Hello from Egui!").show(ctx, |ui| {
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
