#[macro_use]
extern crate glium;

mod app;
mod renderer;

use glium::winit::event_loop::{ControlFlow, EventLoop};
use html::parser::Parser;

use app::App;

fn main() {
    let input = include_str!("../assets/test.html");
    let parser = Parser::new(input);
    println!("{:?}", parser.parse());

    let event_loop = EventLoop::builder().build().unwrap();
    event_loop.set_control_flow(ControlFlow::Wait);

    let mut app = App::default();
    let _ = event_loop.run_app(&mut app);
}
