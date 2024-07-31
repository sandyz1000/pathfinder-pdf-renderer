
use pathfinder_geometry::vector::{vec2f, Vector2F};
use log::*;
use pathfinder_renderer::options::{BuildOptions, RenderTransform};
use winit::application::ApplicationHandler;
use winit::window::Window;
use std::time::{Duration, Instant};
use winit::dpi::{PhysicalPosition, PhysicalSize};
use winit::event::{
    ElementState as WinitElementState, Event, InnerSizeWriter, MouseButton, MouseScrollDelta, StartCause, WindowEvent
};
use winit::event_loop::{ControlFlow, EventLoopProxy};
use winit::keyboard::ModifiersState;
use crate::gl::GlWindow;
use crate::view::Interactive;
use crate::{Context, Icon, Config};


pub struct Emitter<E: 'static>(EventLoopProxy<E>);

impl<E: 'static> Emitter<E> {
    pub fn send(&self, event: E) {
        let _ = self.0.send_event(event);
    }
}

impl<E: 'static> Clone for Emitter<E> {
    fn clone(&self) -> Self {
        Emitter(self.0.clone())
    }
}

pub struct Backend {
    window: GlWindow,
}

impl Backend {
    pub fn new(window: GlWindow) -> Backend {
        Backend { window }
    }
    pub fn resize(&mut self, size: Vector2F) {
        self.window.resize(size);
    }
    pub fn get_scroll_factors(&self) -> (Vector2F, Vector2F) {
        (
            env_vec("PIXEL_SCROLL_FACTOR").unwrap_or(Vector2F::new(1.0, 1.0)),
            env_vec("LINE_SCROLL_FACTOR").unwrap_or(Vector2F::new(10.0, -10.0)),
        )
    }
    pub fn set_icon(&mut self, icon: Icon) {
        self.window.window().set_window_icon(Some(
            winit::window::Icon::from_rgba(icon.data, icon.width, icon.height).unwrap(),
        ));
    }
}

fn env_vec(name: &str) -> Option<Vector2F> {
    use tuple::{Map, TupleElements, T2};
    let val = std::env::var(name).ok()?;
    let t2 = T2::from_iter(val.splitn(2, ","))?;
    let T2(x, y) = t2.map(|s: &str| s.parse().ok()).collect()?;
    Some(Vector2F::new(x, y))
}

pub fn show<T: Interactive>(mut item: T, config: Config) {
    use winit::event_loop::{EventLoopBuilder, EventLoop};

    log::info!("creating event loop");
    let mut event_loop = EventLoop::with_user_event().build().unwrap();

    let mut cursor_pos = Vector2F::default();
    let mut dragging = false;

    let window_size = item.window_size_hint().unwrap_or(vec2f(600., 400.));
    let glwindow = GlWindow::new(&event_loop, item.title(), window_size, &config);
    let window = glwindow.window().clone();
    let backend = Backend::new(glwindow);
    let mut ctx = Context::new(config, backend);
    let scale_factor = ctx.backend.window.scale_factor();
    ctx.set_scale_factor(scale_factor);
    ctx.request_redraw();
    ctx.window_size = window_size;

    let proxy = event_loop.create_proxy();

    item.init(&mut ctx, Emitter(proxy));

    let mut modifiers = ModifiersState::default();
    info!("entering the event loop");

    // ControlFlow::Wait pauses the event loop if no events are available to process.
    // This is ideal for non-game applications that only update in response to user
    // input, and uses significantly less power/CPU time than ControlFlow::Poll.
    event_loop.set_control_flow(ControlFlow::Wait);

    // let mut app = App::new(window, ctx, item, modifiers, cursor_pos, dragging);
    // event_loop.run_app(&mut app);
}



struct App<I> {
    window: Option<Window>,
    ctx: Context,
    item: I,
    modifiers: ModifiersState,
    cursor_pos: Vector2F, 
    dragging: bool
}

impl<I: Interactive> App<I> {
    fn new(window: Window, ctx: Context, item: I, modifiers: ModifiersState, cursor_pos: Vector2F, dragging: bool) -> Self {
        App {
            window: Some(window),
            ctx, 
            item, 
            modifiers,
            cursor_pos,
            dragging,
        }
    }
}

impl<I: Interactive> ApplicationHandler for App<I> {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        self.window = Some(event_loop.create_window(Window::default_attributes()).unwrap());
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        match event {
            // Event::NewEvents(StartCause::Init) => {},
            // Event::NewEvents(StartCause::ResumeTimeReached {
            //     start: _,
            //     requested_resume: _,
            // }) => {
            //     ctx.request_redraw();
            // },
            WindowEvent::RedrawRequested => {
                let options = BuildOptions {
                    transform: RenderTransform::default(),
                    dilation: Vector2F::default(),
                    subpixel_aa_enabled: false,
                };

                self.ctx.backend.window.resized(self.ctx.window_size);
                let scene = self.item.scene(&mut self.ctx);
                self.ctx.backend.window.render(scene, options);
                self.ctx.redraw_requested = false;
            },
            WindowEvent::ScaleFactorChanged {
                scale_factor,
                mut inner_size_writer,
            } => {
                
                self.ctx.set_scale_factor(scale_factor as f32);
                // self.ctx.set_window_size(Vector2F::new(*width as f32, *height as f32));
                let width = self.ctx.window_size.x().ceil() as u32;
                let height = self.ctx.window_size.y().ceil() as u32;
                let new_inner_size = PhysicalSize::new(width, height);
                inner_size_writer.request_inner_size(new_inner_size);
                self.ctx.request_redraw();
            }
            
            WindowEvent::Focused { .. } => self.ctx.request_redraw(),
            
            WindowEvent::Resized(PhysicalSize { width, height }) => {
                let physical_size = Vector2F::new(width as f32, height as f32);
                self.ctx.window_size = physical_size;
                self.ctx.check_bounds();
                self.ctx.request_redraw();
            }
            
            WindowEvent::ModifiersChanged(new_modifiers) => {
                self.modifiers = new_modifiers.state();
            }
            
            WindowEvent::KeyboardInput { event, .. } => {
                self.item.keyboard_input(&mut self.ctx, self.modifiers, event);
            }
            
            WindowEvent::CursorMoved {
                position: PhysicalPosition { x, y },
                ..
            } => {
                let new_pos = Vector2F::new(x as f32, y as f32);
                let cursor_delta = new_pos - self.cursor_pos;
                self.cursor_pos = new_pos;

                if self.dragging {
                    self.ctx.move_by(cursor_delta * (-1.0 / self.ctx.scale));
                } else {
                    self.item.cursor_moved(&mut self.ctx, new_pos);
                }
            }
            
            WindowEvent::MouseInput {
                button: MouseButton::Left,
                state,
                ..
            } => match (state, self.modifiers.shift_key()) {
                (WinitElementState::Pressed, true) if self.ctx.config.pan => self.dragging = true,
                (WinitElementState::Released, _) if self.dragging => self.dragging = false,
                _ => {
                    let page_nr = self.ctx.page_nr;
                    self.item.mouse_input(&mut self.ctx, page_nr, self.cursor_pos, state);
                }
            },
            
            WindowEvent::MouseWheel { delta, .. } => {
                let delta = match delta {
                    MouseScrollDelta::PixelDelta(PhysicalPosition { x: dx, y: dy }) => {
                        Vector2F::new(dx as f32, dy as f32) * self.ctx.pixel_scroll_factor
                    }
                    MouseScrollDelta::LineDelta(dx, dy) => {
                        Vector2F::new(dx as f32, dy as f32) * self.ctx.line_scroll_factor
                    }
                };
                if self.ctx.config.zoom && self.modifiers.control_key() {
                    self.ctx.zoom_by(-0.02 * delta.y());
                } else if self.ctx.config.pan {
                    self.ctx.move_by(delta * (-1.0 / self.ctx.scale));
                }
            }

            WindowEvent::CloseRequested => {
                println!("The close button was pressed; stopping");
                self.ctx.close();
            },
            
            // Event::UserEvent(e) => {
            //     self.item.event(&mut self.ctx, e);
            // }

            // Event::MainEventsCleared => item.idle(&mut ctx),

            
            // Event::LoopDestroyed => {
            //     item.exit(&mut ctx);
            // }
            _ => {}
        }
    }
}
