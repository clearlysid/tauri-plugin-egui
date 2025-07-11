// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Window;
use tauri_plugin_egui::{egui, EguiAppHandleExt, EguiPluginBuilder};

fn main() {
  tauri::Builder::default()
    .setup(|app| {
      // First: register the plugin as a `wry_plugin`.
      app.wry_plugin(EguiPluginBuilder::new(app.handle().to_owned()));

      let monitor = app.handle().primary_monitor().unwrap().unwrap();

      let m_sf = monitor.scale_factor();
      let m_size = monitor.size().to_logical(m_sf);
      let m_pos = monitor.position().to_logical(m_sf);

      // Second: create/obtain a Tauri native `Window` (no webview)
      let window = Window::builder(app, "main")
        .position(m_pos.x, m_pos.y)
        .inner_size(m_size.width, m_size.height)
        .title("clipper dummy")
        .transparent(true)
        .decorations(false)
        .always_on_top(true)
        .skip_taskbar(true)
        .resizable(false)
        .shadow(false)
        .build()?;

      let mut drawing = false;
      let mut cropping = false;
      let mut recording = false;
      let mut lines: Vec<Vec<egui::Pos2>> = Default::default();
      let stroke = egui::Stroke::new(8.0, egui::Color32::from_rgb(250, 160, 120));

      app.handle().start_egui_for_window(
        "main",
        Box::new(move |ctx| {
          egui::Window::new("notch")
            .frame(
              egui::Frame::default()
                .fill(egui::Color32::BLACK)
                .inner_margin(12.0),
            )
            .anchor(egui::Align2::CENTER_BOTTOM, (0.0, 0.0))
            .title_bar(false)
            .auto_sized()
            .show(ctx, |ui| {
              ui.horizontal_wrapped(|ui| {
                if ui.toggle_value(&mut recording, "record").clicked() {
                  cropping = false;
                  println!("start recording");
                }
                ui.separator();
                if ui.button("stream 1").clicked() {
                  println!("prepare stream 1");
                }
                if ui.button("stream 2").clicked() {
                  println!("prepare stream 2");
                }
                if ui.button("stream 3").clicked() {
                  println!("prepare stream 3");
                }
                if ui.button("+").clicked() {
                  println!("add stream");
                }
                ui.separator();
                if ui.toggle_value(&mut cropping, "crop").clicked() {
                  drawing = false;
                }
                if ui.toggle_value(&mut drawing, "draw").clicked() {
                  cropping = false;
                  if drawing {
                    lines.clear();
                  }
                }
              });
            });

          egui::Window::new("Camera")
            .frame(
              egui::Frame::default()
                .fill(egui::Color32::from_rgb(10, 10, 10))
                .rounding(egui::Rounding::same(150.0))
                .inner_margin(50.0),
            )
            .title_bar(false)
            .resizable(false)
            .default_pos((100.0, 100.0))
            .show(ctx, |ui| {
              ui.set_width(200.0);
              ui.set_height(200.0);
              ui.label("<camera preview>");

              // let cursor_pos = window.cursor_position().unwrap();

              // ui.label(format!("cursor pos x: {}", cursor_pos.x));
              // ui.label(format!("cursor pos y: {}", cursor_pos.y));
            });

          if drawing {
            egui::CentralPanel::default()
              .frame(egui::Frame::none())
              .show(ctx, |ui| {
                egui::Frame::none().show(ui, |ui| {
                  let (mut response, painter) =
                    ui.allocate_painter(ui.available_size_before_wrap(), egui::Sense::drag());

                  let to_screen = egui::emath::RectTransform::from_to(
                    egui::Rect::from_min_size(egui::Pos2::ZERO, response.rect.square_proportions()),
                    response.rect,
                  );
                  let from_screen = to_screen.inverse();

                  if lines.is_empty() {
                    lines.push(vec![]);
                  }

                  let current_line = lines.last_mut().unwrap();

                  if let Some(pointer_pos) = response.interact_pointer_pos() {
                    let canvas_pos = from_screen * pointer_pos;
                    if current_line.last() != Some(&canvas_pos) {
                      current_line.push(canvas_pos);
                      response.mark_changed();
                    }
                  } else if !current_line.is_empty() {
                    lines.push(vec![]);
                    response.mark_changed();
                  }

                  let shapes = lines.iter().filter(|line| line.len() >= 2).map(|line| {
                    let points: Vec<egui::Pos2> = line.iter().map(|p| to_screen * *p).collect();
                    egui::Shape::line(points, stroke)
                  });

                  painter.extend(shapes);

                  response
                });
              });
          }

          // here you decide if you want to be passthrough or not.
          if ctx.wants_pointer_input() || ctx.wants_keyboard_input() {
            window.set_ignore_cursor_events(false).unwrap();
          } else {
            window.set_ignore_cursor_events(true).unwrap();
          }
        }),
      )?;

      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
