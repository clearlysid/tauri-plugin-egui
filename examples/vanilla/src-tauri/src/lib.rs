use tauri::Manager;
use tauri_plugin_egui::{EguiExt, WindowEguiExt};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .plugin(tauri_plugin_egui::init())
    .setup(|app| {
      // Get the main window and make it egui-enabled
      let window = app.get_webview_window("main").unwrap();

      window.make_egui(|ctx| {
        tauri_plugin_egui::egui::Window::new("Hello from Egui!").show(ctx, |ui| {
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

      println!("Egui window setup complete!");

      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
