use tauri::Manager;
use tauri_runtime::UserEvent;

use tauri_runtime_wry::{Context, PluginBuilder};
use tauri_runtime_wry::{EventLoopIterationContext, Message, Plugin, WebContextStore};

use tauri_runtime_wry::tao::event::{Event, WindowEvent as TaoWindowEvent};
use tauri_runtime_wry::tao::event_loop::{ControlFlow, EventLoopProxy, EventLoopWindowTarget};

use tauri::{AppHandle, Runtime};

pub struct EguiPluginBuilder<R: Runtime> {
    app: AppHandle<R>,
}

impl<R: Runtime> EguiPluginBuilder<R> {
    pub fn new(app: AppHandle<R>) -> Self {
        Self { app }
    }
}

impl<T: UserEvent, R: Runtime> PluginBuilder<T> for EguiPluginBuilder<R> {
    type Plugin = EguiPlugin<T>;

    fn build(self, _context: Context<T>) -> Self::Plugin {
        let plugin = EguiPlugin::new();
        self.app.manage(plugin.clone());
        plugin
    }
}

#[derive(Clone, Debug)]
pub struct EguiPlugin<T: UserEvent> {
    _phantom: std::marker::PhantomData<T>,
}

unsafe impl<T: UserEvent> Send for EguiPlugin<T> {}
unsafe impl<T: UserEvent> Sync for EguiPlugin<T> {}

impl<T: UserEvent> EguiPlugin<T> {
    pub fn new() -> Self {
        Self {
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
        _context: EventLoopIterationContext<'_, T>,
        _web_context: &WebContextStore,
    ) -> bool {
        // Log all events we receive
        match event {
            Event::WindowEvent { event, .. } => {
                // println!("🪟 WindowEvent for window {:?}: {:?}", window_id, event);

                // Log specific window events with more detail
                match event {
                    // TaoWindowEvent::CloseRequested => {
                    //     println!("❌ Close requested");
                    // }
                    TaoWindowEvent::Resized(size) => {
                        println!("📏 Resized to: {}x{}", size.width, size.height);
                    }
                    // TaoWindowEvent::Moved(position) => {
                    //     println!("📍 Moved to: ({}, {})", position.x, position.y);
                    // }
                    // TaoWindowEvent::Focused(focused) => {
                    //     println!("🎯 Focus changed: {}", focused);
                    // }
                    // TaoWindowEvent::CursorMoved { position, .. } => {
                    //     println!("🖱️ Cursor moved to: ({}, {})", position.x, position.y);
                    // }
                    // TaoWindowEvent::MouseInput { state, button, .. } => {
                    //     println!("🖱️ Mouse {:?}: {:?}", state, button);
                    // }
                    // TaoWindowEvent::KeyboardInput { input, .. } => {
                    //   println!("⌨️  Keyboard input: {:?}", input);
                    // }
                    // TaoWindowEvent::ReceivedCharacter(c) => {
                    //   println!("📝 Character received: '{}'", c);
                    // }
                    &_ => {
                        // Other window events are logged above with the general format
                    }
                }
            }
            // Event::UserEvent(_) => {
            //     println!("👤 UserEvent");
            // }
            // Event::DeviceEvent { .. } => {
            //     println!("🖥️  DeviceEvent for device");
            // }
            // Event::NewEvents(cause) => {
            //     println!("🆕 NewEvents: {:?}", cause);
            // }
            // Event::MainEventsCleared => {
            //     println!("🧹 MainEventsCleared");
            // }
            Event::RedrawRequested(window_id) => {
                println!("🎨 RedrawRequested for window: {:?}", window_id);
            }
            // Event::RedrawEventsCleared => {
            //     println!("🎨 RedrawEventsCleared");
            // }
            // Event::LoopDestroyed => {
            //     println!("💀 LoopDestroyed - Event loop is shutting down");
            // }
            // Event::Suspended => {
            //     println!("⏸️  Suspended");
            // }
            // Event::Resumed => {
            //     println!("▶️  Resumed");
            // }
            &_ => {}
        }

        // Return false to let other plugins/handlers process the event
        false
    }
}

impl<T: UserEvent> Default for EguiPlugin<T> {
    fn default() -> Self {
        Self::new()
    }
}
