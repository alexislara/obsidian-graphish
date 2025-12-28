use ash::vk;
use std::mem;

/// Vértice para sprites 2D
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Vertex2D {
    pub position: [f32; 3],  // x, y, z
    pub color: [f32; 3],     // r, g, b
    pub tex_coord: [f32; 2], // u, v
}

impl Vertex2D {
    /// Descripción de los atributos del vértice para Vulkan
    pub fn get_binding_description() -> vk::VertexInputBindingDescription {
        vk::VertexInputBindingDescription::default()
            .binding(0)
            .stride(mem::size_of::<Self>() as u32)
            .input_rate(vk::VertexInputRate::VERTEX)
    }

    /// Descripción de los atributos individuales
    pub fn get_attribute_descriptions() -> [vk::VertexInputAttributeDescription; 3] {
        [
            // Position
            vk::VertexInputAttributeDescription::default()
                .binding(0)
                .location(0)
                .format(vk::Format::R32G32B32_SFLOAT)
                .offset(0),
            // Color
            vk::VertexInputAttributeDescription::default()
                .binding(0)
                .location(1)
                .format(vk::Format::R32G32B32_SFLOAT)
                .offset(mem::size_of::<[f32; 3]>() as u32),
            // TexCoord
            vk::VertexInputAttributeDescription::default()
                .binding(0)
                .location(2)
                .format(vk::Format::R32G32_SFLOAT)
                .offset((mem::size_of::<[f32; 3]>() * 2) as u32),
        ]
    }
}

/// Genera los vértices para un quad (sprite)
pub fn create_quad_vertices() -> Vec<Vertex2D> {
    vec![
        // Top-left
        Vertex2D {
            position: [-0.5, -0.5, 0.0],
            color: [1.0, 1.0, 1.0],
            tex_coord: [0.0, 0.0],
        },
        // Top-right
        Vertex2D {
            position: [0.5, -0.5, 0.0],
            color: [1.0, 1.0, 1.0],
            tex_coord: [1.0, 0.0],
        },
        // Bottom-right
        Vertex2D {
            position: [0.5, 0.5, 0.0],
            color: [1.0, 1.0, 1.0],
            tex_coord: [1.0, 1.0],
        },
        // Bottom-left
        Vertex2D {
            position: [-0.5, 0.5, 0.0],
            color: [1.0, 1.0, 1.0],
            tex_coord: [0.0, 1.0],
        },
    ]
}

/// Índices para el quad (dos triángulos)
pub fn create_quad_indices() -> Vec<u16> {
    vec![0, 1, 2, 2, 3, 0]
}
