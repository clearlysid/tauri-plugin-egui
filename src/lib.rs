mod plugin;
mod renderer;

pub use plugin::{AppHandleEguiExt, EguiPlugin, EguiPluginBuilder};

// re-export for convenience
pub use egui;
