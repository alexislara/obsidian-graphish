use ash::vk;
use std::mem;

use super::device::VkDevice;
use super::swapchain::VkSwapchain;
use super::vertex::Vertex2D;

/// Wrapper para el pipeline gráfico de Vulkan
pub struct VkPipeline {
    pub render_pass: vk::RenderPass,
    pub pipeline_layout: vk::PipelineLayout,
    pub pipeline: vk::Pipeline,
    pub framebuffers: Vec<vk::Framebuffer>,
    pub descriptor_set_layout: vk::DescriptorSetLayout,
}

impl VkPipeline {
    /// Crea un nuevo pipeline gráfico básico
    pub fn new(device: &VkDevice, swapchain: &VkSwapchain) -> Result<Self, vk::Result> {
        // Crear el descriptor set layout para uniform buffers
        let descriptor_set_layout = Self::create_descriptor_set_layout(device)?;

        // Crear el render pass
        let render_pass = Self::create_render_pass(device, swapchain)?;

        // Crear el pipeline layout
        let pipeline_layout = Self::create_pipeline_layout(device, descriptor_set_layout)?;

        // Crear el pipeline gráfico
        let pipeline = Self::create_graphics_pipeline(device, swapchain, render_pass, pipeline_layout)?;

        // Crear los framebuffers
        let framebuffers = Self::create_framebuffers(device, swapchain, render_pass)?;

        println!("✓ Pipeline gráfico creado");

        Ok(VkPipeline {
            render_pass,
            pipeline_layout,
            pipeline,
            framebuffers,
            descriptor_set_layout,
        })
    }

    /// Crea el render pass
    fn create_render_pass(device: &VkDevice, swapchain: &VkSwapchain) -> Result<vk::RenderPass, vk::Result> {
        let color_attachment = vk::AttachmentDescription::default()
            .format(swapchain.format)
            .samples(vk::SampleCountFlags::TYPE_1)
            .load_op(vk::AttachmentLoadOp::CLEAR)
            .store_op(vk::AttachmentStoreOp::STORE)
            .stencil_load_op(vk::AttachmentLoadOp::DONT_CARE)
            .stencil_store_op(vk::AttachmentStoreOp::DONT_CARE)
            .initial_layout(vk::ImageLayout::UNDEFINED)
            .final_layout(vk::ImageLayout::PRESENT_SRC_KHR);

        let color_attachment_ref = vk::AttachmentReference::default()
            .attachment(0)
            .layout(vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL);

        let color_attachments = [color_attachment_ref];

        let subpass = vk::SubpassDescription::default()
            .pipeline_bind_point(vk::PipelineBindPoint::GRAPHICS)
            .color_attachments(&color_attachments);

        let dependency = vk::SubpassDependency::default()
            .src_subpass(vk::SUBPASS_EXTERNAL)
            .dst_subpass(0)
            .src_stage_mask(vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT)
            .src_access_mask(vk::AccessFlags::empty())
            .dst_stage_mask(vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT)
            .dst_access_mask(vk::AccessFlags::COLOR_ATTACHMENT_WRITE);

        let attachments = [color_attachment];
        let subpasses = [subpass];
        let dependencies = [dependency];

        let render_pass_info = vk::RenderPassCreateInfo::default()
            .attachments(&attachments)
            .subpasses(&subpasses)
            .dependencies(&dependencies);

        unsafe { device.device.create_render_pass(&render_pass_info, None) }
    }

    /// Crea el descriptor set layout para uniform buffers
    fn create_descriptor_set_layout(device: &VkDevice) -> Result<vk::DescriptorSetLayout, vk::Result> {
        let ubo_layout_binding = vk::DescriptorSetLayoutBinding::default()
            .binding(0)
            .descriptor_type(vk::DescriptorType::UNIFORM_BUFFER)
            .descriptor_count(1)
            .stage_flags(vk::ShaderStageFlags::VERTEX);

        let bindings = [ubo_layout_binding];

        let layout_info = vk::DescriptorSetLayoutCreateInfo::default()
            .bindings(&bindings);

        unsafe { device.device.create_descriptor_set_layout(&layout_info, None) }
    }

    /// Crea el pipeline layout
    fn create_pipeline_layout(
        device: &VkDevice,
        descriptor_set_layout: vk::DescriptorSetLayout,
    ) -> Result<vk::PipelineLayout, vk::Result> {
        // Push constants para efectos (tiempo, etc)
        let push_constant_range = vk::PushConstantRange::default()
            .stage_flags(vk::ShaderStageFlags::FRAGMENT)
            .offset(0)
            .size((std::mem::size_of::<f32>() * 4) as u32); // 4 floats (16 bytes)
        
        let push_constant_ranges = [push_constant_range];
        let set_layouts = [descriptor_set_layout];
        
        let pipeline_layout_info = vk::PipelineLayoutCreateInfo::default()
            .set_layouts(&set_layouts)
            .push_constant_ranges(&push_constant_ranges);

        unsafe {
            device
                .device
                .create_pipeline_layout(&pipeline_layout_info, None)
        }
    }

    /// Crea el pipeline gráfico
    fn create_graphics_pipeline(
        device: &VkDevice,
        swapchain: &VkSwapchain,
        render_pass: vk::RenderPass,
        pipeline_layout: vk::PipelineLayout,
    ) -> Result<vk::Pipeline, vk::Result> {
        // Cargar los shaders 2D
        let vert_path = "shaders/sprite.vert.spv";
        let frag_path = "shaders/sprite.frag.spv";
        
        if !std::path::Path::new(vert_path).exists() || !std::path::Path::new(frag_path).exists() {
            eprintln!("⚠ Los shaders 2D no están compilados.");
            eprintln!("  Ejecuta: glslangValidator -V shaders/sprite.vert -o shaders/sprite.vert.spv");
            eprintln!("  Ejecuta: glslangValidator -V shaders/sprite.frag -o shaders/sprite.frag.spv");
            return Err(vk::Result::ERROR_INITIALIZATION_FAILED);
        }

        let vert_shader_code = std::fs::read(vert_path)
            .expect("Failed to read vertex shader");
        let frag_shader_code = std::fs::read(frag_path)
            .expect("Failed to read fragment shader");

        let vert_shader_module = Self::create_shader_module(device, &vert_shader_code)?;
        let frag_shader_module = Self::create_shader_module(device, &frag_shader_code)?;

        let entry_point = std::ffi::CString::new("main").unwrap();

        let vert_stage_info = vk::PipelineShaderStageCreateInfo::default()
            .stage(vk::ShaderStageFlags::VERTEX)
            .module(vert_shader_module)
            .name(&entry_point);

        let frag_stage_info = vk::PipelineShaderStageCreateInfo::default()
            .stage(vk::ShaderStageFlags::FRAGMENT)
            .module(frag_shader_module)
            .name(&entry_point);

        let shader_stages = [vert_stage_info, frag_stage_info];

        // Vertex input con binding y atributos
        let binding_description = Vertex2D::get_binding_description();
        let attribute_descriptions = Vertex2D::get_attribute_descriptions();

        let binding_descriptions = [binding_description];
        
        let vertex_input_info = vk::PipelineVertexInputStateCreateInfo::default()
            .vertex_binding_descriptions(&binding_descriptions)
            .vertex_attribute_descriptions(&attribute_descriptions);

        // Input assembly
        let input_assembly = vk::PipelineInputAssemblyStateCreateInfo::default()
            .topology(vk::PrimitiveTopology::TRIANGLE_LIST)
            .primitive_restart_enable(false);

        // Viewport y scissor
        let viewport = vk::Viewport::default()
            .x(0.0)
            .y(0.0)
            .width(swapchain.extent.width as f32)
            .height(swapchain.extent.height as f32)
            .min_depth(0.0)
            .max_depth(1.0);

        let scissor = vk::Rect2D::default()
            .offset(vk::Offset2D { x: 0, y: 0 })
            .extent(swapchain.extent);

        let viewports = [viewport];
        let scissors = [scissor];

        let viewport_state = vk::PipelineViewportStateCreateInfo::default()
            .viewports(&viewports)
            .scissors(&scissors);

        // Rasterizer
        let rasterizer = vk::PipelineRasterizationStateCreateInfo::default()
            .depth_clamp_enable(false)
            .rasterizer_discard_enable(false)
            .polygon_mode(vk::PolygonMode::FILL)
            .line_width(1.0)
            .cull_mode(vk::CullModeFlags::BACK)
            .front_face(vk::FrontFace::CLOCKWISE)
            .depth_bias_enable(false);

        // Multisampling
        let multisampling = vk::PipelineMultisampleStateCreateInfo::default()
            .sample_shading_enable(false)
            .rasterization_samples(vk::SampleCountFlags::TYPE_1);

        // Color blending
        let color_blend_attachment = vk::PipelineColorBlendAttachmentState::default()
            .color_write_mask(vk::ColorComponentFlags::RGBA)
            .blend_enable(false);

        let color_blend_attachments = [color_blend_attachment];

        let color_blending = vk::PipelineColorBlendStateCreateInfo::default()
            .logic_op_enable(false)
            .attachments(&color_blend_attachments);

        // Crear el pipeline
        let pipeline_info = vk::GraphicsPipelineCreateInfo::default()
            .stages(&shader_stages)
            .vertex_input_state(&vertex_input_info)
            .input_assembly_state(&input_assembly)
            .viewport_state(&viewport_state)
            .rasterization_state(&rasterizer)
            .multisample_state(&multisampling)
            .color_blend_state(&color_blending)
            .layout(pipeline_layout)
            .render_pass(render_pass)
            .subpass(0);

        let pipelines = unsafe {
            device.device.create_graphics_pipelines(
                vk::PipelineCache::null(),
                &[pipeline_info],
                None,
            )
        };

        // Limpiar los módulos de shader
        unsafe {
            device.device.destroy_shader_module(vert_shader_module, None);
            device.device.destroy_shader_module(frag_shader_module, None);
        }

        match pipelines {
            Ok(pipelines) => Ok(pipelines[0]),
            Err((_, err)) => Err(err),
        }
    }

    /// Crea un módulo de shader
    fn create_shader_module(device: &VkDevice, code: &[u8]) -> Result<vk::ShaderModule, vk::Result> {
        let code = ash::util::read_spv(&mut std::io::Cursor::new(code))
            .expect("Failed to read shader code");

        let create_info = vk::ShaderModuleCreateInfo::default().code(&code);

        unsafe { device.device.create_shader_module(&create_info, None) }
    }

    /// Crea los framebuffers
    fn create_framebuffers(
        device: &VkDevice,
        swapchain: &VkSwapchain,
        render_pass: vk::RenderPass,
    ) -> Result<Vec<vk::Framebuffer>, vk::Result> {
        let mut framebuffers = Vec::with_capacity(swapchain.image_views.len());

        for &image_view in &swapchain.image_views {
            let attachments = [image_view];

            let framebuffer_info = vk::FramebufferCreateInfo::default()
                .render_pass(render_pass)
                .attachments(&attachments)
                .width(swapchain.extent.width)
                .height(swapchain.extent.height)
                .layers(1);

            let framebuffer = unsafe { device.device.create_framebuffer(&framebuffer_info, None)? };
            framebuffers.push(framebuffer);
        }

        Ok(framebuffers)
    }

    /// Destruye el pipeline
    pub fn destroy(&mut self, device: &ash::Device) {
        unsafe {
            for &framebuffer in &self.framebuffers {
                device.destroy_framebuffer(framebuffer, None);
            }
            device.destroy_pipeline(self.pipeline, None);
            device.destroy_pipeline_layout(self.pipeline_layout, None);
            device.destroy_render_pass(self.render_pass, None);
            device.destroy_descriptor_set_layout(self.descriptor_set_layout, None);
        }
        println!("✓ Pipeline gráfico destruido");
    }
}
