use std::sync::Arc;
use winit::{
    window::{Window, WindowId, WindowAttributes},
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    dpi::LogicalSize
};
use crate::renderer::graphics::GraphicsContext;

#[derive(Clone)]
pub struct WindowConfig {
    pub title: String,
    pub height: u32,
    pub width: u32,
}

#[derive(Default)]
pub struct WindowApplication {
    core: Option<Arc<Window>>,
    gfx: Option<GraphicsContext>,
    pub config: Option<WindowConfig>,
    window_attributes: WindowAttributes,
}

impl ApplicationHandler for WindowApplication {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = Arc::new(event_loop.create_window(self.window_attributes.clone()).unwrap());
        self.core = Some(window.clone());
        self.gfx = Some(GraphicsContext::new(window.clone()));
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                println!("The close button was pressed; stopping");
                event_loop.exit();
            },
            WindowEvent::Resized(physical_size) => {
                if let Some(gfx) = &mut self.gfx {
                    gfx.resize(physical_size);
                }
            }
            WindowEvent::RedrawRequested => {
                if let Some(gfx) = &mut self.gfx {
                    match gfx.render() {
                        Ok(_) => {}
                        Err(wgpu::SurfaceError::Lost) => gfx.resize(gfx.size),
                        Err(wgpu::SurfaceError::OutOfMemory) => event_loop.exit(),
                        Err(e) => eprintln!("{:?}", e),
                    }
                }
                self.core.as_ref().unwrap().request_redraw();
            }
            _ => (),
        }
    }
}

fn create_window_attribute(window_config:Option<WindowConfig>) -> WindowAttributes{
    match window_config {
        Some(window_config) => Window::default_attributes()
            .with_title(window_config.title)
            .with_inner_size(LogicalSize::new(window_config.width, window_config.height))
            .with_transparent(false),
        None => WindowAttributes::default()
    }
}

pub fn run_with_event_loop(application:&mut WindowApplication, event_loop: EventLoop<()>, config: Option<WindowConfig>) {
    // ControlFlow::Poll continuously runs the event loop, even if the OS hasn't
    // dispatched any events. This is ideal for games and similar applications.
    event_loop.set_control_flow(ControlFlow::Poll);
    application.window_attributes = create_window_attribute(config);
    event_loop.run_app(application).expect("Panic event loop error");
}

