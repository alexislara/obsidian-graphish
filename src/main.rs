use winit::event_loop::EventLoop;
use obsidian_graphish::core::{
    window::{
        start_event_loop,
        Application,
    },
    window_config::{
        create_window_config,
    }
};

fn main() {
    let mut application = Application::default();
    let app_config = create_window_config(String::from("title render"), 600, 800);

    application.config = Some(app_config);
    start_event_loop(&mut application, EventLoop::new().unwrap());
}