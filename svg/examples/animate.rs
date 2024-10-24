use view::{show, Config, Interactive, Context, Emitter};
use pathfinder_renderer::scene::Scene;
use pathfinder_resources::embedded::EmbeddedResourceLoader;
use svg_dom::{Svg, Time};
use svg_draw::{DrawOptions, DrawContext};
use std::time::Instant;
use svg_text::{Font, FontCollection};
use std::sync::Arc;

struct AnimatedSvg {
    svg: Svg,
    fonts: FontCollection,
    start: Instant
}

impl Interactive for AnimatedSvg {
    fn init(&mut self, ctx: &mut Context, sender: Emitter<Self::Event>) {
        self.start = Instant::now();
        ctx.update_interval = Some(0.02);
        ctx.num_pages = 1;
    }
    fn idle(&mut self, ctx: &mut Context) {
        ctx.request_redraw();
    }
    fn scene(&mut self, ctx: &mut Context) -> Scene {
        let ctx = DrawContext::new(&self.svg, &self.fonts);
        let mut options = DrawOptions::new(&ctx);
        options.time = Time::from_seconds(self.start.elapsed().as_secs_f64());
        ctx.compose_with_options(&options)
    }
}

fn main() {
    env_logger::init();
    let input = std::env::args().nth(1).unwrap();
    let data = std::fs::read(input).unwrap();
    let mut config = Config::new(Box::new(EmbeddedResourceLoader));
    config.zoom = true;
    config.pan = false;

    let fonts = FontCollection::from_fonts(vec![
        Font::load(include_bytes!("../../resources/latinmodern-math.otf")),
        Font::load(include_bytes!("../../resources/NotoNaskhArabic-Regular.ttf")),
        Font::load(include_bytes!("../../resources/NotoSerifBengali-Regular.ttf")),
    ]);

    let svg = Svg::from_data(&data).unwrap();
    show(AnimatedSvg {
        svg,
        fonts,
        start: Instant::now()
    }, config)
}
