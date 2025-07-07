use tauri::{
    plugin::{Builder, TauriPlugin},
    Runtime,
};

use anyhow::Error;

// Configuration for creating egui windows
#[derive(Debug, Clone)]
pub struct EguiWindowConfig {
    pub title: String,
    pub width: u32,
    pub height: u32,
    pub resizable: bool,
    pub transparent: bool,
}

impl Default for EguiWindowConfig {
    fn default() -> Self {
        Self {
            title: "Egui Window".to_string(),
            width: 800,
            height: 600,
            resizable: true,
            transparent: false,
        }
    }
}

// Re-export egui for convenience
pub use egui;

mod renderer;

use renderer::Renderer;

/// Extension trait for Tauri windows to add egui capabilities
pub trait WindowEguiExt<R: Runtime> {
    /// Convert this window to support egui rendering
    fn make_egui<F>(&self, ui_fn: F) -> Result<(), Error>
    where
        F: Fn(&egui::Context) + Send + Sync + 'static;
}

impl<R: Runtime> WindowEguiExt<R> for tauri::Window<R> {
    fn make_egui<F>(&self, ui_fn: F) -> Result<(), Error>
    where
        F: Fn(&egui::Context) + Send + Sync + 'static,
    {
        // 1. Create egui context
        let ctx = egui::Context::default();
        let raw_input = egui::RawInput::default();

        let inner_size = self.inner_size().unwrap();
        let (width, height) = (inner_size.width, inner_size.height);

        // 2. Create GPU renderer
        // This struct stores the GPU handle and egui-wgpu renderer
        // Ideally this takes triangles and paints them to window
        let mut renderer = tauri::async_runtime::block_on(async {
            Renderer::new(self.clone(), width, height).await
        });

        // 3. Run the UI function (which draws actual UI)
        // This function comes from the tauri app src code
        // And it is supposed to run every frame
        let egui::FullOutput {
            textures_delta,
            shapes,
            pixels_per_point,
            // platform_output,
            ..
        } = ctx.run(raw_input, |ctx| {
            ui_fn(&ctx);
        });

        // The `.run()` function processes all the drawing and returns
        // output from the egui context which contains:
        // - shapes/textures.. to be given to GPU
        // - platform_output.. to handle events like cursor, copy-paste etc.
        // - any scale factor changes etc etc.

        // This tesselate method converts all the shapes into triangles
        let paint_jobs = ctx.tessellate(shapes, pixels_per_point);

        let screen_descriptor = {
            egui_wgpu::ScreenDescriptor {
                size_in_pixels: [width, height],
                pixels_per_point: self.scale_factor().unwrap() as f32,
            }
        };

        // Finally we render textures, paint jobs, etc. using the GPU
        renderer.render_frame(screen_descriptor, paint_jobs, textures_delta);

        println!(
            "Made window '{}' egui-enabled (basic implementation)",
            self.label()
        );
        Ok(())
    }
}

impl<R: Runtime> WindowEguiExt<R> for tauri::WebviewWindow<R> {
    fn make_egui<F>(&self, ui_fn: F) -> Result<(), Error>
    where
        F: Fn(&egui::Context) + Send + Sync + 'static,
    {
        // For now, just run the UI function once to test the API
        let ctx = egui::Context::default();
        let raw_input = egui::RawInput::default();

        let _full_output = ctx.run(raw_input, |ctx| {
            ui_fn(ctx);
        });

        println!(
            "Made webview window '{}' egui-enabled (basic implementation)",
            self.label()
        );
        Ok(())
    }
}

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("tauri-plugin-egui")
        .invoke_handler(tauri::generate_handler![])
        .setup(|_app, _api| {
            println!("tauri-plugin-egui initialized");
            Ok(())
        })
        .build()
}
