use crate::channels::{PackingToWindowReceiver, WindowToPackingSender};
use glutin::event::{Event, WindowEvent};
use glutin::event_loop::ControlFlow;
use graphics::{ExternalEvent, GraphicsCapabilities, Gui, RenderCaller};
use log::info;

use crate::channels::*;

pub fn start_window(
    rx: PackingToWindowReceiver,
    tx: WindowToLogicSender,
    packing_tx: WindowToPackingSender,
) {
    let window = unsafe { Window::new(rx, packing_tx) };

    let eh: EventHandler = Box::new(move |event| {
        if let Some(event) = ExternalEvent::create_from_glut_event(event) {
            tx.channel_sender.send(event).unwrap();
        }
    });

    unsafe { window.run(eh) };
}

pub struct Window {
    event_loop: Option<glutin::event_loop::EventLoop<()>>,
    context: glutin::ContextWrapper<glutin::PossiblyCurrent, glutin::window::Window>,
    render_caller: RenderCaller,
    render_messages: PackingToWindowReceiver,
    capabilities_sender: WindowToPackingSender,
    gui: Gui,
}

pub type EventHandler = Box<dyn FnMut(glutin::event::Event<()>) + Send + 'static>;

impl Window {
    ///
    /// unsafe, since calling twice on the same thread is likely to lead to serious trouble.
    /// Also, extremely stateful.
    pub unsafe fn new(rx: PackingToWindowReceiver, packing_tx: WindowToPackingSender) -> Window {
        let el = glutin::event_loop::EventLoop::new();
        let wb = glutin::window::WindowBuilder::new()
            .with_title("Hello world!")
            .with_inner_size(glutin::dpi::LogicalSize::new(1024.0, 768.0));
        let windowed_context = glutin::ContextBuilder::new()
            .build_windowed(wb, &el)
            .unwrap();

        let windowed_context = windowed_context.make_current().unwrap();

        let screen_dimensions = (
            windowed_context.window().inner_size().width,
            windowed_context.window().inner_size().height,
        );
        info!("Loading gl!");
        graphics::RenderCaller::initialize_gl(&windowed_context);
        let render_caller = RenderCaller::new(screen_dimensions);

        let res = Window {
            event_loop: Some(el),
            context: windowed_context,
            render_caller,
            render_messages: rx,
            capabilities_sender: packing_tx,
            gui: Gui::new((400.0, 400.0), (-1.0, -1.0)),
        };
        res.send_capabilities();

        res
    }

    fn get_capabilities(&self) -> GraphicsCapabilities {
        GraphicsCapabilities {
            vbo_count: self.render_caller.get_vbo_count(),
            texture_metadata: self
                .render_caller
                .get_texture_manager()
                .get_texture_metadata(),
            shader_metadata: self
                .render_caller
                .get_shader_manager()
                .get_shader_metadata(),
            framebuffer_metadata: self
                .render_caller
                .get_framebuffer_manager()
                .get_framebuffer_metadata(),
            screen_dimensions: (
                self.context.window().inner_size().width,
                self.context.window().inner_size().height,
            ),
        }
    }

    fn send_capabilities(&self) {
        self.capabilities_sender
            .channel_sender
            .send(self.get_capabilities())
            .unwrap();
    }

    unsafe fn render(&mut self) {
        // Try getting the lock; only render if there's render messages available.
        let render_messages = self.render_messages.render_pack.try_lock();
        self.gui.reset_gui();
        self.gui.add_text(
            "the quick brown fox jumped over the lazy dog",
            (100.0, 100.0),
            200.0,
            200.0,
            16.0,
        );

        if let Ok(mut render_messages) = render_messages {
            if let Some(messages) = render_messages.take() {
                for message in messages.iter() {
                    self.render_caller.read_message(message);
                }

                // TODO: VALIDATE GUI RENDER MESSAGES
                let messages = self.gui.collect_render_messages();
                for message in messages.iter() {
                    self.render_caller.read_message(message);
                }

                self.context.swap_buffers().unwrap();
            }
        }
    }

    unsafe fn update_screen_dimensions(&mut self, screen_dimensions: (u32, u32)) {
        //gl::Viewport(0, 0, screen_dimensions.0 as i32, screen_dimensions.1 as i32);
        self.render_caller
            .update_screen_dimensions(screen_dimensions);
        self.send_capabilities();
    }

    ///
    /// Starts this graphics object
    ///
    /// NOTE: THE EXECUTION GOES TO THE GRAPHICS OBJECT WHEN THIS IS CALLED!
    ///
    pub unsafe fn run(mut self, mut event_handler: EventHandler) {
        //Ignore the result from the function.
        let _ = self.context.window().set_cursor_grab(true);
        self.context.window().set_cursor_visible(false);
        if let Some(el) = self.event_loop.take() {
            el.run(move |event, _, control_flow| {
                *control_flow = ControlFlow::Poll;

                match event {
                    Event::LoopDestroyed => {} //TODO: HANDLE LOOP DESTROYED
                    Event::WindowEvent { window_id, event } => match event {
                        WindowEvent::Resized(physical_size) => {
                            self.context.resize(physical_size);
                            self.update_screen_dimensions((
                                physical_size.width,
                                physical_size.height,
                            ));
                        }
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
