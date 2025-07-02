
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
