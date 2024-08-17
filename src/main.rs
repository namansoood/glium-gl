#![windows_subsystem = "windows"]

use glium::glutin::{
    dpi::LogicalSize,
    event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::Fullscreen,
    window::WindowBuilder,
    ContextBuilder,
};
use glium::{Display, Surface};
use imgui::Context;

use imgui_glium_renderer::Renderer;
use imgui_winit_support::{HiDpiMode, WinitPlatform};

fn main() {
    let (width, height) = get_config();

    let event_loop = EventLoop::new();

    let wb = WindowBuilder::new()
        .with_title("Sprite Loader")
        .with_inner_size(LogicalSize::new(width, height))
        .with_resizable(false);
    let cb = ContextBuilder::new();

    let display = Display::new(wb, cb, &event_loop).unwrap();

    let mut is_fullscreen = false;

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
                WindowEvent::KeyboardInput { input, .. } => {
                    if let KeyboardInput {
                        virtual_keycode: Some(VirtualKeyCode::F12),
                        state: ElementState::Pressed,
                        ..
                    } = input
                    {
                        if is_fullscreen {
                            window.set_fullscreen(None);
                        } else {
                            window.set_fullscreen(Some(Fullscreen::Borderless(None)));
                        }
                        is_fullscreen = !is_fullscreen;
                    }
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
                frame.clear_color(0.0, 0.0, 0.0, 1.0);

                platform.prepare_render(&ui, &window);

                imgui_renderer
                    .render(&mut frame, imgui.render())
                    .expect("Failed to render imgui.");

                frame.finish().unwrap();
            }
            _ => (),
        };
    });
}

fn get_config() -> (u32, u32) {
    let conf = ini::ini!("./config.ini");

    let default_width: u32 = 1366;
    let default_height: u32 = 800;

    let default_width_str = default_width.to_string();
    let default_height_str = default_height.to_string();

    let width: u32 = conf["window"]["width"]
        .as_ref()
        .unwrap_or(&default_width_str)
        .parse::<u32>()
        .unwrap_or(default_width);
    let height: u32 = conf["window"]["height"]
        .as_ref()
        .unwrap_or(&default_height_str)
        .parse::<u32>()
        .unwrap_or(default_height);

    (width, height)
}
