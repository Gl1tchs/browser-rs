use glm::Mat4;

pub fn mat4_to_array(mat: &Mat4) -> [[f32; 4]; 4] {
    let slice = mat.as_slice();
    [
        [slice[0], slice[1], slice[2], slice[3]],
        [slice[4], slice[5], slice[6], slice[7]],
        [slice[8], slice[9], slice[10], slice[11]],
        [slice[12], slice[13], slice[14], slice[15]],
    ]
}
