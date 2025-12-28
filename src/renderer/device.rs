use ash::{vk, Device as AshDevice};

use super::super::core::{instance::VkInstance, window::VkSurface};

/// Índices de las familias de colas
#[derive(Clone, Debug)]
pub struct QueueFamilyIndices {
    pub graphics_family: Option<u32>,
    pub present_family: Option<u32>,
}

impl QueueFamilyIndices {
    pub fn is_complete(&self) -> bool {
        self.graphics_family.is_some() && self.present_family.is_some()
    }
}

/// Wrapper para el dispositivo lógico de Vulkan
pub struct VkDevice {
    pub instance: VkInstance,
    pub physical_device: vk::PhysicalDevice,
    pub device: AshDevice,
    pub graphics_queue: vk::Queue,
    pub present_queue: vk::Queue,
    pub queue_family_indices: QueueFamilyIndices,
}

impl VkDevice {
    /// Selecciona el mejor dispositivo físico y crea un dispositivo lógico
    pub fn new(instance: &VkInstance, surface: &VkSurface) -> Result<Self, vk::Result> {
        // Enumerar dispositivos físicos
        let physical_devices = instance.enumerate_physical_devices()?;
        
        if physical_devices.is_empty() {
            panic!("No se encontraron dispositivos con soporte para Vulkan");
        }

        // Seleccionar el mejor dispositivo
        let (physical_device, queue_family_indices) = Self::pick_physical_device(
            instance,
            surface,
            &physical_devices,
        )?;

        // Obtener información del dispositivo
        let device_properties = instance.get_physical_device_properties(physical_device);
        let device_name = unsafe {
            std::ffi::CStr::from_ptr(device_properties.device_name.as_ptr())
                .to_str()
                .unwrap()
        };
        println!("✓ Dispositivo físico seleccionado: {}", device_name);

        // Crear el dispositivo lógico
        let device = Self::create_logical_device(
            instance,
            physical_device,
            &queue_family_indices,
        )?;

        // Obtener las colas
        let graphics_queue = unsafe {
            device.get_device_queue(queue_family_indices.graphics_family.unwrap(), 0)
        };
        let present_queue = unsafe {
            device.get_device_queue(queue_family_indices.present_family.unwrap(), 0)
        };

        println!("✓ Dispositivo lógico creado");

        Ok(VkDevice {
            instance: instance.clone(),
            physical_device,
            device,
            graphics_queue,
            present_queue,
            queue_family_indices,
        })
    }

    /// Selecciona el dispositivo físico más adecuado
    fn pick_physical_device(
        instance: &VkInstance,
        surface: &VkSurface,
        devices: &[vk::PhysicalDevice],
    ) -> Result<(vk::PhysicalDevice, QueueFamilyIndices), vk::Result> {
        for &device in devices {
            let indices = Self::find_queue_families(instance, surface, device)?;
            if indices.is_complete() && Self::check_device_extension_support(instance, device)? {
                return Ok((device, indices));
            }
        }
        panic!("No se encontró un dispositivo físico adecuado");
    }

    /// Encuentra las familias de colas necesarias
    fn find_queue_families(
        instance: &VkInstance,
        surface: &VkSurface,
        device: vk::PhysicalDevice,
    ) -> Result<QueueFamilyIndices, vk::Result> {
        let queue_families = instance.get_physical_device_queue_families(device);

        let mut indices = QueueFamilyIndices {
            graphics_family: None,
            present_family: None,
        };

        for (i, queue_family) in queue_families.iter().enumerate() {
            // Buscar familia con soporte gráfico
            if queue_family.queue_flags.contains(vk::QueueFlags::GRAPHICS) {
                indices.graphics_family = Some(i as u32);
            }

            // Buscar familia con soporte de presentación
            let present_support = surface.get_physical_device_surface_support(device, i as u32)?;
            if present_support {
                indices.present_family = Some(i as u32);
            }

            if indices.is_complete() {
                break;
            }
        }

        Ok(indices)
    }

    /// Verifica que el dispositivo soporte las extensiones necesarias
    fn check_device_extension_support(
        instance: &VkInstance,
        device: vk::PhysicalDevice,
    ) -> Result<bool, vk::Result> {
        let required_extensions = [ash::khr::swapchain::NAME.as_ptr()];
        
        let available_extensions = unsafe {
            instance
                .instance
                .enumerate_device_extension_properties(device)?
        };

        for required in &required_extensions {
            let required_name = unsafe { std::ffi::CStr::from_ptr(*required) };
            let found = available_extensions.iter().any(|ext| {
                let ext_name = unsafe { std::ffi::CStr::from_ptr(ext.extension_name.as_ptr()) };
                ext_name == required_name
            });
            if !found {
                return Ok(false);
            }
        }

        Ok(true)
    }

    /// Crea el dispositivo lógico
    fn create_logical_device(
        instance: &VkInstance,
        physical_device: vk::PhysicalDevice,
        queue_family_indices: &QueueFamilyIndices,
    ) -> Result<AshDevice, vk::Result> {
        let queue_priorities = [1.0];

        // Crear colas únicas
        let mut unique_queue_families = std::collections::HashSet::new();
        unique_queue_families.insert(queue_family_indices.graphics_family.unwrap());
        unique_queue_families.insert(queue_family_indices.present_family.unwrap());

        let queue_create_infos: Vec<vk::DeviceQueueCreateInfo> = unique_queue_families
            .iter()
            .map(|&family| {
                vk::DeviceQueueCreateInfo::default()
                    .queue_family_index(family)
                    .queue_priorities(&queue_priorities)
            })
            .collect();

        // Extensiones del dispositivo
        let device_extensions = [ash::khr::swapchain::NAME.as_ptr()];

        // Características del dispositivo
        let device_features = vk::PhysicalDeviceFeatures::default();

        let device_create_info = vk::DeviceCreateInfo::default()
            .queue_create_infos(&queue_create_infos)
            .enabled_extension_names(&device_extensions)
            .enabled_features(&device_features);

        unsafe {
            instance
                .instance
                .create_device(physical_device, &device_create_info, None)
        }
    }
}

impl Drop for VkDevice {
    fn drop(&mut self) {
        unsafe {
            self.device.destroy_device(None);
        }
        println!("✓ Dispositivo lógico destruido");
    }
}
