pub struct WindowConfig {
    pub(crate) title: String,
    pub(crate) width: u32,
    pub(crate) height: u32,
}

pub fn create_window_config(title:String, height:u32, width:u32) -> WindowConfig {
    WindowConfig {
        title,
        height,
        width,
    }
}