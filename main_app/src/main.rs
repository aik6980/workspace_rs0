// https://github.com/unknownue/vulkan-tutorial-rust/tree/master/src/tutorials

use winit::{ ControlFlow, Event, EventsLoop, VirtualKeyCode, WindowEvent };

// constants
const WINDOW_TITLE : &'static str = "main_app";
const WINDOW_DIM : (u32, u32) = (1024, 768);

struct APP
{
    events_loop : EventsLoop,
    window      : winit::Window,
}

impl APP
{
    pub fn new() -> APP
    {
        let events_loop = EventsLoop::new();
        let window = APP::init_window(&events_loop);

        APP
        {
            events_loop,
            window,
        }
    }

    fn init_window(events_loop : &EventsLoop) -> winit::Window
    {
        winit::WindowBuilder::new()
            .with_title(WINDOW_TITLE)
            .with_dimensions(WINDOW_DIM.into())
            .build(&events_loop)
            .expect("Failed to create window")
    }

    fn main_loop(&mut self)
    {
        self.events_loop.run_forever(|event|
        {
            match event 
            {
                Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => winit::ControlFlow::Break,
                _ => winit::ControlFlow::Continue,
            }
        });
    }
}

fn main()
{
    let mut app = APP::new();
    app.main_loop();
}
