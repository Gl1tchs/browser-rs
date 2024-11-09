use glm::Mat4;

pub struct Camera {
    pub position: [f32; 2],
    pub screen_size: (u32, u32),
    pub near_clip: f32,
    pub far_clip: f32,
}

impl Camera {
    pub fn get_projection(&self) -> Mat4 {
        glm::ortho(
            0.0,
            self.screen_size.0 as f32 * 0.5,
            self.screen_size.1 as f32,
            0.0,
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
            screen_size: (0, 0).into(),
            near_clip: -1.0,
            far_clip: 1.0,
        }
    }
}
