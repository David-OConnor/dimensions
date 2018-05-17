use gfx;
use gfx::traits::FactoryExt;
use gfx::Device;
use gfx_window_glutin as gfx_glutin;

use glutin;
use glutin::{GlContext, GlRequest};
use glutin::Api::OpenGl;

pub type ColorFormat = gfx::format::Srgba8;
pub type DepthFormat = gfx::format::DepthStencil;

const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

pub fn main() {
    let mut events_loop = glutin::EventsLoop::new();
    let windowbuilder = glutin::WindowBuilder::new()
        .with_title("Triangle Example".to_string())
        .with_dimensions(512, 512);
    let contextbuilder = glutin::ContextBuilder::new()
        .with_gl(GlRequest::Specific(OpenGl,(3,2)))
        .with_vsync(true);
    let (window, mut device, mut factory, color_view, mut depth_view) =
        gfx_glutin::init::<ColorFormat, DepthFormat>(windowbuilder, contextbuilder, &events_loop);

    let mut running = true;
    while running {
        events_loop.poll_events(|event| {
            if let glutin::Event::WindowEvent { event, .. } = event {
                match event {
                    glutin::WindowEvent::Closed |
                    glutin::WindowEvent::KeyboardInput {
                        input: glutin::KeyboardInput {
                            virtual_keycode: Some(glutin::VirtualKeyCode::Escape), ..
                        }, ..
                    } => running = false,
                    _ => {}
                }
            }
        });

        window.swap_buffers().unwrap();
        device.cleanup();
    }
}