use ash::{vk, Entry};
use std::ffi::CString;
use std::sync::Arc;

/// Wrapper para la instancia de Vulkan
#[derive(Clone)]
pub struct VkInstance {
    pub entry: Arc<Entry>,
    pub instance: Arc<ash::Instance>,
}

impl VkInstance {
    /// Crea una nueva instancia de Vulkan
    pub fn new(window_extensions: &[*const i8]) -> Result<Self, vk::Result> {
        // Cargar la librería de Vulkan
        let entry = unsafe { Entry::load().expect("Failed to load Vulkan library") };

        // Información de la aplicación
        let app_name = CString::new("Obsidian Graphish").unwrap();
        let engine_name = CString::new("Obsidian Engine").unwrap();
        
        let app_info = vk::ApplicationInfo::default()
            .application_name(&app_name)
            .application_version(vk::make_api_version(0, 1, 0, 0))
            .engine_name(&engine_name)
            .engine_version(vk::make_api_version(0, 1, 0, 0))
            .api_version(vk::make_api_version(0, 1, 3, 0));

        // Extensiones requeridas (de la ventana)
        let extension_names = window_extensions;

        // Capas de validación para debugging (opcional)
        let layer_names = if cfg!(debug_assertions) {
            vec![CString::new("VK_LAYER_KHRONOS_validation").unwrap()]
        } else {
            vec![]
        };
        
        let layer_names_raw: Vec<*const i8> = layer_names
            .iter()
            .map(|name| name.as_ptr())
            .collect();

        // Crear la instancia
        let create_info = vk::InstanceCreateInfo::default()
            .application_info(&app_info)
            .enabled_extension_names(extension_names)
            .enabled_layer_names(&layer_names_raw);

        let instance = unsafe {
            entry
                .create_instance(&create_info, None)
                .expect("Failed to create Vulkan instance")
        };

        println!("✓ Instancia de Vulkan creada");

        Ok(VkInstance { 
            entry: Arc::new(entry), 
            instance: Arc::new(instance) 
        })
    }

    /// Enumera los dispositivos físicos disponibles
    pub fn enumerate_physical_devices(&self) -> Result<Vec<vk::PhysicalDevice>, vk::Result> {
        unsafe { self.instance.enumerate_physical_devices() }
    }

    /// Obtiene las propiedades de un dispositivo físico
    pub fn get_physical_device_properties(&self, device: vk::PhysicalDevice) -> vk::PhysicalDeviceProperties {
        unsafe { self.instance.get_physical_device_properties(device) }
    }

    /// Obtiene las familias de colas de un dispositivo físico
    pub fn get_physical_device_queue_families(&self, device: vk::PhysicalDevice) -> Vec<vk::QueueFamilyProperties> {
        unsafe { self.instance.get_physical_device_queue_family_properties(device) }
    }
}

impl Drop for VkInstance {
    fn drop(&mut self) {
        unsafe {
            self.instance.destroy_instance(None);
        }
        println!("✓ Instancia de Vulkan destruida");
    }
}
