mod renderer;

use anyhow::Error;
use renderer::Renderer;
use tauri::{
    plugin::{Builder, TauriPlugin},
    PhysicalSize, Runtime,
};

pub use egui; // re-export for convenience

/// Extension trait for Tauri windows to add egui capabilities
pub trait WindowEguiExt<R: Runtime> {
    /// Convert this window to support egui rendering
    fn egui<F>(&self, ui_fn: F) -> Result<(), Error>
    where
        F: Fn(&egui::Context) + Send + Sync + 'static;
}

impl<R: Runtime> WindowEguiExt<R> for tauri::Window<R> {
    fn egui<F>(&self, ui_fn: F) -> Result<(), Error>
    where
        F: Fn(&egui::Context) + Send + Sync + 'static,
    {
        // Create egui context + gpu + renderer
        let ctx = egui::Context::default();

        let scale_factor = self.scale_factor().unwrap_or(1.0) as f32;
        ctx.set_zoom_factor(scale_factor);

        let PhysicalSize { width, height } = self.inner_size()?;

        // TODO: support other renderers (glow?)
        let mut renderer = tauri::async_runtime::block_on(async {
            Renderer::new(self.clone(), width, height).await
        })?;

        // Things below should ideally run in the window's event loop
        // TODO: figure out how to access tao/wry events for `redraw_requested`

        let raw_input = egui::RawInput::default();

        // Run `ui_fn` (which describes the UI)
        // This function comes from the tauri app itself and runs every frame.
        // The `ctx.run()` method processes the inputs and drawings and returns output:
        // 1. texture info to give to GPU
        // 2. platform_output to handl events like cursor, copy-paste etc.
        // 3. pixels_per_point which is the scale factor for rendering
        let egui::FullOutput {
            textures_delta,
            shapes,
            pixels_per_point,
            // platform_output,
            ..
        } = ctx.run(raw_input, |ctx| {
            ui_fn(&ctx);
        });

        // Converts all the shapes into triangles meshes
        let paint_jobs = ctx.tessellate(shapes, pixels_per_point);

        let screen_descriptor = egui_wgpu::ScreenDescriptor {
            size_in_pixels: [width, height],
            pixels_per_point: pixels_per_point,
        };

        // Finally we render textures, paint jobs, etc. using the GPU
        renderer.render_frame(screen_descriptor, paint_jobs, textures_delta);

        Ok(())
    }
}

impl<R: Runtime> WindowEguiExt<R> for tauri::WebviewWindow<R> {
    fn egui<F>(&self, ui_fn: F) -> Result<(), Error>
    where
        F: Fn(&egui::Context) + Send + Sync + 'static,
    {
        // Create egui context + gpu + renderer
        let ctx = egui::Context::default();

        let scale_factor = self.scale_factor().unwrap_or(1.0) as f32;
        ctx.set_zoom_factor(scale_factor);

        let PhysicalSize { width, height } = self.inner_size()?;

        // TODO: support other renderers (glow?)
        let mut renderer = tauri::async_runtime::block_on(async {
            Renderer::new(self.clone(), width, height).await
        })?;

        // Things below should ideally run in the window's event loop
        // TODO: figure out how to access tao/wry events for `redraw_requested`

        let raw_input = egui::RawInput::default();

        // Run `ui_fn` (which describes the UI)
        // This function comes from the tauri app itself and runs every frame.
        // The `ctx.run()` method processes the inputs and drawings and returns output:
        // 1. texture info to give to GPU
        // 2. platform_output to handl events like cursor, copy-paste etc.
        // 3. pixels_per_point which is the scale factor for rendering
        let egui::FullOutput {
            textures_delta,
            shapes,
            pixels_per_point,
            // platform_output,
            ..
        } = ctx.run(raw_input, |ctx| {
            ui_fn(&ctx);
        });

        // Converts all the shapes into triangles meshes
        let paint_jobs = ctx.tessellate(shapes, pixels_per_point);

        let screen_descriptor = egui_wgpu::ScreenDescriptor {
            size_in_pixels: [width, height],
            pixels_per_point: pixels_per_point,
        };

        // Finally we render textures, paint jobs, etc. using the GPU
        renderer.render_frame(screen_descriptor, paint_jobs, textures_delta);

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
