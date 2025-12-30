use winit::event_loop::EventLoop;
use obsidian_graphish::core::window::{run_with_event_loop, WindowApplication, WindowConfig};
fn main() {
    let config = WindowConfig {
        title: String::from("Obsidian graphish"),
        height: 600,
        width: 800
    };
    
    
    run_with_event_loop(
        &mut WindowApplication::default(),
        EventLoop::new().unwrap(),
        Option::from(config)
    );
}
