use winit::application::ApplicationHandler;
use winit::dpi::LogicalSize;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Window, WindowId};
use crate::core::window_config::WindowConfig;

#[derive(Default)]
pub struct Application {
    window: Option<Window>,
    pub config: Option<WindowConfig>
}

impl ApplicationHandler for Application {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        match &self.config {
            Some(config) => {
                let window_attributes = Window::default_attributes()
                    .with_title(config.title.clone())
                    .with_inner_size(LogicalSize::new(config.width, config.height))
                    .with_transparent(false);
                self.window = Some(event_loop.create_window(window_attributes).unwrap());
            }
            None => self.window = Some(event_loop.create_window(Window::default_attributes()).unwrap())
        }
    }
    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                println!("The close button was pressed; stopping");
                event_loop.exit();
            },
            WindowEvent::RedrawRequested => {
                self.window.as_ref().unwrap().request_redraw();

            }
            _ => (),
        }
    }
}

pub fn start_event_loop(application:&mut Application, event_loop: EventLoop<()>) {
    // ControlFlow::Poll continuously runs the event loop, even if the OS hasn't
    // dispatched any events. This is ideal for games and similar applications.
    event_loop.set_control_flow(ControlFlow::Poll);

    // ControlFlow::Wait pauses the event loop if no events are available to process.
    // This is ideal for non-game applications that only update in response to user
    // input, and uses significantly less power/CPU time than ControlFlow::Poll.
    event_loop.set_control_flow(ControlFlow::Wait);
    event_loop.run_app(application).expect("Panic event loop error");
}