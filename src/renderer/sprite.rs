use glam::{Vec2, Vec3, Mat4};

/// Representa un sprite 2D con transformación
#[derive(Debug, Clone)]
pub struct Sprite2D {
    pub position: Vec2,
    pub rotation: f32,  // En radianes
    pub scale: Vec2,
    pub color: Vec3,    // RGB
    pub z_order: f32,   // Profundidad para sorting
}

impl Sprite2D {
    pub fn new(position: Vec2) -> Self {
        Self {
            position,
            rotation: 0.0,
            scale: Vec2::new(1.0, 1.0),
            color: Vec3::new(1.0, 1.0, 1.0),
            z_order: 0.0,
        }
    }

    /// Calcula la matriz de transformación del sprite
    pub fn model_matrix(&self) -> Mat4 {
        let translation = Mat4::from_translation(Vec3::new(
            self.position.x,
            self.position.y,
            self.z_order,
        ));
        
        let rotation = Mat4::from_rotation_z(self.rotation);
        
        let scale = Mat4::from_scale(Vec3::new(
            self.scale.x,
            self.scale.y,
            1.0,
        ));

        translation * rotation * scale
    }
}

impl Default for Sprite2D {
    fn default() -> Self {
        Self::new(Vec2::ZERO)
    }
}

/// Cámara ortográfica 2D
#[derive(Debug, Clone)]
pub struct Camera2D {
    pub position: Vec2,
    pub zoom: f32,
    pub rotation: f32,
    pub viewport_size: Vec2,
}

impl Camera2D {
    pub fn new(viewport_width: f32, viewport_height: f32) -> Self {
        Self {
            position: Vec2::ZERO,
            zoom: 1.0,
            rotation: 0.0,
            viewport_size: Vec2::new(viewport_width, viewport_height),
        }
    }

    /// Actualiza el tamaño del viewport
    pub fn update_viewport(&mut self, width: f32, height: f32) {
        self.viewport_size = Vec2::new(width, height);
    }

    /// Calcula la matriz de vista
    pub fn view_matrix(&self) -> Mat4 {
        let translation = Mat4::from_translation(Vec3::new(
            -self.position.x,
            -self.position.y,
            0.0,
        ));
        
        let rotation = Mat4::from_rotation_z(-self.rotation);
        
        rotation * translation
    }

    /// Calcula la matriz de proyección ortográfica
    pub fn projection_matrix(&self) -> Mat4 {
        let half_width = (self.viewport_size.x / 2.0) / self.zoom;
        let half_height = (self.viewport_size.y / 2.0) / self.zoom;

        Mat4::orthographic_rh(
            -half_width,
            half_width,
            -half_height,
            half_height,
            -1.0,
            1.0,
        )
    }

    /// Calcula la matriz view-projection combinada
    pub fn view_projection_matrix(&self) -> Mat4 {
        self.projection_matrix() * self.view_matrix()
    }
}

impl Default for Camera2D {
    fn default() -> Self {
        Self::new(800.0, 600.0)
    }
}
