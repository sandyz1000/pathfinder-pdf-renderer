use pathfinder_geometry::{transform2d::Transform2F, vector::Vector2F};
use pathfinder_renderer::gpu::renderer::Renderer;
use pathfinder_webgl::WebGlDevice;
use web_sys::{Event, HtmlCanvasElement, KeyboardEvent, MouseEvent, WheelEvent, Window};
use winit::{event::{ElementState, KeyEvent, Modifiers, RawKeyEvent}, keyboard::{KeyCode, PhysicalKey}};

use crate::{Context, Interactive};


pub struct Emitter<T>(PhantomData<T>);


pub fn virtual_key_code(event: &KeyboardEvent) -> Option<KeyCode> {
    Some(match &event.code()[..] {
        "Digit1" => KeyCode::Digit1,
        "Digit2" => KeyCode::Digit2,
        "Digit3" => KeyCode::Digit3,
        "Digit4" => KeyCode::Digit4,
        "Digit5" => KeyCode::Digit5,
        "Digit6" => KeyCode::Digit6,
        "Digit7" => KeyCode::Digit7,
        "Digit8" => KeyCode::Digit8,
        "Digit9" => KeyCode::Digit9,
        "Digit0" => KeyCode::Digit0,
        "KeyA" => KeyCode::KeyA,
        "KeyB" => KeyCode::KeyB,
        "KeyC" => KeyCode::KeyC,
        "KeyD" => KeyCode::KeyD,
        "KeyE" => KeyCode::KeyE,
        "KeyF" => KeyCode::KeyF,
        "KeyG" => KeyCode::KeyG,
        "KeyH" => KeyCode::KeyH,
        "KeyI" => KeyCode::KeyI,
        "KeyJ" => KeyCode::KeyJ,
        "KeyK" => KeyCode::KeyK,
        "KeyL" => KeyCode::KeyL,
        "KeyM" => KeyCode::KeyM,
        "KeyN" => KeyCode::KeyN,
        "KeyO" => KeyCode::KeyO,
        "KeyP" => KeyCode::KeyP,
        "KeyQ" => KeyCode::KeyQ,
        "KeyR" => KeyCode::KeyR,
        "KeyS" => KeyCode::KeyS,
        "KeyT" => KeyCode::KeyT,
        "KeyU" => KeyCode::KeyU,
        "KeyV" => KeyCode::KeyV,
        "KeyW" => KeyCode::KeyW,
        "KeyX" => KeyCode::KeyX,
        "KeyY" => KeyCode::KeyY,
        "KeyZ" => KeyCode::KeyZ,
        "Escape" => KeyCode::Escape,
        "F1" => KeyCode::F1,
        "F2" => KeyCode::F2,
        "F3" => KeyCode::F3,
        "F4" => KeyCode::F4,
        "F5" => KeyCode::F5,
        "F6" => KeyCode::F6,
        "F7" => KeyCode::F7,
        "F8" => KeyCode::F8,
        "F9" => KeyCode::F9,
        "F10" => KeyCode::F10,
        "F11" => KeyCode::F11,
        "F12" => KeyCode::F12,
        "F13" => KeyCode::F13,
        "F14" => KeyCode::F14,
        "F15" => KeyCode::F15,
        "F16" => KeyCode::F16,
        "F17" => KeyCode::F17,
        "F18" => KeyCode::F18,
        "F19" => KeyCode::F19,
        "F20" => KeyCode::F20,
        "F21" => KeyCode::F21,
        "F22" => KeyCode::F22,
        "F23" => KeyCode::F23,
        "F24" => KeyCode::F24,
        "PrintScreen" => KeyCode::PrintScreen,
        "ScrollLock" => KeyCode::ScrollLock,
        "Pause" => KeyCode::Pause,
        "Insert" => KeyCode::Insert,
        "Home" => KeyCode::Home,
        "Delete" => KeyCode::Delete,
        "End" => KeyCode::End,
        "PageDown" => KeyCode::PageDown,
        "PageUp" => KeyCode::PageUp,
        "ArrowLeft" => KeyCode::ArrowLeft,
        "ArrowUp" => KeyCode::ArrowUp,
        "ArrowRight" => KeyCode::ArrowRight,
        "ArrowDown" => KeyCode::ArrowDown,
        "Backspace" => KeyCode::Backspace,
        "Enter" => KeyCode::Enter,
        "Space" => KeyCode::Space,
        // "Compose" => KeyCode::Compose,
        // "Caret" => KeyCode::Caret,
        "NumLock" => KeyCode::NumLock,
        "Numpad0" => KeyCode::Numpad0,
        "Numpad1" => KeyCode::Numpad1,
        "Numpad2" => KeyCode::Numpad2,
        "Numpad3" => KeyCode::Numpad3,
        "Numpad4" => KeyCode::Numpad4,
        "Numpad5" => KeyCode::Numpad5,
        "Numpad6" => KeyCode::Numpad6,
        "Numpad7" => KeyCode::Numpad7,
        "Numpad8" => KeyCode::Numpad8,
        "Numpad9" => KeyCode::Numpad9,
        // "AbntC1" => KeyCode::AbntC1,
        // "AbntC2" => KeyCode::AbntC2,
        "NumpadAdd" => KeyCode::NumpadAdd,
        "Quote" => KeyCode::Quote,
        // "Apps" => KeyCode::Apps,
        // "At" => KeyCode::At,
        // "Ax" => KeyCode::Ax,
        "Backslash" => KeyCode::Backslash,
        "Calculator" => KeyCode::LaunchApp2,
        "Capital" => KeyCode::CapsLock,
        "Semicolon" => KeyCode::Semicolon,
        "Comma" => KeyCode::Comma,
        "Convert" => KeyCode::Convert,
        "NumpadDecimal" => KeyCode::NumpadDecimal,
        "NumpadDivide" => KeyCode::NumpadDivide,
        "Equal" => KeyCode::Equal,
        "Backquote" => KeyCode::Backquote,
        "Kana" => KeyCode::KanaMode,
        // "Kanji" => NamedKey::KanjiMode,
        "AltLeft" => KeyCode::AltLeft,
        "BracketLeft" => KeyCode::BracketLeft,
        "ControlLeft" => KeyCode::ControlLeft,
        "ShiftLeft" => KeyCode::ShiftLeft,
        "MetaLeft" => KeyCode::SuperLeft,
        "Mail" => KeyCode::LaunchMail,
        "MediaSelect" => KeyCode::MediaSelect,
        "MediaStop" => KeyCode::MediaStop,
        "Minus" => KeyCode::Minus,
        "NumpadMultiply" => KeyCode::NumpadMultiply,
        "Mute" => KeyCode::AudioVolumeMute,
        "LaunchMyComputer" => KeyCode::LaunchApp1,
        "NavigateForward" => KeyCode::BrowserForward,
        "NavigateBackward" => KeyCode::BrowserBack,
        "NextTrack" => KeyCode::MediaTrackNext,
        // "NoConvert" => KeyCode::NoConvert,
        "NumpadComma" => KeyCode::NumpadComma,
        "NumpadEnter" => KeyCode::NumpadEnter,
        "NumpadEquals" => KeyCode::NumpadEqual,
        // "OEM102" => KeyCode::OEM102,
        "Period" => KeyCode::Period,
        "PlayPause" => KeyCode::MediaPlayPause,
        "Power" => KeyCode::Power,
        "PrevTrack" => KeyCode::MediaTrackPrevious,
        "AltRight" => KeyCode::AltRight,
        "BracketRight" => KeyCode::BracketRight,
        "ControlRight" => KeyCode::ControlRight,
        "ShiftRight" => KeyCode::ShiftRight,
        "MetaRight" => KeyCode::SuperRight,
        "Slash" => KeyCode::Slash,
        "Sleep" => KeyCode::Sleep,
        "Stop" => KeyCode::MediaStop,
        "NumpadSubtract" => KeyCode::NumpadSubtract,
        // "Sysrq" => KeyCode::Sysrq,
        "Tab" => KeyCode::Tab,
        // "Underline" => KeyCode::Underline,
        // "Unlabeled" => KeyCode::Unlabeled,
        "AudioVolumeDown" => KeyCode::AudioVolumeDown,
        "AudioVolumeUp" => KeyCode::AudioVolumeUp,
        "Wake" => KeyCode::WakeUp,
        "WebBack" => KeyCode::BrowserBack,
        "WebFavorites" => KeyCode::BrowserFavorites,
        "WebForward" => KeyCode::BrowserForward,
        "WebHome" => KeyCode::BrowserHome,
        "WebRefresh" => KeyCode::BrowserRefresh,
        "WebSearch" => KeyCode::BrowserSearch,
        "WebStop" => KeyCode::BrowserStop,
        // "Yen" => KeyCode::Yen,
        _ => return None,
    })
}

pub struct Backend;

impl Backend {
    pub fn resize(&mut self, size: Vector2F) {}
    pub fn get_scroll_factors(&self) -> (Vector2F, Vector2F) {
        (Vector2F::new(1.0, 1.0), Vector2F::new(10.0, -10.0))
    }
    pub fn set_icon(&mut self, icon: Icon) {}
}

#[wasm_bindgen]
pub struct WasmView {
    item: Box<dyn Interactive<Event = Vec<u8>>>,
    ctx: Context,
    window: Window,
    renderer: Renderer<WebGlDevice>,
    framebuffer_size: Vector2F,
    canvas: HtmlCanvasElement,
}

impl WasmView {
    pub fn new(
        canvas: HtmlCanvasElement,
        context: WebGl2RenderingContext,
        config: Config,
        mut item: Box<dyn Interactive<Event = Vec<u8>>>,
    ) -> Self {
        canvas.set_attribute("tabindex", "0").unwrap();
        canvas.set_attribute("contenteditable", "true").unwrap();

        let window = web_sys::window().unwrap();
        let scale_factor = scale_factor(&window);
        let backend = Backend {};
        let mut ctx = Context::new(config, backend);
        ctx.set_scale_factor(scale_factor);

        // figure out the framebuffer, as that can only be integer values
        let framebuffer_size = v_ceil(item.window_size_hint().unwrap_or(vec2f(100., 100.)));

        // then figure out the css size
        ctx.window_size = framebuffer_size * (1.0 / ctx.scale_factor);

        set_canvas_size(&canvas, ctx.window_size, framebuffer_size.to_i32());

        let render_mode = RendererMode {
            level: ctx.config.render_level,
        };
        let render_options = RendererOptions {
            dest: DestFramebuffer::full_window(framebuffer_size.to_i32()),
            background_color: Some(ctx.config.background),
            show_debug_ui: false,
        };

        let renderer = Renderer::new(
            WebGlDevice::new(context),
            &*ctx.config.resource_loader,
            render_mode,
            render_options,
        );

        item.init(&mut ctx, Emitter(PhantomData));

        WasmView {
            item,
            ctx,
            window,
            renderer,
            canvas,
            framebuffer_size,
        }
    }
}

fn v_ceil(v: Vector2F) -> Vector2F {
    Vector2F::new(v.x().ceil(), v.y().ceil())
}

#[wasm_bindgen]
impl WasmView {
    pub fn render(&mut self) {
        let mut scene = self.item.scene(&mut self.ctx);
        let scene_view_box = view_box(&scene);

        // figure out the framebuffer, as that can only be integer values
        let framebuffer_size = v_ceil(scene_view_box.size());

        // then figure out the css size
        self.ctx.window_size = framebuffer_size * (1.0 / self.ctx.scale_factor);

        if framebuffer_size != self.framebuffer_size {
            set_canvas_size(
                &self.canvas,
                self.ctx.window_size,
                framebuffer_size.to_i32(),
            );
            self.renderer.options_mut().dest =
                DestFramebuffer::full_window(framebuffer_size.to_i32());
            self.framebuffer_size = framebuffer_size;
        }

        // temp fix
        scene.set_view_box(RectF::new(
            Vector2F::default(),
            round_v_to_16(framebuffer_size.to_i32()).to_f32(),
        ));

        let tr = if self.ctx.config.pan {
            Transform2F::from_translation(self.ctx.window_size * 0.5)
                * Transform2F::from_translation(-self.ctx.view_center)
        } else {
            Transform2F::from_translation(-scene_view_box.origin())
        };
        let options = BuildOptions {
            transform: RenderTransform::Transform2D(tr),
            dilation: Vector2F::default(),
            subpixel_aa_enabled: false,
        };

        scene.build_and_render(&mut self.renderer, options, SequentialExecutor);
        self.ctx.redraw_requested = false;
    }

    pub fn animation_frame(&mut self, timestamp: f64) {
        self.render();
    }

    pub fn mouse_move(&mut self, event: &MouseEvent) -> bool {
        false
    }

    pub fn mouse_down(&mut self, event: &MouseEvent) -> bool {
        self.mouse_input(event, ElementState::Pressed);
        self.ctx.redraw_requested
    }

    pub fn mouse_up(&mut self, event: &MouseEvent) -> bool {
        self.mouse_input(event, ElementState::Released);
        self.ctx.redraw_requested
    }

    fn mouse_input(&mut self, event: &MouseEvent, state: ElementState) {
        let css_pos = Vector2F::new(event.offset_x() as f32, event.offset_y() as f32);

        let scale = 1.0 / self.ctx.scale;
        let tr = if self.ctx.config.pan {
            Transform2F::from_translation(self.ctx.view_center)
                * Transform2F::from_scale(Vector2F::splat(scale))
                * Transform2F::from_translation(
                    self.ctx.window_size * (-0.5 * self.ctx.scale_factor),
                )
        } else {
            Transform2F::from_scale(Vector2F::splat(scale))
        };

        let scene_pos = tr * css_pos;
        let page = self.ctx.page_nr;
        self.item.mouse_input(&mut self.ctx, page, scene_pos, state);
    }

    pub fn wheel(&mut self, event: &WheelEvent) -> bool {
        self.ctx.redraw_requested
    }

    pub fn key_down(&mut self, event: &KeyboardEvent) -> bool {
        self.keyboard_input(event, ElementState::Pressed);
        self.ctx.redraw_requested
    }

    pub fn key_up(&mut self, event: &KeyboardEvent) -> bool {
        self.keyboard_input(event, ElementState::Released);
        self.ctx.redraw_requested
    }

    fn keyboard_input(&mut self, event: &KeyboardEvent, state: ElementState) {
        let keycode = match virtual_key_code(&event) {
            Some(keycode) => keycode,
            None => return,
        };
        let rkevt = RawKeyEvent {
            physical_key: PhysicalKey::Code(keycode),
            state,
        };

        let key_event: KeyEvent = rkevt.into();
        self.item.keyboard_input(&mut self.ctx, state, key_event.clone());

        if key_event.cancelled {
            cancel(&event);
        }
    }

    pub fn resize(&mut self, event: &UiEvent) -> bool {
        self.ctx.set_scale_factor(scale_factor(&self.window));
        self.ctx.request_redraw();
        self.ctx.redraw_requested
    }

    pub fn data(&mut self, data: &Uint8Array) -> bool {
        self.item.event(&mut self.ctx, data.to_vec());
        self.ctx.redraw_requested
    }

    pub fn idle(&mut self) -> bool {
        self.item.idle(&mut self.ctx);
        self.ctx.redraw_requested
    }

    pub fn input(&mut self, text: String) -> bool {
        self.item.text_input(&mut self.ctx, text);
        self.ctx.redraw_requested
    }
}

fn cancel(event: impl AsRef<Event>) {
    event.as_ref().prevent_default();
}

fn set_canvas_size(canvas: &HtmlCanvasElement, css_size: Vector2F, framebuffer_size: Vector2I) {
    canvas.set_width(framebuffer_size.x() as u32);
    canvas.set_height(framebuffer_size.y() as u32);

    let style = canvas.style();
    style
        .set_property("width", &format!("{}px", css_size.x()))
        .expect("Failed to set canvas width");
    style
        .set_property("height", &format!("{}px", css_size.y()))
        .expect("Failed to set canvas height");
}

pub fn scale_factor(window: &Window) -> f32 {
    window.device_pixel_ratio() as f32
}

pub fn window_size(window: &Window) -> Vector2F {
    let width = window.inner_width().unwrap().as_f64().unwrap();

    let height = window.inner_height().unwrap().as_f64().unwrap();

    Vector2F::new(width as f32, height as f32)
}

pub fn mouse_modifiers(event: &MouseEvent) -> Modifiers {
    Modifiers::default()
}

pub fn keyboard_modifiers(event: &KeyboardEvent) -> Modifiers {
    // shift: event.shift_key(),
    // ctrl: event.ctrl_key(),
    // alt: event.alt_key(),
    // meta: event.meta_key(),
    Modifiers::default()
}