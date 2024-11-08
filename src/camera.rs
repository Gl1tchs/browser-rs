use glm::Mat4;

pub struct Camera {
    pub position: [f32; 2],
    pub aspect_ratio: f32,
    pub zoom_level: f32,
    pub near_clip: f32,
    pub far_clip: f32,
}

impl Camera {
    pub fn get_projection(&self) -> Mat4 {
        glm::ortho(
            -self.aspect_ratio * self.zoom_level,
            self.aspect_ratio * self.zoom_level,
            -self.zoom_level,
            self.zoom_level,
            self.near_clip,
            self.far_clip,
        )
    }

    pub fn get_view(&self) -> Mat4 {
        let transform = Mat4::identity();
        transform.prepend_translation(&glm::vec3(
            self.position[0],
            self.position[1],
            0.0,
        ));

        transform.try_inverse().unwrap()
    }
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            position: [0.0, 0.0],
            aspect_ratio: 1.77,
            zoom_level: 1.0,
            near_clip: -1.0,
            far_clip: 1.0,
        }
    }
}
