mod plugin;
mod renderer;
mod window;

pub use plugin::{EguiPlugin, EguiPluginBuilder};
pub use window::WindowEguiExt;

// re-export for convenience
pub use egui;
