use ash::vk;
use raw_window_handle::HasDisplayHandle;
use winit::window::Window;

use super::instance::VkInstance;
use super::window::VkSurface;
use super::renderer2d::Renderer2D;
use crate::renderer::device::VkDevice;
use crate::renderer::swapchain::VkSwapchain;
use crate::renderer::pipeline::VkPipeline;

/// Motor principal de renderizado Vulkan
pub struct Engine {
    // El orden de los campos importa: se destruyen en orden inverso al que aparecen
    image_available_semaphores: Vec<vk::Semaphore>,
    render_finished_semaphores: Vec<vk::Semaphore>,
    in_flight_fences: Vec<vk::Fence>,
    command_buffers: Vec<vk::CommandBuffer>,
    command_pool: vk::CommandPool,
    renderer2d: Renderer2D,
    pipeline: VkPipeline,
    swapchain: VkSwapchain,
    device: VkDevice,
    surface: VkSurface,
    instance: VkInstance,
    current_frame: usize,
    start_time: std::time::Instant,
}

const MAX_FRAMES_IN_FLIGHT: usize = 2;

impl Engine {
    /// Inicializa el motor de renderizado
    pub fn new(window: &Window) -> Result<Self, vk::Result> {
        println!("🚀 Inicializando motor Vulkan...");

        // Obtener las extensiones requeridas por la ventana
        let extensions = ash_window::enumerate_required_extensions(
            window.display_handle().unwrap().as_raw()
        )
        .expect("Failed to enumerate required extensions")
        .to_vec();

        // Crear la instancia de Vulkan
        let instance = VkInstance::new(&extensions)?;

        // Crear la superficie
        let surface = VkSurface::new(&instance, window)?;

        // Crear el dispositivo lógico
        let device = VkDevice::new(&instance, &surface)?;

        // Crear el swapchain
        let swapchain = VkSwapchain::new(&instance, &device, &surface, window)?;

        // Crear el pipeline
        let pipeline = VkPipeline::new(&device, &swapchain)?;

        // Crear el command pool
        let command_pool = Self::create_command_pool(&device)?;

        // Crear los command buffers
        let command_buffers = Self::create_command_buffers(&device, command_pool, &pipeline, &swapchain)?;

        // Crear los objetos de sincronización
        let (image_available_semaphores, render_finished_semaphores, in_flight_fences) =
            Self::create_sync_objects(&device)?;

        // Inicializar el renderer 2D
        let renderer2d = Renderer2D::new(
            &device,
            &pipeline,
            swapchain.images.len(),
            swapchain.extent.width as f32,
            swapchain.extent.height as f32,
        )?;

        println!("✓ Motor Vulkan inicializado exitosamente\n");

        Ok(Engine {
            image_available_semaphores,
            render_finished_semaphores,
            in_flight_fences,
            command_buffers,
            command_pool,
            renderer2d,
            pipeline,
            swapchain,
            device,
            surface,
            instance,
            current_frame: 0,
            start_time: std::time::Instant::now(),
        })
    }

    /// Crea el command pool
    fn create_command_pool(device: &VkDevice) -> Result<vk::CommandPool, vk::Result> {
        let pool_info = vk::CommandPoolCreateInfo::default()
            .flags(vk::CommandPoolCreateFlags::RESET_COMMAND_BUFFER)
            .queue_family_index(device.queue_family_indices.graphics_family.unwrap());

        unsafe { device.device.create_command_pool(&pool_info, None) }
    }

    /// Crea los command buffers
    fn create_command_buffers(
        device: &VkDevice,
        command_pool: vk::CommandPool,
        _pipeline: &VkPipeline,
        _swapchain: &VkSwapchain,
    ) -> Result<Vec<vk::CommandBuffer>, vk::Result> {
        let alloc_info = vk::CommandBufferAllocateInfo::default()
            .command_pool(command_pool)
            .level(vk::CommandBufferLevel::PRIMARY)
            .command_buffer_count(MAX_FRAMES_IN_FLIGHT as u32);

        unsafe { device.device.allocate_command_buffers(&alloc_info) }
    }

    /// Crea los objetos de sincronización
    fn create_sync_objects(
        device: &VkDevice,
    ) -> Result<(Vec<vk::Semaphore>, Vec<vk::Semaphore>, Vec<vk::Fence>), vk::Result> {
        let semaphore_info = vk::SemaphoreCreateInfo::default();
        let fence_info = vk::FenceCreateInfo::default().flags(vk::FenceCreateFlags::SIGNALED);

        let mut image_available_semaphores = Vec::with_capacity(MAX_FRAMES_IN_FLIGHT);
        let mut render_finished_semaphores = Vec::with_capacity(MAX_FRAMES_IN_FLIGHT);
        let mut in_flight_fences = Vec::with_capacity(MAX_FRAMES_IN_FLIGHT);

        for _ in 0..MAX_FRAMES_IN_FLIGHT {
            unsafe {
                image_available_semaphores.push(device.device.create_semaphore(&semaphore_info, None)?);
                render_finished_semaphores.push(device.device.create_semaphore(&semaphore_info, None)?);
                in_flight_fences.push(device.device.create_fence(&fence_info, None)?);
            }
        }

        Ok((
            image_available_semaphores,
            render_finished_semaphores,
            in_flight_fences,
        ))
    }

    /// Graba los comandos de renderizado
    fn record_command_buffer(
        &mut self,
        command_buffer: vk::CommandBuffer,
        image_index: u32,
        time: f32,
    ) -> Result<(), vk::Result> {
        let begin_info = vk::CommandBufferBeginInfo::default();

        unsafe {
            self.device
                .device
                .begin_command_buffer(command_buffer, &begin_info)?;
        }

        let clear_values = [vk::ClearValue {
            color: vk::ClearColorValue {
                float32: [0.1, 0.1, 0.1, 1.0], // Gris oscuro
            },
        }];

        let render_pass_info = vk::RenderPassBeginInfo::default()
            .render_pass(self.pipeline.render_pass)
            .framebuffer(self.pipeline.framebuffers[image_index as usize])
            .render_area(vk::Rect2D {
                offset: vk::Offset2D { x: 0, y: 0 },
                extent: self.swapchain.extent,
            })
            .clear_values(&clear_values);

        unsafe {
            self.device.device.cmd_begin_render_pass(
                command_buffer,
                &render_pass_info,
                vk::SubpassContents::INLINE,
            );

            self.device.device.cmd_bind_pipeline(
                command_buffer,
                vk::PipelineBindPoint::GRAPHICS,
                self.pipeline.pipeline,
            );

            // Bind vertex buffer
            let vertex_buffers = [self.renderer2d.vertex_buffer];
            let offsets = [0];
            self.device.device.cmd_bind_vertex_buffers(
                command_buffer,
                0,
                &vertex_buffers,
                &offsets,
            );

            // Bind index buffer
            self.device.device.cmd_bind_index_buffer(
                command_buffer,
                self.renderer2d.index_buffer,
                0,
                vk::IndexType::UINT16,
            );

            // Dibujar cada sprite
            let sprite_count = self.renderer2d.sprites.len();
            for i in 0..sprite_count {
                // Obtener el color del sprite actual
                let sprite_color = self.renderer2d.sprites[i].color;
                
                // Push constants: tiempo + color RGB del sprite
                let push_constants = [time, sprite_color.x, sprite_color.y, sprite_color.z];
                let push_bytes = bytemuck::cast_slice(&push_constants);
                self.device.device.cmd_push_constants(
                    command_buffer,
                    self.pipeline.pipeline_layout,
                    vk::ShaderStageFlags::FRAGMENT,
                    0,
                    push_bytes,
                );

                // Actualizar uniform buffer para este sprite
                self.renderer2d.update_uniforms(image_index as usize, i);

                // Bind descriptor set
                let descriptor_sets = [self.renderer2d.descriptor_sets[image_index as usize]];
                self.device.device.cmd_bind_descriptor_sets(
                    command_buffer,
                    vk::PipelineBindPoint::GRAPHICS,
                    self.pipeline.pipeline_layout,
                    0,
                    &descriptor_sets,
                    &[],
                );

                // Dibujar el quad del sprite
                self.device.device.cmd_draw_indexed(
                    command_buffer,
                    self.renderer2d.index_count,
                    1,
                    0,
                    0,
                    0,
                );
            }

            self.device.device.cmd_end_render_pass(command_buffer);

            self.device.device.end_command_buffer(command_buffer)?;
        }

        Ok(())
    }

    /// Renderiza un frame
    pub fn draw_frame(&mut self) -> Result<(), vk::Result> {
        // Esperar a que el frame anterior termine
        unsafe {
            self.device.device.wait_for_fences(
                &[self.in_flight_fences[self.current_frame]],
                true,
                u64::MAX,
            )?;
        }

        // Adquirir la siguiente imagen
        let (image_index, _is_suboptimal) = unsafe {
            self.swapchain.swapchain_loader.acquire_next_image(
                self.swapchain.swapchain,
                u64::MAX,
                self.image_available_semaphores[self.current_frame],
                vk::Fence::null(),
            )?
        };

        // Resetear el fence solo si vamos a enviar trabajo
        unsafe {
            self.device
                .device
                .reset_fences(&[self.in_flight_fences[self.current_frame]])?;
        }

        // Calcular el tiempo transcurrido
        let time = self.start_time.elapsed().as_secs_f32();

        // Resetear y grabar el command buffer
        let command_buffer = self.command_buffers[self.current_frame];
        unsafe {
            self.device
                .device
                .reset_command_buffer(command_buffer, vk::CommandBufferResetFlags::empty())?;
        }
        self.record_command_buffer(command_buffer, image_index, time)?;

        // Enviar el command buffer
        let wait_semaphores = [self.image_available_semaphores[self.current_frame]];
        let wait_stages = [vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT];
        let command_buffers = [command_buffer];
        let signal_semaphores = [self.render_finished_semaphores[self.current_frame]];

        let submit_info = vk::SubmitInfo::default()
            .wait_semaphores(&wait_semaphores)
            .wait_dst_stage_mask(&wait_stages)
            .command_buffers(&command_buffers)
            .signal_semaphores(&signal_semaphores);

        unsafe {
            self.device.device.queue_submit(
                self.device.graphics_queue,
                &[submit_info],
                self.in_flight_fences[self.current_frame],
            )?;
        }

        // Presentar la imagen
        let swapchains = [self.swapchain.swapchain];
        let image_indices = [image_index];

        let present_info = vk::PresentInfoKHR::default()
            .wait_semaphores(&signal_semaphores)
            .swapchains(&swapchains)
            .image_indices(&image_indices);

        unsafe {
            self.swapchain
                .swapchain_loader
                .queue_present(self.device.present_queue, &present_info)?;
        }

        // Avanzar al siguiente frame
        self.current_frame = (self.current_frame + 1) % MAX_FRAMES_IN_FLIGHT;

        Ok(())
    }

    /// Espera a que el dispositivo termine todas las operaciones
    pub fn wait_idle(&self) {
        unsafe {
            self.device.device.device_wait_idle().unwrap();
        }
    }
}

impl Drop for Engine {
    fn drop(&mut self) {
        self.wait_idle();

        unsafe {
            // Destruir objetos de sincronización
            for i in 0..MAX_FRAMES_IN_FLIGHT {
                self.device
                    .device
                    .destroy_semaphore(self.image_available_semaphores[i], None);
                self.device
                    .device
                    .destroy_semaphore(self.render_finished_semaphores[i], None);
                self.device
                    .device
                    .destroy_fence(self.in_flight_fences[i], None);
            }

            // Destruir command pool
            self.device
                .device
                .destroy_command_pool(self.command_pool, None);
        }

        // Destruir renderer 2D
        self.renderer2d.destroy(&self.device.device);

        // Destruir pipeline
        self.pipeline.destroy(&self.device.device);

        // Destruir swapchain
        self.swapchain.destroy(&self.device.device);

        // Destruir superficie
        self.surface.destroy();

        println!("✓ Motor Vulkan finalizado");
    }
}
