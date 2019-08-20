// https://github.com/unknownue/vulkan-tutorial-rust/tree/master/src/tutorials

use std::ffi::CString;
use std::ptr;

use winit::{ ControlFlow, Event, EventsLoop, VirtualKeyCode, WindowEvent };

use ash::version::EntryV1_1;
use ash::version::InstanceV1_1;
use ash::vk;


// constants
const WINDOW_TITLE : &'static str = "main_app";
const WINDOW_DIM : (u32, u32) = (1024, 768);

struct APP
{
    events_loop : EventsLoop,
    window      : winit::Window,
    
    entry       : ash::Entry,
    instance    : ash::Instance,
}

impl APP
{
    pub fn new() -> APP
    {
        let events_loop = EventsLoop::new();
        let window = APP::init_window(&events_loop);
        
        let entry = ash::Entry::new();
        let instance = APP::create_instance(&entry); 

        APP
        {
            events_loop,
            window,
            entry,
            instance,
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

    fn create_instance(entry : &ash::Entry) -> ash::Instance
    {
        
    }
}

fn main()
{
    let mut app = APP::new();
    app.main_loop();
}
