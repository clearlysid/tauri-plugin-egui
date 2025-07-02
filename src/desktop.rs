use parking_lot::Mutex;
use serde::de::DeserializeOwned;
use std::collections::HashMap;
use std::sync::Arc;
use std::marker::PhantomData;
use tauri::{plugin::PluginApi, AppHandle, Runtime, Window, WebviewWindow};

pub fn init<R: Runtime, C: DeserializeOwned>(
    app: &AppHandle<R>,
    _api: PluginApi<R, C>,
) -> crate::Result<Egui<R>> {
    Ok(Egui {
        app: app.clone(),
        egui_windows: Arc::new(Mutex::new(HashMap::new())),
    })
}

/// Represents an egui-enabled window with its context and rendering state
pub struct EguiWindow<R: Runtime> {
    pub context: egui::Context,
    pub window_label: String,
    pub ui_fn: Option<Arc<dyn Fn(&egui::Context) + Send + Sync>>,
    _phantom: PhantomData<R>,
    // TODO: Add wgpu rendering state here
}

impl<R: Runtime> EguiWindow<R> {
    pub fn new(window_label: String) -> Self {
        Self {
            context: egui::Context::default(),
            window_label,
            ui_fn: None,
            _phantom: PhantomData,
        }
    }
    
    pub fn set_ui_fn<F>(&mut self, ui_fn: F)
    where
        F: Fn(&egui::Context) + Send + Sync + 'static,
    {
        self.ui_fn = Some(Arc::new(ui_fn));
    }
}

/// Access to the egui APIs.
pub struct Egui<R: Runtime> {
    app: AppHandle<R>,
    egui_windows: Arc<Mutex<HashMap<String, EguiWindow<R>>>>,
}

impl<R: Runtime> Egui<R> {
    /// Convert an existing Tauri window to support egui rendering
    pub fn make_window_egui<F>(&self, window: &Window<R>, ui_fn: F) -> crate::Result<()>
    where
        F: Fn(&egui::Context) + Send + Sync + 'static,
    {
        let label = window.label().to_string();
        
        // Create new egui-enabled window
        let mut egui_window = EguiWindow::<R>::new(label.clone());
        egui_window.set_ui_fn(ui_fn);
        
        // Store the egui window
        self.egui_windows.lock().insert(label.clone(), egui_window);
        
        // TODO: Set up wgpu surface and rendering for this window
        
        println!("Made window '{}' egui-enabled", label);
        
        Ok(())
    }

    /// Convert an existing Tauri webview window to support egui rendering
    pub fn make_webview_window_egui<F>(&self, webview_window: &WebviewWindow<R>, ui_fn: F) -> crate::Result<()>
    where
        F: Fn(&egui::Context) + Send + Sync + 'static,
    {
        let label = webview_window.label().to_string();
        
        // Create new egui-enabled window
        let mut egui_window = EguiWindow::<R>::new(label.clone());
        egui_window.set_ui_fn(ui_fn);
        
        // Store the egui window
        self.egui_windows.lock().insert(label.clone(), egui_window);
        
        // TODO: Set up wgpu surface and rendering for this webview window
        
        println!("Made webview window '{}' egui-enabled", label);
        
        Ok(())
    }

    /// Check if a window has egui enabled
    pub fn has_egui_window(&self, label: &str) -> bool {
        self.egui_windows.lock().contains_key(label)
    }

    /// Render a frame for all egui-enabled windows
    pub fn render_frame(&self) -> crate::Result<()> {
        let mut windows = self.egui_windows.lock();
        
        for (label, egui_window) in windows.iter_mut() {
            if let Some(ui_fn) = &egui_window.ui_fn {
                // Create a raw input for the frame
                let raw_input = egui::RawInput::default();
                
                // Run the egui frame properly
                let full_output = egui_window.context.run(raw_input, |ctx| {
                    ui_fn(ctx);
                });
                
                // TODO: Render the output using wgpu
                // For now, we just ignore the output since we don't have rendering set up yet
                let _ = full_output;
                
                println!("Rendered egui frame for window '{}'", label);
            }
        }
        
        Ok(())
    }

    /// Get the egui context for a window
    pub fn get_context(&self, window_label: &str) -> Option<egui::Context> {
        let windows = self.egui_windows.lock();
        windows.get(window_label).map(|w| w.context.clone())
    }
}
