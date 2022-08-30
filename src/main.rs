#[macro_use]
extern crate glium;
extern crate image;

use std::io::Cursor;
use glium::{glutin,Surface,texture};
use std::fs;



#[derive(Copy,Clone)]
struct Vertex{
    position : [f32;2],
    tex_coords : [f32;2],
}
implement_vertex!(Vertex,position,tex_coords);

fn main() {
    let mut event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new();
    let cb = glutin::ContextBuilder::new().with_depth_buffer(24);
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    let texture = build_image(&display);
    let shape = build_shape(&display);
    
    let ver_sh = fs::read_to_string("src/vert.glsl").unwrap().to_string();
    let frag_sh = fs::read_to_string("src/frag.glsl").unwrap().to_string();
    
    let program = glium::Program::from_source(&display, &ver_sh,&frag_sh, None).unwrap();
    let mut t = 0.0;
    event_loop.run(move |ev, _, control_flow| {
        let next_frame_time =
            std::time::Instant::now() + std::time::Duration::from_nanos(16_666_667);
        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);
        
        match ev {
            glutin::event::Event::WindowEvent { event, .. } => match event {    
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                }
                _ => return,
            },
            _ => (),
        }



        let mut target = display.draw();
        target.clear_color(1.0, 0.0, 0.0, 1.0);
        t = move_ferris(t);
        let uniforms = uniform! {
            matrix: [
                [0.5 + t, 0.0, 0.0, 0.0],
                [0.0, 0.5, 0.0, 0.0],
                [0.0, 0.0, 0.5, 0.0],
                [0.0 ,0.0, 0.0, 1.0f32],
            ],
            tex: &texture,
        };
        

        target.draw(&shape,
            glium::index::NoIndices(glium::index::PrimitiveType::TriangleStrip),
            &program,
            &uniforms,
            &Default::default()).unwrap();
        target.finish().unwrap();
    });

}
fn build_image(display : &glium::Display) -> glium::texture::SrgbTexture2d{
    let image = image::load(Cursor::new(&include_bytes!("../assets/ferris.png")),
        image::ImageFormat::Png).unwrap().to_rgba8();
    let image_dimensions = image.dimensions();
    let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
    let diffuse_tex = glium::texture::SrgbTexture2d::new(display, image).unwrap();
    diffuse_tex
}
fn build_shape(display: &glium::Display) -> glium::VertexBuffer<Vertex>{
    let shape = glium::vertex::VertexBuffer::new(display, &[
        Vertex { position: [-1.0,  1.0], tex_coords: [0.0, 1.0]},
        Vertex { position: [ 1.0,  1.0], tex_coords: [1.0, 1.0]},
        Vertex { position: [-1.0, -1.0], tex_coords: [0.0, 0.0]},
        Vertex { position: [ 1.0, -1.0], tex_coords: [1.0, 0.0]},
    ]).unwrap();
    shape
}
fn move_ferris(mut t : f32)-> f32 {
    if t > 1.0 {
        t = 0.0;
    }
    t+0.002
}