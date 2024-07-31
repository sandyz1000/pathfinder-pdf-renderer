mod wasm;

use log::*;
use std::sync::Arc;
use pathfinder_geometry::vector::Vector2F;
use pathfinder_renderer::scene::Scene;
use js_sys::Uint8Array;
use pdf::any::AnySync;
use pdf::backend::Backend;
use pdf::file::{Cache as PdfCache, File as PdfFile, Log};
use pdf::PdfError;
use pdf_render::{page_bounds, render_page, Cache, SceneBackend};
use winit::{
    event::{ElementState, KeyEvent, Modifiers},
    keyboard::{KeyCode, ModifiersState, PhysicalKey},
};

use pdf_view::{Context, view::Interactive, show::Emitter};

pub struct PdfView<B: Backend, OC, SC, L> {
    file: PdfFile<B, OC, SC, L>,
    num_pages: usize,
    cache: Cache,
}

impl<B, OC, SC, L> PdfView<B, OC, SC, L>
where
    B: Backend + 'static,
    OC: PdfCache<Result<AnySync, Arc<PdfError>>> + 'static,
    SC: PdfCache<Result<Arc<[u8]>, Arc<PdfError>>> + 'static,
    L: Log,
{
    pub fn new(file: PdfFile<B, OC, SC, L>) -> Self {
        PdfView {
            num_pages: file.num_pages() as usize,
            file,
            cache: Cache::new(),
        }
    }
}

impl<B, OC, SC, L> Interactive for PdfView<B, OC, SC, L>
where
    B: Backend + 'static,
    OC: PdfCache<Result<AnySync, Arc<PdfError>>> + 'static,
    SC: PdfCache<Result<Arc<[u8]>, Arc<PdfError>>> + 'static,
    L: Log + 'static,
{
    type Event = Vec<u8>;
    fn title(&self) -> String {
        self.file
            .trailer
            .info_dict
            .as_ref()
            .and_then(|info| info.title.as_ref())
            .and_then(|p| p.to_string().ok())
            .unwrap_or_else(|| "PDF View".into())
    }
    fn init(&mut self, ctx: &mut Context, sender: Emitter<Self::Event>) {
        ctx.num_pages = self.num_pages;
        ctx.set_icon(
            image::load_from_memory_with_format(
                include_bytes!("../../logo.png"),
                image::ImageFormat::Png,
            )
            .unwrap()
            .to_rgba8()
            .into(),
        );
    }
    fn scene(&mut self, ctx: &mut Context) -> Scene {
        info!("drawing page {}", ctx.page_nr());
        let page = self.file.get_page(ctx.page_nr as u32).unwrap();

        ctx.set_bounds(page_bounds(&page));

        let mut backend = SceneBackend::new(&mut self.cache);
        let resolver = self.file.resolver();
        render_page(&mut backend, &resolver, &page, ctx.view_transform()).unwrap();
        backend.finish()
    }
    fn mouse_input(&mut self, ctx: &mut Context, page: usize, pos: Vector2F, state: ElementState) {
        if state != ElementState::Pressed {
            return;
        }
        info!("x={}, y={}", pos.x(), pos.y());
    }
    fn keyboard_input(&mut self, ctx: &mut Context, state: ModifiersState, event: KeyEvent) {
        if event.state == ElementState::Released {
            return;
        }
        if state.shift_key() {
            let page = ctx.page_nr();
            match event.physical_key {
                PhysicalKey::Code(KeyCode::ArrowRight) => ctx.goto_page(page + 10),
                PhysicalKey::Code(KeyCode::ArrowLeft) => ctx.goto_page(page.saturating_sub(10)),
                _ => return,
            }
        }
        match event.physical_key {
            PhysicalKey::Code(KeyCode::ArrowRight | KeyCode::PageDown) => ctx.next_page(),
            PhysicalKey::Code(KeyCode::ArrowLeft | KeyCode::PageUp) => ctx.prev_page(),
            _ => return,
        }
    }
}
