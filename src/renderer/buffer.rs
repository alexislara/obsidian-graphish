use ash::vk;
use glam::Mat4;
use std::mem;

use super::device::VkDevice;

/// Uniform Buffer Object para transformaciones
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct UniformBufferObject {
    pub model: Mat4,
    pub view: Mat4,
    pub proj: Mat4,
}

impl UniformBufferObject {
    pub fn new() -> Self {
        Self {
            model: Mat4::IDENTITY,
            view: Mat4::IDENTITY,
            proj: Mat4::IDENTITY,
        }
    }
}

/// Wrapper para uniform buffer en Vulkan
pub struct UniformBuffer {
    pub buffer: vk::Buffer,
    pub memory: vk::DeviceMemory,
    pub mapped: *mut std::ffi::c_void,
}

impl UniformBuffer {
    /// Crea un uniform buffer
    pub fn new(device: &VkDevice, size: vk::DeviceSize) -> Result<Self, vk::Result> {
        let buffer_info = vk::BufferCreateInfo::default()
            .size(size)
            .usage(vk::BufferUsageFlags::UNIFORM_BUFFER)
            .sharing_mode(vk::SharingMode::EXCLUSIVE);

        let buffer = unsafe { device.device.create_buffer(&buffer_info, None)? };

        let mem_requirements = unsafe { device.device.get_buffer_memory_requirements(buffer) };

        let memory_type_index = Self::find_memory_type(
            device,
            mem_requirements.memory_type_bits,
            vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT,
        )?;

        let alloc_info = vk::MemoryAllocateInfo::default()
            .allocation_size(mem_requirements.size)
            .memory_type_index(memory_type_index);

        let memory = unsafe { device.device.allocate_memory(&alloc_info, None)? };

        unsafe {
            device.device.bind_buffer_memory(buffer, memory, 0)?;
        }

        // Mapear memoria permanentemente
        let mapped = unsafe {
            device
                .device
                .map_memory(memory, 0, size, vk::MemoryMapFlags::empty())?
        };

        Ok(Self {
            buffer,
            memory,
            mapped,
        })
    }

    /// Actualiza los datos del uniform buffer
    pub fn update(&mut self, data: &UniformBufferObject) {
        unsafe {
            let data_ptr = self.mapped as *mut UniformBufferObject;
            *data_ptr = *data;
        }
    }

    /// Encuentra el tipo de memoria adecuado
    fn find_memory_type(
        device: &VkDevice,
        type_filter: u32,
        properties: vk::MemoryPropertyFlags,
    ) -> Result<u32, vk::Result> {
        let mem_properties = unsafe {
            device
                .instance
                .instance
                .get_physical_device_memory_properties(device.physical_device)
        };

        for i in 0..mem_properties.memory_type_count {
            if (type_filter & (1 << i)) != 0
                && (mem_properties.memory_types[i as usize].property_flags & properties)
                    == properties
            {
                return Ok(i);
            }
        }

        Err(vk::Result::ERROR_INITIALIZATION_FAILED)
    }

    /// Destruye el buffer
    pub fn destroy(&mut self, device: &ash::Device) {
        unsafe {
            device.unmap_memory(self.memory);
            device.destroy_buffer(self.buffer, None);
            device.free_memory(self.memory, None);
        }
    }
}
