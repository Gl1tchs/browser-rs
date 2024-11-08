use glium::glutin::surface::WindowSurface;
use glium::winit::application::ApplicationHandler;
use glium::winit::event::WindowEvent;
use glium::winit::event_loop::ActiveEventLoop;
use glium::winit::window::{Window, WindowId};
use glium::{Display, Surface};

use crate::renderer::Renderer;

#[derive(Default)]
pub struct App {
    window: Option<Window>,
    display: Option<Display<WindowSurface>>,
    initialized: bool,
    // drawing state
    renderer: Option<Renderer>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let (window, display) =
            glium::backend::glutin::SimpleWindowBuilder::new()
                .build(event_loop);

        self.window = Some(window);
        self.display = Some(display);

        if !self.initialized {
            self.renderer = Some(Renderer::new(self.display.as_ref().unwrap()));

            self.initialized = true;
        }
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                println!("The close button was pressed; stopping");
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                let mut target = self.display.as_ref().unwrap().draw();
                target.clear_color(0.0, 0.0, 1.0, 1.0);

                if let Some(r) = &mut self.renderer {
                    r.begin();

                    r.draw_quad(&mut target, [-0.5, 0.0], [1.0, 0.0, 0.0, 1.0]);
                    r.draw_quad(&mut target, [0.8, 0.0], [0.0, 1.0, 1.0, 1.0]);

                    r.end(&mut target);
                }

                target.finish().unwrap();

                // TODO: since this is a static page renderer we can just call this once
                // we only need to redraw the window if camera position changes (page scroll)
                self.window.as_ref().unwrap().request_redraw();
            }
            _ => (),
        }
    }
}
