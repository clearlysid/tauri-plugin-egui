use tauri::{
    plugin::{Builder, TauriPlugin},
    Manager, Runtime,
};

pub use models::*;

// Re-export egui for convenience
pub use egui;


#[cfg(desktop)]
mod desktop;

mod error;
mod models;

pub use error::{Error, Result};

#[cfg(desktop)]
use desktop::Egui;

/// Extensions to [`tauri::App`], [`tauri::AppHandle`] and [`tauri::Window`] to access the egui APIs.
pub trait EguiExt<R: Runtime> {
    fn egui(&self) -> &Egui<R>;
}

impl<R: Runtime, T: Manager<R>> crate::EguiExt<R> for T {
    fn egui(&self) -> &Egui<R> {
        // For now, we'll implement this differently since we can't store in global state
        // This is a temporary implementation
        todo!("Global state access needs different approach")
    }
}

/// Extension trait for Tauri windows to add egui capabilities
pub trait WindowEguiExt<R: Runtime> {
    /// Convert this window to support egui rendering
    fn make_egui<F>(&self, ui_fn: F) -> crate::Result<()>
    where
        F: Fn(&egui::Context) + Send + Sync + 'static;
}

impl<R: Runtime> WindowEguiExt<R> for tauri::Window<R> {
    fn make_egui<F>(&self, ui_fn: F) -> crate::Result<()>
    where
        F: Fn(&egui::Context) + Send + Sync + 'static,
    {
        // For now, just run the UI function once to test the API
        let ctx = egui::Context::default();
        let raw_input = egui::RawInput::default();
        
        let _full_output = ctx.run(raw_input, |ctx| {
            ui_fn(ctx);
        });
        
        println!("Made window '{}' egui-enabled (basic implementation)", self.label());
        Ok(())
    }
}

impl<R: Runtime> WindowEguiExt<R> for tauri::WebviewWindow<R> {
    fn make_egui<F>(&self, ui_fn: F) -> crate::Result<()>
    where
        F: Fn(&egui::Context) + Send + Sync + 'static,
    {
        // For now, just run the UI function once to test the API
        let ctx = egui::Context::default();
        let raw_input = egui::RawInput::default();
        
        let _full_output = ctx.run(raw_input, |ctx| {
            ui_fn(ctx);
        });
        
        println!("Made webview window '{}' egui-enabled (basic implementation)", self.label());
        Ok(())
    }
}

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("egui")
        .invoke_handler(tauri::generate_handler![])
        .setup(|_app, _api| {
            println!("Tauri egui plugin initialized");
            Ok(())
        })
        .build()
}
