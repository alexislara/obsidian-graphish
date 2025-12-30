use std::sync::Arc;
use sysinfo::{System, MemoryRefreshKind, Pid};
use std::io::{Write, stdout};
use std::time::{Duration, Instant};
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

    // Campos de Métricas
    pid: Option<Pid>, // <--- Guardaremos el ID de nuestro proceso aquí
    last_render_time: Option<Instant>,
    frame_count: u32,
    accumulated_time: Duration,
    // Nuevo: Monitor de sistema
    sys: System,
}

// function debug view resource hardware
fn view_info_resource(frame_count: &u32, accumulated_time: &Duration, sys: &mut System, pid: Option<Pid>) {

    if let Some(my_pid) = pid {
        sys.refresh_memory_specifics(MemoryRefreshKind::nothing().with_ram());

        if let Some(process) = sys.process(my_pid) {
            let fps = frame_count;
            let avg_frame_time = accumulated_time.as_secs_f64() / *frame_count as f64;

            const TO_MB: f64 = 1024.0 * 1024.0;

            let app_memory = process.memory() as f64 / TO_MB;

            let app_cpu_usage = process.cpu_usage();

            print!(
                "\r🚀 Frames: FPS: {:<2} | Frame Time: {:.2}ms | ram usage: {:.2}mb | cpu usage: {:.2}",
                fps,
                avg_frame_time * 1000.0,
                app_memory,
                app_cpu_usage
            );
            let _ = stdout().flush();
        }
    }
}

impl ApplicationHandler for WindowApplication {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = Arc::new(event_loop.create_window(self.window_attributes.clone()).unwrap());
        self.core = Some(window.clone());
        self.gfx = Some(GraphicsContext::new(window.clone()));

        // debug view resource hardware
        let my_pid = sysinfo::Pid::from_u32(std::process::id());
        self.pid = Some(my_pid);
        self.last_render_time = Some(Instant::now());
        self.accumulated_time = Duration::ZERO;
        self.frame_count = 0;
        self.sys.refresh_all();
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                println!("\nThe close button was pressed; stopping");
                event_loop.exit();
            },
            WindowEvent::Resized(physical_size) => {
                if let Some(gfx) = &mut self.gfx {
                    gfx.resize(physical_size);
                }
            }
            WindowEvent::RedrawRequested => {
                let now = Instant::now();
                let delta_time = match self.last_render_time {
                    Some(last_time) => now.duration_since(last_time),
                    None => Duration::ZERO,
                };
                self.last_render_time = Some(now);

                if let Some(gfx) = &mut self.gfx {
                    match gfx.render() {
                        Ok(_) => {}
                        Err(wgpu::SurfaceError::Lost) => gfx.resize(gfx.size),
                        Err(wgpu::SurfaceError::OutOfMemory) => event_loop.exit(),
                        Err(e) => eprintln!("{:?}", e),
                    }
                }

                self.frame_count += 1;
                self.accumulated_time += delta_time;

                // view status resource hardware
                if self.accumulated_time >= Duration::from_secs(1) {
                    view_info_resource(
                        &self.frame_count,
                        &self.accumulated_time,
                        &mut self.sys,
                        self.pid
                    );
                    self.frame_count = 0;
                    self.accumulated_time = Duration::ZERO;
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

