// use winit::{
//   event::*,
//   event_loop::{ControlFlow, EventLoop},
//   window::WindowBuilder,
// };
// use wgpu::util::DeviceExt;
// use glyph_brush::*;
// use std::borrow::Cow;

// #[tokio::main]
// async fn run() {
//   // Inicialize o logger
//   env_logger::init();

//   // Criando o loop de eventos e a janela
//   let event_loop = EventLoop::new();
//   let window = WindowBuilder::new().build(&event_loop).unwrap();

//   // Inicializando wgpu
//   let instance = wgpu::Instance::new(wgpu::Backends::all());
//   let surface = unsafe { instance.create_surface(&window) };
//   let adapter = instance
//       .request_adapter(&wgpu::RequestAdapterOptions {
//           power_preference: wgpu::PowerPreference::default(),
//           compatible_surface: Some(&surface),
//       })
//       .await
//       .unwrap();

//   let (device, queue) = adapter
//       .request_device(
//           &wgpu::DeviceDescriptor {
//               features: wgpu::Features::empty(),
//               limits: wgpu::Limits::default(),
//               label: None,
//           },
//           None,
//       )
//       .await
//       .unwrap();

//   // Configurando a superfície e o swapchain
//   let size = window.inner_size();
//   let mut config = wgpu::SurfaceConfiguration {
//       usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
//       format: surface.get_supported_formats(&adapter)[0],
//       width: size.width,
//       height: size.height,
//       present_mode: wgpu::PresentMode::Fifo,
//   };
//   surface.configure(&device, &config);

//   // Configurando o GlyphBrush para renderizar texto
//   let mut glyph_brush = GlyphBrushBuilder::using_font_bytes(include_bytes!(
//       "path_para_fonte.ttf" // Especifique o caminho da sua fonte
//   )).unwrap().build(&device, config.format);

//   // Texto para renderizar
//   let section = Section {
//       screen_position: (30.0, 30.0),
//       text: vec![Text::new("Olá, mundo!")
//           .with_color([0.0, 0.0, 0.0, 1.0])
//           .with_scale(40.0)],
//       ..Section::default()
//   };

//   event_loop.run(move |event, _, control_flow| {
//       *control_flow = ControlFlow::Poll;
//       match event {
//           Event::WindowEvent {
//               ref event,
//               window_id,
//           } if window_id == window.id() => match event {
//               WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
//               WindowEvent::Resized(new_size) => {
//                   config.width = new_size.width;
//                   config.height = new_size.height;
//                   surface.configure(&device, &config);
//               }
//               _ => {}
//           },
//           Event::RedrawRequested(_) => {
//               let frame = surface
//                   .get_current_texture()
//                   .expect("Failed to acquire next swap chain texture");
//               let view = frame
//                   .texture
//                   .create_view(&wgpu::TextureViewDescriptor::default());

//               // Preparar para renderizar
//               let mut encoder =
//                   device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

//               // Renderizando texto
//               glyph_brush.queue(section.clone());
//               glyph_brush.draw_queued(
//                   &device,
//                   &mut encoder,
//                   &view,
//                   config.width,
//                   config.height,
//               ).expect("Draw queued");

//               queue.submit(Some(encoder.finish()));
//               frame.present();
//           }
//           _ => {}
//       }
//       window.request_redraw();
//   });
// }
