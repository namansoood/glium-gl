use glium::glutin::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
    ContextBuilder,
};
use glium::{Display, Surface};
use imgui::Context;

use imgui_glium_renderer::Renderer;
use imgui_winit_support::{HiDpiMode, WinitPlatform};

fn main() {
    let event_loop = EventLoop::new();

    let wb = WindowBuilder::new().with_title("Glium with ImGui");
    let cb = ContextBuilder::new();

    let display = Display::new(wb, cb, &event_loop).unwrap();

    let mut imgui = Context::create();
    imgui.set_ini_filename(None);

    let mut platform = WinitPlatform::init(&mut imgui);
    {
        let gl_window = display.gl_window();
        let window = gl_window.window();
        platform.attach_window(imgui.io_mut(), window, HiDpiMode::Rounded);
    }

    let mut imgui_renderer = Renderer::init(&mut imgui, &display).unwrap();

    event_loop.run(move |event, _, control_flow| {
        let gl_window = display.gl_window();
        let window = gl_window.window();
        platform.handle_event(imgui.io_mut(), &window, &event);

        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit;
                }
                WindowEvent::DroppedFile(path) => {
                    // TODO: handle the dropped file
                    println!("File dropped: {:?}", path);
                }
                _ => (),
            },
            Event::MainEventsCleared => {
                platform
                    .prepare_frame(imgui.io_mut(), &window)
                    .expect("Failed to prepare frame");
                let ui = imgui.frame();

                ui.text("Sup?!?!");
                if ui.button("Click me!") {
                    println!("Button clicked!");
                }

                let mut frame = display.draw();
                frame.clear_color(0.0, 0.0, 1.0, 1.0);

                platform.prepare_render(&ui, &window);

                imgui_renderer.render(&mut frame, imgui.render()).unwrap();

                frame.finish().unwrap();
            }
            _ => (),
        };
    });
}
