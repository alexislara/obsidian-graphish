use ash::{vk, khr::swapchain};
use winit::window::Window;

use super::super::core::{instance::VkInstance, window::VkSurface};
use super::device::VkDevice;

/// Detalles de soporte del swapchain
pub struct SwapchainSupportDetails {
    pub capabilities: vk::SurfaceCapabilitiesKHR,
    pub formats: Vec<vk::SurfaceFormatKHR>,
    pub present_modes: Vec<vk::PresentModeKHR>,
}

/// Wrapper para el swapchain de Vulkan
pub struct VkSwapchain {
    pub swapchain_loader: swapchain::Device,
    pub swapchain: vk::SwapchainKHR,
    pub images: Vec<vk::Image>,
    pub image_views: Vec<vk::ImageView>,
    pub format: vk::Format,
    pub extent: vk::Extent2D,
}

impl VkSwapchain {
    /// Crea un nuevo swapchain
    pub fn new(
        instance: &VkInstance,
        device: &VkDevice,
        surface: &VkSurface,
        window: &Window,
    ) -> Result<Self, vk::Result> {
        let support = Self::query_swapchain_support(surface, device.physical_device)?;

        // Elegir configuración óptima
        let surface_format = Self::choose_swap_surface_format(&support.formats);
        let present_mode = Self::choose_swap_present_mode(&support.present_modes);
        let extent = Self::choose_swap_extent(&support.capabilities, window);

        // Determinar número de imágenes
        let mut image_count = support.capabilities.min_image_count + 1;
        if support.capabilities.max_image_count > 0
            && image_count > support.capabilities.max_image_count
        {
            image_count = support.capabilities.max_image_count;
        }

        // Crear el swapchain
        let mut create_info = vk::SwapchainCreateInfoKHR::default()
            .surface(surface.surface)
            .min_image_count(image_count)
            .image_format(surface_format.format)
            .image_color_space(surface_format.color_space)
            .image_extent(extent)
            .image_array_layers(1)
            .image_usage(vk::ImageUsageFlags::COLOR_ATTACHMENT);

        let indices = &device.queue_family_indices;
        let queue_family_indices = [
            indices.graphics_family.unwrap(),
            indices.present_family.unwrap(),
        ];

        if indices.graphics_family != indices.present_family {
            create_info = create_info
                .image_sharing_mode(vk::SharingMode::CONCURRENT)
                .queue_family_indices(&queue_family_indices);
        } else {
            create_info = create_info.image_sharing_mode(vk::SharingMode::EXCLUSIVE);
        }

        create_info = create_info
            .pre_transform(support.capabilities.current_transform)
            .composite_alpha(vk::CompositeAlphaFlagsKHR::OPAQUE)
            .present_mode(present_mode)
            .clipped(true)
            .old_swapchain(vk::SwapchainKHR::null());

        let swapchain_loader = swapchain::Device::new(&instance.instance, &device.device);
        let swapchain = unsafe { swapchain_loader.create_swapchain(&create_info, None)? };

        // Obtener las imágenes del swapchain
        let images = unsafe { swapchain_loader.get_swapchain_images(swapchain)? };

        // Crear image views
        let image_views = Self::create_image_views(&device.device, &images, surface_format.format)?;

        println!(
            "✓ Swapchain creado: {}x{} con {} imágenes",
            extent.width,
            extent.height,
            images.len()
        );

        Ok(VkSwapchain {
            swapchain_loader,
            swapchain,
            images,
            image_views,
            format: surface_format.format,
            extent,
        })
    }

    /// Consulta el soporte del swapchain
    fn query_swapchain_support(
        surface: &VkSurface,
        physical_device: vk::PhysicalDevice,
    ) -> Result<SwapchainSupportDetails, vk::Result> {
        let capabilities = surface.get_surface_capabilities(physical_device)?;
        let formats = surface.get_surface_formats(physical_device)?;
        let present_modes = surface.get_surface_present_modes(physical_device)?;

        Ok(SwapchainSupportDetails {
            capabilities,
            formats,
            present_modes,
        })
    }

    /// Elige el formato de superficie óptimo
    fn choose_swap_surface_format(formats: &[vk::SurfaceFormatKHR]) -> vk::SurfaceFormatKHR {
        // Preferir SRGB si está disponible
        for format in formats {
            if format.format == vk::Format::B8G8R8A8_SRGB
                && format.color_space == vk::ColorSpaceKHR::SRGB_NONLINEAR
            {
                return *format;
            }
        }
        formats[0]
    }

    /// Elige el modo de presentación óptimo
    fn choose_swap_present_mode(present_modes: &[vk::PresentModeKHR]) -> vk::PresentModeKHR {
        // Preferir MAILBOX (triple buffering) si está disponible
        for &mode in present_modes {
            if mode == vk::PresentModeKHR::MAILBOX {
                return mode;
            }
        }
        // FIFO siempre está disponible (v-sync)
        vk::PresentModeKHR::FIFO
    }

    /// Elige la resolución del swapchain
    fn choose_swap_extent(
        capabilities: &vk::SurfaceCapabilitiesKHR,
        window: &Window,
    ) -> vk::Extent2D {
        if capabilities.current_extent.width != u32::MAX {
            capabilities.current_extent
        } else {
            let size = window.inner_size();
            vk::Extent2D {
                width: size.width.clamp(
                    capabilities.min_image_extent.width,
                    capabilities.max_image_extent.width,
                ),
                height: size.height.clamp(
                    capabilities.min_image_extent.height,
                    capabilities.max_image_extent.height,
                ),
            }
        }
    }

    /// Crea las vistas de imagen para el swapchain
    fn create_image_views(
        device: &ash::Device,
        images: &[vk::Image],
        format: vk::Format,
    ) -> Result<Vec<vk::ImageView>, vk::Result> {
        let mut image_views = Vec::with_capacity(images.len());

        for &image in images {
            let create_info = vk::ImageViewCreateInfo::default()
                .image(image)
                .view_type(vk::ImageViewType::TYPE_2D)
                .format(format)
                .components(vk::ComponentMapping {
                    r: vk::ComponentSwizzle::IDENTITY,
                    g: vk::ComponentSwizzle::IDENTITY,
                    b: vk::ComponentSwizzle::IDENTITY,
                    a: vk::ComponentSwizzle::IDENTITY,
                })
                .subresource_range(vk::ImageSubresourceRange {
                    aspect_mask: vk::ImageAspectFlags::COLOR,
                    base_mip_level: 0,
                    level_count: 1,
                    base_array_layer: 0,
                    layer_count: 1,
                });

            let image_view = unsafe { device.create_image_view(&create_info, None)? };
            image_views.push(image_view);
        }

        Ok(image_views)
    }

    /// Destruye el swapchain
    pub fn destroy(&mut self, device: &ash::Device) {
        unsafe {
            for &image_view in &self.image_views {
                device.destroy_image_view(image_view, None);
            }
            self.swapchain_loader
                .destroy_swapchain(self.swapchain, None);
        }
        println!("✓ Swapchain destruido");
    }
}
