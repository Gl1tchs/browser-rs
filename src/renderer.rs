use glium::index::PrimitiveType;
use glium::{Display, IndexBuffer, Program, Surface, VertexBuffer};
use glium_glyph::{GlyphBrush, GlyphBrushBuilder};
use glyph_brush::ab_glyph::FontArc;
use glyph_brush::{HorizontalAlign, Layout, Section, Text, VerticalAlign};

use crate::camera::Camera;
use crate::lalg::mat4_to_array;

const QUAD_MAX_BATCHES: usize = 20000;
const QUAD_MAX_VERTICES: usize = 4 * QUAD_MAX_BATCHES;
const QUAD_MAX_INDICES: usize = 6 * QUAD_MAX_BATCHES;

#[derive(Copy, Clone)]
struct QuadVertex {
    pub position: [f32; 2],
    pub color: [f32; 4],
}

implement_vertex!(QuadVertex, position, color);

pub struct TextDrawConfig {
    pub screen_pos: (f32, f32),
    pub bounds: (f32, f32),
    pub fg_color: [f32; 4],
    pub bg_color: [f32; 4],
    pub h_align: HorizontalAlign,
    pub v_align: VerticalAlign,
}

impl Default for TextDrawConfig {
    fn default() -> Self {
        Self {
            screen_pos: (0.0, 0.0).into(),
            bounds: (f32::INFINITY, f32::INFINITY),
            fg_color: [0.0, 0.0, 0.0, 1.0],
            bg_color: [0.0, 0.0, 0.0, 0.0],
            h_align: HorizontalAlign::Left,
            v_align: VerticalAlign::Top,
        }
    }
}

pub struct Renderer<'a> {
    camera: Camera,
    screen_size: (u32, u32),
    aspect_ratio: f32,
    glyph_brush: GlyphBrush<'a, FontArc>,
    // quad resources
    quad_vb: VertexBuffer<QuadVertex>,
    quad_ib: IndexBuffer<u32>,
    quad_shader: Program,
    quad_index_count: i32,
    quad_vertices: Vec<QuadVertex>,
}

impl<'a> Renderer<'a> {
    pub fn new(display: &Display) -> Self {
        let quad_vertices = Vec::with_capacity(QUAD_MAX_VERTICES);

        // Define indices for quads
        let mut quad_indices = Vec::with_capacity(QUAD_MAX_INDICES);
        let mut offset = 0;
        let mut i = 0;
        while i < QUAD_MAX_INDICES {
            quad_indices.push(offset + 0);
            quad_indices.push(offset + 1);
            quad_indices.push(offset + 2);
            quad_indices.push(offset + 2);
            quad_indices.push(offset + 3);
            quad_indices.push(offset + 0);

            i += 6;
            offset += 4;
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

        let camera = Camera::default();

        let screen_size = (0, 0);
        let aspect_ratio = 0.0;

        // Initialize glyph_brush with font
        // TODO: let user select their font
        let font = FontArc::try_from_slice(include_bytes!(
            "../assets/fonts/Roboto-Regular.ttf"
        ) as &[u8])
        .unwrap();

        let glyph_brush = GlyphBrushBuilder::using_font(font).build(display);

        Self {
            camera,
            screen_size,
            aspect_ratio,
            glyph_brush,
            quad_vb,
            quad_ib,
            quad_shader,
            quad_index_count,
            quad_vertices,
        }
    }

    pub fn update_dimension(&mut self, dims: (u32, u32)) {
        self.screen_size = dims;

        self.aspect_ratio = dims.0 as f32 / dims.1 as f32;
        self.camera.aspect_ratio = self.aspect_ratio;
    }

    pub fn begin(&mut self) {
        // Clear the quad vertices buffer without changing capacity
        self.quad_vertices.clear();
        self.quad_index_count = 0;
    }

    pub fn end(&mut self, display: &mut Display) {
        let mut target = display.draw();
        target.clear_color(1.0, 1.0, 1.0, 1.0);

        let view_matrix = mat4_to_array(&self.camera.get_view());
        let projection_matrix = mat4_to_array(&self.camera.get_projection());

        if self.quad_index_count > 0 {
            // Update GPU buffer with vertices
            self.quad_vb
                .slice_mut(0..self.quad_vertices.len())
                .unwrap()
                .write(&self.quad_vertices);

            let uniforms = uniform! {
                view: view_matrix,
                proj: projection_matrix,
            };

            target
                .draw(
                    &self.quad_vb,
                    &self.quad_ib,
                    &self.quad_shader,
                    &uniforms,
                    &Default::default(),
                )
                .unwrap();
        }

        // Render all queued text
        self.glyph_brush.draw_queued(display, &mut target);

        target.finish().unwrap();
    }

    pub fn draw_quad(
        &mut self,
        display: &mut Display,
        screen_pos: (f32, f32),
        bounds: (f32, f32),
        color: [f32; 4],
    ) {
        if self.quad_index_count + 6 > QUAD_MAX_INDICES as i32 {
            self.end(display);
            self.begin();
        }

        let ndc_x = ((screen_pos.0 / self.screen_size.0 as f32) * 2.0 - 1.0)
            * self.aspect_ratio;
        let ndc_y = 1.0 - (screen_pos.1 / self.screen_size.1 as f32) * 2.0;

        let half_width = bounds.0 / self.screen_size.0 as f32;
        let half_height = bounds.1 / self.screen_size.1 as f32;

        let v1 = QuadVertex {
            position: [ndc_x - half_width, ndc_y - half_height],
            color,
        };
        let v2 = QuadVertex {
            position: [ndc_x - half_width, ndc_y + half_height],
            color,
        };
        let v3 = QuadVertex {
            position: [ndc_x + half_width, ndc_y + half_height],
            color,
        };
        let v4 = QuadVertex {
            position: [ndc_x + half_width, ndc_y - half_height],
            color,
        };

        self.quad_vertices.push(v1);
        self.quad_vertices.push(v2);
        self.quad_vertices.push(v3);
        self.quad_vertices.push(v4);

        self.quad_index_count += 6;
    }

    pub fn draw_text(
        &mut self,
        display: &mut Display,
        text: &str,
        size: f32,
        cfg: TextDrawConfig,
    ) {
        let section = Section::default()
            .with_screen_position(cfg.screen_pos)
            .with_bounds(cfg.bounds)
            .with_text(vec![Text::new(text)
                .with_scale(size)
                .with_color(cfg.fg_color)
                .with_z(1.0)])
            .with_layout(
                Layout::default().h_align(cfg.h_align).v_align(cfg.v_align),
            );

        // if background color is not transparent then a draw quad
        if cfg.bg_color[3] != 0.0 {
            let line_count = text.chars().filter(|c| *c == '\n').count() + 1;
            let line_height = size * line_count as f32 * 2.0;

            let quad_bounds = (
                if cfg.bounds.0 == f32::INFINITY {
                    self.screen_size.1 as f32
                } else {
                    cfg.bounds.0
                },
                line_height,
            );

            self.draw_quad(display, cfg.screen_pos, quad_bounds, cfg.bg_color);
        }

        self.glyph_brush.queue(section);
    }
}
