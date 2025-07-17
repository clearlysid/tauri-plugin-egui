// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::time::Instant;
use tauri::Window;
use tauri_plugin_egui::{egui, AppHandleExt};

fn main() {
  tauri::Builder::default()
    .setup(|app| {
      // First: initialize it for app using `.wry_plugin()`.
      app.wry_plugin(tauri_plugin_egui::Builder::new(app.handle().to_owned()));

      // Second: create/obtain a Tauri native `Window` (no webview)
      Window::builder(app, "main")
        .inner_size(600.0, 400.0)
        .title("tauri-plugin-egui demo")
        .transparent(true)
        .title_bar_style(tauri::TitleBarStyle::Overlay)
        .build()?;

      app.handle().start_egui_for_window(
        "main",
        Box::new(|ctx| {
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

            ui.separator();

            // Cursor test - hover over different areas to see cursor changes
            ui.label("Hover over different areas to test cursor changes:");
            ui.horizontal(|ui| {
                // Force hand cursor for this button
                if ui.add(egui::Button::new("Click me (hand cursor)")).on_hover_cursor(egui::CursorIcon::PointingHand).clicked() {
                    println!("Button clicked!");
                }
                
                // Force text cursor for this text edit
                ui.add(egui::TextEdit::singleline(&mut "Edit me".to_string()).hint_text("Type here (text cursor)"));
                
                // Force help cursor
                ui.add(egui::Button::new("Help (?)")).on_hover_cursor(egui::CursorIcon::Help);
            });

            ui.separator();

            // Timer demonstration - shows continuous rendering
            static mut START_TIME: Option<Instant> = None;
            unsafe {
              if START_TIME.is_none() {
                START_TIME = Some(Instant::now());
              }
              if let Some(start) = START_TIME {
                let elapsed = start.elapsed().as_secs_f32();
                ui.label(format!("Timer: {:.1}s", elapsed));

                // Request repaint to keep the timer updating
                ctx.request_repaint();
              }
            }
          });
        }),
      )?;

      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
