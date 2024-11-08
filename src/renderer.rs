use glium::glutin::surface::WindowSurface;
use glium::index::PrimitiveType;
use glium::{Display, Frame, IndexBuffer, Program, Surface, VertexBuffer};

const QUAD_MAX_BATCHES: usize = 20000;
const QUAD_MAX_VERTICES: usize = 4 * QUAD_MAX_BATCHES;
const QUAD_MAX_INDICES: usize = 6 * QUAD_MAX_BATCHES;

#[derive(Copy, Clone)]
pub struct QuadVertex {
    pub position: [f32; 2],
    pub color: [f32; 4],
}
implement_vertex!(QuadVertex, position, color);

pub struct Renderer {
    quad_vb: VertexBuffer<QuadVertex>,
    quad_ib: IndexBuffer<u32>,
    quad_shader: glium::Program,
    quad_index_count: i32,
    quad_vertices: Vec<QuadVertex>,
}

impl Renderer {
    pub fn new(display: &Display<WindowSurface>) -> Self {
        let quad_vertices = Vec::with_capacity(QUAD_MAX_VERTICES);

        // define indices
        let mut quad_indices = Vec::with_capacity(QUAD_MAX_INDICES);
        {
            let mut offset = 0;
            let mut i = 0;
            while i < QUAD_MAX_INDICES {
                // first triangle
                quad_indices.push(offset + 0);
                quad_indices.push(offset + 1);
                quad_indices.push(offset + 2);
                // second triangle
                quad_indices.push(offset + 2);
                quad_indices.push(offset + 3);
                quad_indices.push(offset + 0);

                i += 6; // incremenet index with index count
                offset += 4; // increment offset with vertex count
            }
        }

        let quad_vb =
            VertexBuffer::empty_dynamic(display, QUAD_MAX_VERTICES).unwrap();
        let quad_ib = IndexBuffer::new(
            display,
            PrimitiveType::TrianglesList,
            &quad_indices,
        )
        .unwrap();

        let quad_vertex_src = include_str!("shaders/quad.vert");
        let quad_fragment_src = include_str!("shaders/quad.frag");

        let quad_shader = Program::from_source(
            display,
            quad_vertex_src,
            quad_fragment_src,
            None,
        )
        .unwrap();

        let quad_index_count = 0;

        Self {
            quad_vb,
            quad_ib,
            quad_shader,
            quad_index_count,
            quad_vertices,
        }
    }

    pub fn begin(&mut self) {
        // clear the vector without chaning the capacity
        self.quad_vertices.clear();
        self.quad_index_count = 0;
    }

    pub fn end(&mut self, target: &mut Frame) {
        if self.quad_index_count > 0 {
            // update the partion in gpu to match our vertices
            self.quad_vb
                .slice_mut(0..self.quad_vertices.len())
                .unwrap()
                .write(&self.quad_vertices);

            target
                .draw(
                    &self.quad_vb,
                    &self.quad_ib,
                    &self.quad_shader,
                    &glium::uniforms::EmptyUniforms,
                    &Default::default(),
                )
                .unwrap();
        }
    }

    pub fn draw_quad(
        &mut self,
        target: &mut Frame,
        world_pos: [f32; 2],
        color: [f32; 4],
    ) {
        if self.quad_index_count + 6 > QUAD_MAX_INDICES as i32 {
            self.end(target);
            self.begin();
        }

        let v1 = QuadVertex {
            position: [-0.5 + world_pos[0], -0.5 + world_pos[1]],
            color,
        };
        self.quad_vertices.push(v1);
        let v2 = QuadVertex {
            position: [-0.5 + world_pos[0], 0.5 + world_pos[1]],
            color,
        };
        self.quad_vertices.push(v2);
        let v3 = QuadVertex {
            position: [0.5 + world_pos[0], 0.5 + world_pos[1]],
            color,
        };
        self.quad_vertices.push(v3);
        let v4 = QuadVertex {
            position: [0.5 + world_pos[0], -0.5 + world_pos[1]],
            color,
        };
        self.quad_vertices.push(v4);

        self.quad_index_count += 6;
    }
}
