#[macro_use]
extern crate glium;
extern crate nalgebra_glm as glm;

mod camera;
mod html_renderer;
mod lalg;
mod renderer;

use glium::glutin;
use glium::glutin::{Api, GlProfile, GlRequest};
use glutin::event::{Event, WindowEvent};
use glutin::event_loop::{ControlFlow, EventLoop};
use html_renderer::HtmlRenderer;

use crate::renderer::Renderer;

fn main() {
    let event_loop = EventLoop::new();
    let window = glutin::window::WindowBuilder::new();
    let context = glutin::ContextBuilder::new()
        .with_gl_profile(GlProfile::Core)
        .with_gl(GlRequest::Specific(Api::OpenGl, (3, 2)))
        .with_srgb(true);
    let mut display =
        glium::Display::new(window, context, &event_loop).unwrap();

    let mut renderer = Renderer::new(&display);

    let mut html_renderer = HtmlRenderer::new();
    html_renderer.load_html(include_str!("../assets/test.html"));

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
            html_renderer.render(&mut renderer, &mut display);
        }
        renderer.end(&mut display);
    });
}
