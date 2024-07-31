use pathfinder_renderer::gpu::options::RendererLevel;

use pathfinder_resources::embedded::EmbeddedResourceLoader;
use pathfinder_color::ColorF;
use pdf::file::FileOptions;
use pdf_view::show::show;
use pdf_view::{Config, PdfView};
use crate::viewer::{Interactive, Config, Context};
use crate::Icon;

fn main() {
    env_logger::init();
    let path = std::env::args().nth(1).unwrap();
    let file = FileOptions::uncached().open(&path).unwrap();
    let view = PdfView::new(file);
    let mut config = Config::new(Box::new(EmbeddedResourceLoader));
    config.zoom = true;
    config.pan = true;
    config.background = ColorF::new(0.9, 0.9, 0.9, 1.0);
    config.render_level = RendererLevel::D3D9;
    show(view, config);
}
