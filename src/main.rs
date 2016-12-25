#[macro_use] extern crate gfx;
extern crate gfx_window_glutin;
extern crate glutin;

use gfx::{Factory, Device};
use gfx::traits::FactoryExt;

pub type ColorFormat = gfx::format::Rgba8;
pub type DepthFormat = gfx::format::DepthStencil;

gfx_defines!{
    vertex Vertex {
        pos: [f32; 2] = "in_pos",
        color: [f32; 4] = "in_color",
    }

    pipeline pipe {
        vertex_buffer: gfx::VertexBuffer<Vertex> = (),
        out: gfx::RenderTarget<ColorFormat> = "pixel",
        displacement: gfx::Global<[f32; 2]> = "displacement",
        scale: gfx::Global<[f32; 2]> = "scale",
    }
}

fn main() {
    let w = 1024;
    let h = 768;

    let builder = glutin::WindowBuilder::new()
        .with_title("Test".to_string())
        .with_dimensions(w as u32, h as u32)
        .with_vsync();
    let (window, mut device, mut factory, main_color, mut main_depth) = gfx_window_glutin::init::<ColorFormat, DepthFormat>(builder);
    let mut encoder: gfx::Encoder<_, _> = factory.create_command_buffer().into();
    let pso = factory.create_pipeline_simple(
        include_bytes!("../shader/element.vert"),
        include_bytes!("../shader/element.frag"),
        pipe::new()
    ).unwrap();

    let mut size_in_pixels = (w, h);
    let scale = 30.0;

    let default_vertex = Vertex {
        pos: [0.0, 0.0],
        color: [0.0, 1.0, 0.0, 1.0],
    };

    let vertices = [
        Vertex {
            pos: [0.0, 0.5],
            color: [1.0, 0.0, 0.0, 1.0],
        },
        Vertex {
            pos: [0.5, 0.0],
            color: [0.0, 1.0, 0.0, 1.0],
        },
        Vertex {
            pos: [0.0, -0.5],
            color: [0.0, 0.0, 1.0, 1.0],
        }
    ];
    let indices = [0u16, 1, 2];

    let vertex_buffer = factory.create_vertex_buffer(&vertices);
    let mut data = pipe::Data {
        vertex_buffer: vertex_buffer,
        out: main_color,
        displacement: [0.0, 0.0],
        scale: [scale/(size_in_pixels.0 as f32), scale/(size_in_pixels.1 as f32)],
    };

    let index_buffer = factory.create_buffer_const(&indices, gfx::BufferRole::Index, gfx::Bind::empty()).unwrap();

    let slice = gfx::Slice {
        start: 0,
        end: indices.len() as u32,
        base_vertex: 0,
        instances: None,
        buffer: gfx::IndexBuffer::Index16(index_buffer),
    };

    'main: loop {
        // loop over events
        for event in window.poll_events() {
            match event {
                glutin::Event::Resized(w, h) => {
                    gfx_window_glutin::update_views(&window, &mut data.out, &mut main_depth);
                    size_in_pixels = (w as i32, h as i32);
                }
                glutin::Event::Closed => break 'main,
                _ => {},
            }
        }

        encoder.update_buffer(&data.vertex_buffer, &vertices, 0).unwrap();

        data.displacement = [0.0, 0.0];
        data.scale = [1.0, 1.0];

        // draw a frame
        encoder.clear(&data.out, [0.0, 0.0, 0.0, 1.0]);
        encoder.draw(&slice, &pso, &data);
        encoder.flush(&mut device);
        window.swap_buffers().unwrap();
        device.cleanup();
    }
}
