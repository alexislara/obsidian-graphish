use ash::{vk, khr::surface};
use raw_window_handle::{HasDisplayHandle, HasWindowHandle};
use winit::window::Window;

use super::instance::VkInstance;

/// Wrapper para la superficie de Vulkan
pub struct VkSurface {
    pub surface_loader: surface::Instance,
    pub surface: vk::SurfaceKHR,
}

impl VkSurface {
    /// Crea una superficie de Vulkan desde una ventana de winit
    pub fn new(instance: &VkInstance, window: &Window) -> Result<Self, vk::Result> {
        let surface = unsafe {
            ash_window::create_surface(
                &instance.entry,
                &instance.instance,
                window.display_handle().unwrap().as_raw(),
                window.window_handle().unwrap().as_raw(),
                None,
            )
            .expect("Failed to create window surface")
        };

        let surface_loader = surface::Instance::new(&instance.entry, &instance.instance);

        println!("✓ Superficie de Vulkan creada");

        Ok(VkSurface {
            surface_loader,
            surface,
        })
    }

    /// Verifica si un dispositivo físico soporta presentación en esta superficie
    pub fn get_physical_device_surface_support(
        &self,
        physical_device: vk::PhysicalDevice,
        queue_family_index: u32,
    ) -> Result<bool, vk::Result> {
        unsafe {
            self.surface_loader.get_physical_device_surface_support(
                physical_device,
                queue_family_index,
                self.surface,
            )
        }
    }

    /// Obtiene las capacidades de la superficie para un dispositivo físico
    pub fn get_surface_capabilities(
        &self,
        physical_device: vk::PhysicalDevice,
    ) -> Result<vk::SurfaceCapabilitiesKHR, vk::Result> {
        unsafe {
            self.surface_loader
                .get_physical_device_surface_capabilities(physical_device, self.surface)
        }
    }

    /// Obtiene los formatos soportados por la superficie
    pub fn get_surface_formats(
        &self,
        physical_device: vk::PhysicalDevice,
    ) -> Result<Vec<vk::SurfaceFormatKHR>, vk::Result> {
        unsafe {
            self.surface_loader
                .get_physical_device_surface_formats(physical_device, self.surface)
        }
    }

    /// Obtiene los modos de presentación soportados
    pub fn get_surface_present_modes(
        &self,
        physical_device: vk::PhysicalDevice,
    ) -> Result<Vec<vk::PresentModeKHR>, vk::Result> {
        unsafe {
            self.surface_loader
                .get_physical_device_surface_present_modes(physical_device, self.surface)
        }
    }

    /// Destruye la superficie
    pub fn destroy(&mut self) {
        unsafe {
            self.surface_loader.destroy_surface(self.surface, None);
        }
        println!("✓ Superficie de Vulkan destruida");
    }
}
