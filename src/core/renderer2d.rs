use ash::vk;
use glam::{Vec2, Vec3};
use std::mem;

use crate::renderer::buffer::{UniformBuffer, UniformBufferObject};
use crate::renderer::device::VkDevice;
use crate::renderer::pipeline::VkPipeline;
use crate::renderer::sprite::{Camera2D, Sprite2D};
use crate::renderer::vertex::{create_quad_indices, create_quad_vertices, Vertex2D};

/// Sistema de renderizado 2D
pub struct Renderer2D {
    // Buffers de geometría
    pub vertex_buffer: vk::Buffer,
    pub vertex_buffer_memory: vk::DeviceMemory,
    pub index_buffer: vk::Buffer,
    pub index_buffer_memory: vk::DeviceMemory,
    pub index_count: u32,

    // Uniform buffers (uno por frame in flight)
    pub uniform_buffers: Vec<UniformBuffer>,

    // Descriptor pool y sets
    pub descriptor_pool: vk::DescriptorPool,
    pub descriptor_sets: Vec<vk::DescriptorSet>,

    // Cámara
    pub camera: Camera2D,

    // Sprites en la escena
    pub sprites: Vec<Sprite2D>,
}

impl Renderer2D {
    pub fn new(
        device: &VkDevice,
        pipeline: &VkPipeline,
        swapchain_image_count: usize,
        viewport_width: f32,
        viewport_height: f32,
    ) -> Result<Self, vk::Result> {
        // Crear buffers de geometría del quad
        let vertices = create_quad_vertices();
        let indices = create_quad_indices();

        let (vertex_buffer, vertex_buffer_memory) =
            Self::create_vertex_buffer(device, &vertices)?;

        let (index_buffer, index_buffer_memory) = Self::create_index_buffer(device, &indices)?;

        // Crear uniform buffers (uno por frame)
        let mut uniform_buffers = Vec::new();
        for _ in 0..swapchain_image_count {
            let ubo = UniformBuffer::new(
                device,
                mem::size_of::<UniformBufferObject>() as vk::DeviceSize,
            )?;
            uniform_buffers.push(ubo);
        }

        // Crear descriptor pool
        let descriptor_pool = Self::create_descriptor_pool(device, swapchain_image_count)?;

        // Crear descriptor sets
        let descriptor_sets = Self::create_descriptor_sets(
            device,
            descriptor_pool,
            pipeline.descriptor_set_layout,
            &uniform_buffers,
        )?;

        // Inicializar cámara
        let camera = Camera2D::new(viewport_width, viewport_height);

        // Sprites de ejemplo
        let sprites = vec![
            Sprite2D {
                position: Vec2::new(0.0, 0.0),
                scale: Vec2::new(100.0, 100.0),
                rotation: 0.0,
                color: Vec3::new(1.0, 0.0, 0.0), // Rojo
                z_order: 0.0,
            },
            Sprite2D {
                position: Vec2::new(150.0, 100.0),
                scale: Vec2::new(80.0, 80.0),
                rotation: 0.785, // 45 grados
                color: Vec3::new(0.0, 1.0, 0.0), // Verde
                z_order: 0.1,
            },
            Sprite2D {
                position: Vec2::new(-150.0, -50.0),
                scale: Vec2::new(120.0, 60.0),
                rotation: 0.0,
                color: Vec3::new(0.0, 0.0, 1.0), // Azul
                z_order: 0.2,
            },
        ];

        println!("✓ Renderer 2D inicializado con {} sprites", sprites.len());

        Ok(Self {
            vertex_buffer,
            vertex_buffer_memory,
            index_buffer,
            index_buffer_memory,
            index_count: indices.len() as u32,
            uniform_buffers,
            descriptor_pool,
            descriptor_sets,
            camera,
            sprites,
        })
    }

    /// Actualiza las matrices para un sprite específico
    pub fn update_uniforms(&mut self, frame_index: usize, sprite_index: usize) {
        if sprite_index >= self.sprites.len() {
            return;
        }

        let sprite = &self.sprites[sprite_index];

        let mut ubo = UniformBufferObject::new();
        ubo.model = sprite.model_matrix();
        ubo.view = self.camera.view_matrix();
        ubo.proj = self.camera.projection_matrix();

        self.uniform_buffers[frame_index].update(&ubo);
    }

    /// Crea el vertex buffer
    fn create_vertex_buffer(
        device: &VkDevice,
        vertices: &[Vertex2D],
    ) -> Result<(vk::Buffer, vk::DeviceMemory), vk::Result> {
        let buffer_size = (mem::size_of::<Vertex2D>() * vertices.len()) as vk::DeviceSize;

        let buffer_info = vk::BufferCreateInfo::default()
            .size(buffer_size)
            .usage(vk::BufferUsageFlags::VERTEX_BUFFER)
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

            let data = device
                .device
                .map_memory(memory, 0, buffer_size, vk::MemoryMapFlags::empty())?;

            std::ptr::copy_nonoverlapping(vertices.as_ptr(), data as *mut Vertex2D, vertices.len());

            device.device.unmap_memory(memory);
        }

        Ok((buffer, memory))
    }

    /// Crea el index buffer
    fn create_index_buffer(
        device: &VkDevice,
        indices: &[u16],
    ) -> Result<(vk::Buffer, vk::DeviceMemory), vk::Result> {
        let buffer_size = (mem::size_of::<u16>() * indices.len()) as vk::DeviceSize;

        let buffer_info = vk::BufferCreateInfo::default()
            .size(buffer_size)
            .usage(vk::BufferUsageFlags::INDEX_BUFFER)
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

            let data = device
                .device
                .map_memory(memory, 0, buffer_size, vk::MemoryMapFlags::empty())?;

            std::ptr::copy_nonoverlapping(indices.as_ptr(), data as *mut u16, indices.len());

            device.device.unmap_memory(memory);
        }

        Ok((buffer, memory))
    }

    /// Crea el descriptor pool
    fn create_descriptor_pool(
        device: &VkDevice,
        swapchain_image_count: usize,
    ) -> Result<vk::DescriptorPool, vk::Result> {
        let pool_size = vk::DescriptorPoolSize::default()
            .ty(vk::DescriptorType::UNIFORM_BUFFER)
            .descriptor_count(swapchain_image_count as u32);

        let pool_sizes = [pool_size];

        let pool_info = vk::DescriptorPoolCreateInfo::default()
            .pool_sizes(&pool_sizes)
            .max_sets(swapchain_image_count as u32);

        unsafe { device.device.create_descriptor_pool(&pool_info, None) }
    }

    /// Crea los descriptor sets
    fn create_descriptor_sets(
        device: &VkDevice,
        pool: vk::DescriptorPool,
        layout: vk::DescriptorSetLayout,
        uniform_buffers: &[UniformBuffer],
    ) -> Result<Vec<vk::DescriptorSet>, vk::Result> {
        let layouts: Vec<vk::DescriptorSetLayout> = vec![layout; uniform_buffers.len()];

        let alloc_info = vk::DescriptorSetAllocateInfo::default()
            .descriptor_pool(pool)
            .set_layouts(&layouts);

        let descriptor_sets = unsafe { device.device.allocate_descriptor_sets(&alloc_info)? };

        for (i, &descriptor_set) in descriptor_sets.iter().enumerate() {
            let buffer_info = vk::DescriptorBufferInfo::default()
                .buffer(uniform_buffers[i].buffer)
                .offset(0)
                .range(mem::size_of::<UniformBufferObject>() as vk::DeviceSize);

            let buffer_infos = [buffer_info];

            let descriptor_write = vk::WriteDescriptorSet::default()
                .dst_set(descriptor_set)
                .dst_binding(0)
                .dst_array_element(0)
                .descriptor_type(vk::DescriptorType::UNIFORM_BUFFER)
                .buffer_info(&buffer_infos);

            let descriptor_writes = [descriptor_write];

            unsafe {
                device
                    .device
                    .update_descriptor_sets(&descriptor_writes, &[]);
            }
        }

        Ok(descriptor_sets)
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

    /// Destruye todos los recursos
    pub fn destroy(&mut self, device: &ash::Device) {
        unsafe {
            device.destroy_buffer(self.vertex_buffer, None);
            device.free_memory(self.vertex_buffer_memory, None);
            device.destroy_buffer(self.index_buffer, None);
            device.free_memory(self.index_buffer_memory, None);

            for uniform_buffer in &mut self.uniform_buffers {
                uniform_buffer.destroy(device);
            }

            device.destroy_descriptor_pool(self.descriptor_pool, None);
        }
        println!("✓ Renderer 2D destruido");
    }
}
