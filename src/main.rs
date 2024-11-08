#[macro_use]
extern crate glium;
extern crate nalgebra_glm as glm;

mod camera;
mod lalg;
mod renderer;

use glium::glutin;
use glium::glutin::{Api, GlProfile, GlRequest};
use glutin::event::{Event, WindowEvent};
use glutin::event_loop::{ControlFlow, EventLoop};
use glyph_brush::{HorizontalAlign, VerticalAlign};
use renderer::TextDrawConfig;

use crate::renderer::Renderer;
use html::parser::Parser;

fn main() {
    let input = include_str!("../assets/test.html");
    let parser = Parser::new(input);
    println!("{:?}", parser.parse());

    let event_loop = EventLoop::new();
    let window = glutin::window::WindowBuilder::new();
    let context = glutin::ContextBuilder::new()
        .with_gl_profile(GlProfile::Core)
        .with_gl(GlRequest::Specific(Api::OpenGl, (3, 2)))
        .with_srgb(true);
    let mut display =
        glium::Display::new(window, context, &event_loop).unwrap();

    let mut renderer = Renderer::new(&display);

    event_loop.run(move |event, _tgt, control_flow| {
        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit;
                }
                _ => (),
            },
            _ => (),
        }

        let screen_dims = display.get_framebuffer_dimensions();
        renderer.update_dimension(screen_dims);

        renderer.begin();
        {
            renderer.draw_text(
                &mut display,
                "Example Text\nExample Text2",
                24.0,
                TextDrawConfig {
                    bg_color: [1.0, 0.0, 0.0, 1.0],
                    ..Default::default()
                },
            );

            renderer.draw_text(
                &mut display,
                "Hello, World",
                64.0,
                TextDrawConfig {
                    screen_pos: (
                        screen_dims.0 as f32 / 2.0,
                        screen_dims.1 as f32 / 2.0,
                    ),
                    bounds: (screen_dims.0 as f32, screen_dims.1 as f32),
                    bg_color: [0.0, 0.0, 1.0, 1.0],
                    h_align: HorizontalAlign::Center,
                    v_align: VerticalAlign::Center,
                    ..Default::default()
                },
            );
        }
        renderer.end(&mut display);
    });
}
