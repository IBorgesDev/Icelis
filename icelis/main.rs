use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use pixels::{Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use rusttype::{Font, Scale};

fn main() {
    // Carrega a fonte embutida
    let font_data: &[u8] = include_bytes!("/System/Library/Fonts/helvetica.ttc"); // Defina o caminho para sua fonte .ttf
    let font = Font::try_from_bytes(font_data).unwrap();

    // Escala para o texto
    let scale = Scale::uniform(40.0);

    // Cria o EventLoop
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Minha Janela")
        .with_inner_size(LogicalSize::new(800, 600))
        .build(&event_loop)
        .unwrap();

    // Cria a área de renderização para os pixels
    let window_size = window.inner_size();
    let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
    let mut pixels = Pixels::new(800, 600, surface_texture).unwrap();

    // Loop de eventos
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => {
                *control_flow = ControlFlow::Exit;
            }
            Event::RedrawRequested(_) => {
                let mut frame = pixels.get_frame_mut();
                // Preenchendo o fundo de branco
                for pixel in frame.chunks_exact_mut(4) {
                    pixel.copy_from_slice(&[255, 255, 255, 255]); // Fundo branco
                }

                // Cria um buffer para renderizar texto
                let mut text_buffer = vec![0u8; frame.len()];
                let position = rusttype::point(100.0, 100.0);
                let text = "Olá, Mundo!";

                // Renderiza o texto no buffer
                for glyph in font.layout(text, scale, position) {
                    if let Some(bounding_box) = glyph.pixel_bounding_box() {
                        glyph.draw(|x, y, v| {
                            let x = (bounding_box.min.x + x as i32) as usize;
                            let y = (bounding_box.min.y + y as i32) as usize;
                            let offset = (y * 800 + x) * 4;

                            if offset < text_buffer.len() {
                                text_buffer[offset] = (1.0 - v) as u8 * 0;
                                text_buffer[offset + 1] = (1.0 - v) as u8 * 0;
                                text_buffer[offset + 2] = (1.0 - v) as u8 * 0;
                                text_buffer[offset + 3] = 255; // Opacidade
                            }
                        });
                    }
                }

                // Renderiza o texto no frame
                frame.copy_from_slice(&text_buffer);

                pixels.render().unwrap();
            }
            _ => {}
        }

        window.request_redraw();
    });
}
