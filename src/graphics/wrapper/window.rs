use super::RenderCaller;
use crate::{channels::{PackingToWindowReceiver, WindowToPackingSender}, graphics::GraphicsCapabilities};
use glutin::event::{Event, WindowEvent};
use glutin::event_loop::ControlFlow;

pub struct Window {
    event_loop: Option<glutin::event_loop::EventLoop<()>>,
    context: glutin::ContextWrapper<glutin::PossiblyCurrent, glutin::window::Window>,
    render_caller: RenderCaller,
    render_messages: PackingToWindowReceiver,
    capabilities_sender : WindowToPackingSender
}

pub type EventHandler = Box<dyn FnMut(glutin::event::Event<()>) + Send + 'static>;

impl Window {
    ///
    /// unsafe, since calling twice on the same thread is likely to lead to serious trouble.
    /// Also, extremely stateful.
    pub unsafe fn new(rx: PackingToWindowReceiver, packing_tx : WindowToPackingSender) -> Window {
        let el = glutin::event_loop::EventLoop::new();
        let wb = glutin::window::WindowBuilder::new()
            .with_title("Hello world!")
            .with_inner_size(glutin::dpi::LogicalSize::new(1024.0, 768.0));
        let windowed_context = glutin::ContextBuilder::new()
            .build_windowed(wb, &el)
            .unwrap();

        let windowed_context = windowed_context.make_current().unwrap();

        println!("Loading gl!");
        gl::load_with(|s| windowed_context.get_proc_address(s) as *const _);

        gl::ClearColor(0.3, 0.3, 0.5, 1.0);

        //TODO: DEPTH TESTING IS OPTIONAL, NOT REQUIRED!
        gl::Enable(gl::DEPTH_TEST);
        gl::DepthFunc(gl::LESS);
        gl::ClearColor(0.6, 0.6, 0.6, 1.0);
        gl::Enable(gl::CULL_FACE);
        gl::CullFace(gl::BACK);

        let render_caller = RenderCaller::new();
        //unsafe { render_caller.load_shaders("shaders") };

        let res = Window {
            event_loop: Some(el),
            context: windowed_context,
            render_caller,
            render_messages: rx,
            capabilities_sender : packing_tx
        };
        res.capabilities_sender.channel_sender.send(res.get_capabilities()).unwrap();

        res
    }

    fn get_capabilities(&self) -> GraphicsCapabilities {
        GraphicsCapabilities {
            vbo_count : self.render_caller.get_vbo_count(),
            texture_names : self.render_caller.get_texture_names()
        }
    }

    unsafe fn render(&mut self) {
        // Try getting the lock; only render if there's render messages available.
        // TODO: THIS HAS CHANGED BECAUSE NEW RENDER SENDER SYSTEM
        let render_messages = self.render_messages.render_pack.try_lock();

        if let Ok(_) = render_messages {
            let mut render_messages = render_messages.unwrap();

            if let Some(messages) = render_messages.take() {
                //println!("heey!");
                for message in messages.iter() {
                    self.render_caller.read_message(message);
                }

                self.context.swap_buffers().unwrap();
            }
        }
    }

    ///
    /// Starts this graphics object
    ///
    /// NOTE: THE EXECUTION GOES TO THE GRAPHICS OBJECT WHEN THIS IS CALLED!
    ///
    pub unsafe fn run(mut self, mut event_handler: EventHandler) {
        if let Some(el) = self.event_loop.take() {
            el.run(move |event, _, control_flow| {
                *control_flow = ControlFlow::Poll;

                match event {
                    Event::LoopDestroyed => return,
                    Event::WindowEvent { window_id, event } => match event {
                        WindowEvent::Resized(physical_size) => self.context.resize(physical_size),
                        WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                        _ => event_handler(Event::WindowEvent { window_id, event }),
                    },
                    Event::RedrawRequested(_) => {}
                    Event::NewEvents(cs) => match cs {
                        glutin::event::StartCause::Poll => {
                            // Perform a render
                            self.render();
                        }
                        _ => event_handler(Event::NewEvents(cs)),
                    },
                    _ => event_handler(event),
                }
            });
        } else {
            panic!("Graphics object was told to run, but the event loop is already consumed!");
        }
    }

    //pub fn message(&mut self, messages: RenderPack<T>) {
    //    self.render_messages = Some(messages);
    //}
}
