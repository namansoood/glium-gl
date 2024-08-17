use glium::Surface;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoopBuilder};

fn main() {
    let event_loop = EventLoopBuilder::new().build();
    let (_window, display) = glium::backend::glutin::SimpleWindowBuilder::new()
        .with_title("Sprite Loader")
        .build(&event_loop);

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

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
            Event::RedrawEventsCleared => {
                let mut frame = display.draw();
                frame.clear_color(0.0, 0.0, 0.0, 1.0);
                frame.finish().unwrap();
            }
            _ => (),
        };
    });
}
