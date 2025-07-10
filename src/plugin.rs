use anyhow::Error;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use tauri::{AppHandle, Manager, PhysicalSize, Runtime};
use tauri_runtime::UserEvent;

use tauri_runtime_wry::{Context, PluginBuilder};
use tauri_runtime_wry::{EventLoopIterationContext, Message, Plugin, WebContextStore};

use tauri_runtime_wry::tao::event::{Event, WindowEvent as TaoWindowEvent};
use tauri_runtime_wry::tao::event_loop::{ControlFlow, EventLoopProxy, EventLoopWindowTarget};

use crate::renderer::Renderer;

// Calling `.wry_plugin()` requires a `.build()` method that returns a plugin
pub struct EguiPluginBuilder<R: Runtime> {
    app: AppHandle<R>,
}

impl<R: Runtime> EguiPluginBuilder<R> {
    pub fn new(app: AppHandle<R>) -> Self {
        Self { app }
    }
}

type EguiWindowMap = Arc<Mutex<HashMap<String, EguiWindow>>>;

impl<T: UserEvent, R: Runtime> PluginBuilder<T> for EguiPluginBuilder<R> {
    type Plugin = EguiPlugin<T>;

    fn build(self, _context: Context<T>) -> Self::Plugin {
        let managed_windows: EguiWindowMap = Arc::new(Mutex::new(HashMap::new()));

        self.app.manage(managed_windows.clone());

        EguiPlugin::new(managed_windows)
    }
}

/// Meant to be stored in the Tauri state in the AppHandle.
/// has a map of window labels to their EguiWindow instances.
// #[derive(Clone)]
pub struct EguiPlugin<T: UserEvent> {
    windows: EguiWindowMap,
    _phantom: std::marker::PhantomData<T>,
}

unsafe impl<T: UserEvent> Send for EguiPlugin<T> {}
unsafe impl<T: UserEvent> Sync for EguiPlugin<T> {}

impl<T: UserEvent> EguiPlugin<T> {
    pub fn new(windows: EguiWindowMap) -> Self {
        Self {
            windows,
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<T: UserEvent> Plugin<T> for EguiPlugin<T> {
    fn on_event(
        &mut self,
        event: &Event<Message<T>>,
        _event_loop: &EventLoopWindowTarget<Message<T>>,
        _proxy: &EventLoopProxy<Message<T>>,
        _control_flow: &mut ControlFlow,
        context: EventLoopIterationContext<'_, T>,
        _web_context: &WebContextStore,
    ) -> bool {
        match event {
            Event::WindowEvent {
                event, window_id, ..
            } => {
                // determine label of window
                let mapped_id = context
                    .window_id_map
                    .get(window_id)
                    .expect("window id not found");
                let mw = context.windows.0.borrow();
                let label = mw.get(&mapped_id).expect("window not found").label.clone();

                let windows = self.windows.lock().unwrap();

                if let Some(egui_win) = windows.get(&label) {
                    match event {
                        TaoWindowEvent::Resized(size) => {}
                        &_ => {
                            println!("this event isn't handled yet");
                        }
                    }
                }
            }
            Event::RedrawRequested(window_id) => {
                println!("🎨 RedrawRequested for window: {:?}", window_id);

                // determine label of window
                let mapped_id = context
                    .window_id_map
                    .get(window_id)
                    .expect("window id not found");
                let mw = context.windows.0.borrow();
                let label = mw.get(&mapped_id).expect("window not found").label.clone();

                let mut windows = self.windows.lock().unwrap();

                if let Some(egui_win) = windows.get_mut(&label) {
                    // Get the egui context from the EguiWindow

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
                    } = egui_win.context.run(raw_input, |ctx| {
                        egui::CentralPanel::default()
                            // .frame(egui::Frame::none().fill(egui::Color32::default()))
                            .show(ctx, |ui| {
                                ui.add_space(28.0);
                                ui.heading("Hello from Egui!");
                                ui.label("This is rendered natively with egui!");
                                ui.separator();
                            });
                    });

                    // Converts all the shapes into triangles meshes
                    let paint_jobs = egui_win.context.tessellate(shapes, pixels_per_point);

                    let width = egui_win.size.width;
                    let height = egui_win.size.height;

                    let screen_descriptor = egui_wgpu::ScreenDescriptor {
                        size_in_pixels: [width, height],
                        pixels_per_point: pixels_per_point,
                    };

                    // Finally we render textures, paint jobs, etc. using the GPU
                    egui_win
                        .renderer
                        .render_frame(screen_descriptor, paint_jobs, textures_delta);
                }
            }
            &_ => {}
        }

        // Return false to let other plugins/handlers process the event
        false
    }
}

/// An egui context, renderer and (optional) UI function
/// Each of these are needed to render egui UIs in a window
pub struct EguiWindow {
    context: egui::Context,
    renderer: Renderer,
    size: PhysicalSize<u32>,
    ui_fn: Box<dyn FnMut(&egui::Context)>,
}

unsafe impl Send for EguiWindow {}
unsafe impl Sync for EguiWindow {}

pub trait AppHandleEguiExt {
    fn start_egui_for_window(
        &self,
        label: &str,
        ui_fn: Box<dyn FnMut(&egui::Context)>,
    ) -> Result<(), Error>;
}

impl AppHandleEguiExt for AppHandle {
    fn start_egui_for_window(
        &self,
        label: &str,
        ui_fn: Box<dyn FnMut(&egui::Context)>,
    ) -> Result<(), Error> {
        // 0. check if plugin is init'd + if window exists
        let egui_windows = self
            .try_state::<EguiWindowMap>()
            .ok_or(Error::msg("EguiPlugin is not initialized"))?;

        let window = self
            .get_window(label)
            .ok_or(Error::msg("a window for this provided label doesn't exist"))?;

        // 1. extract relevant window deets
        let scale_factor = window.scale_factor().unwrap_or(1.0) as f32;
        let size = window.inner_size()?;
        let PhysicalSize { width, height } = size;

        // 2. create egui context
        let context = egui::Context::default();
        context.set_zoom_factor(scale_factor);

        // 3. create renderer
        let renderer =
            tauri::async_runtime::block_on(
                async move { Renderer::new(window, width, height).await },
            )?;

        // 4. set up the EguiWindow struct
        let egui_window = EguiWindow {
            context,
            renderer,
            ui_fn,
            size,
        };

        // 5. insert into the plugin state
        let mut managed_windows = egui_windows.lock().unwrap();
        managed_windows.insert(label.to_string(), egui_window);

        Ok(())
    }
}
