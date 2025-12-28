mod core;
mod renderer;

use ash::vk;
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Window, WindowId};

use core::engine::Engine;

struct App {
    window: Option<Window>,
    engine: Option<Engine>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            window: None,
            engine: None,
        }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        println!("📥 Evento: resumed");
        // Crear la ventana
        let window = event_loop
            .create_window(
                Window::default_attributes()
                    .with_title("Obsidian Graphish - Vulkan Engine")
                    .with_inner_size(winit::dpi::LogicalSize::new(800, 600)),
            )
            .unwrap();
        
        println!("🪟 Ventana creada");

        // Inicializar el motor de Vulkan
        match Engine::new(&window) {
            Ok(engine) => {
                println!("📊 Motor de renderizado listo\n");
                self.engine = Some(engine);
                // Solicitar el primer frame inmediatamente
                window.request_redraw();
                self.window = Some(window);
                println!("✅ Inicialización completa");
            }
            Err(e) => {
                eprintln!("❌ Error al inicializar el motor: {:?}", e);
                event_loop.exit();
            }
        }
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                println!("Cerrando aplicación...");
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                // Renderizar un frame
                if let Some(engine) = &mut self.engine {
                    match engine.draw_frame() {
                        Ok(_) => {}
                        Err(vk::Result::ERROR_OUT_OF_DATE_KHR) | Err(vk::Result::SUBOPTIMAL_KHR) => {
                            // El swapchain necesita recrearse (por cambio de tamaño, etc)
                            // Por ahora solo ignoramos el error
                        }
                        Err(e) => {
                            eprintln!("Error al renderizar: {:?}", e);
                            event_loop.exit();
                        }
                    }
                }

                // Solicitar otro frame
                if let Some(window) = &self.window {
                    window.request_redraw();
                }
            }
            _ => (),
        }
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        // Solicitar redibujado en cada ciclo del loop
        if let Some(window) = &self.window {
            window.request_redraw();
        }
    }
}

fn main() {
    let event_loop = EventLoop::new().unwrap();

    // ControlFlow::Poll ejecuta el loop continuamente (ideal para juegos)
    event_loop.set_control_flow(ControlFlow::Poll);

    let mut app = App::default();
    let _ = event_loop.run_app(&mut app);
}