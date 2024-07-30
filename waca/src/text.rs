use std::sync::Arc;

use glyphon::{
    fontdb::ID, Attrs, Buffer, Cache, Color, Family, FontSystem, LayoutRun, Metrics, Resolution,
    Shaping, SwashCache, TextArea, TextAtlas, TextBounds, TextRenderer, Viewport,
};
use wgpu::{MultisampleState, TextureFormat};

use crate::{
    app::System,
    display::{Graphics, MaybeGraphics},
    gpu::RenderFrame,
};

pub struct TextEngine {
    pub atlas: TextAtlas,
    pub attrs: Arc<Attrs<'static>>,
    pub buffer: Buffer,
    pub font_system: FontSystem,
    pub swash_cache: SwashCache,
    pub text_renderer: TextRenderer,
    pub viewport: Viewport,
}

impl TextEngine {
    pub fn new(gfx: &Graphics) -> Self {
        let cache = Cache::new(&gfx.device);
        let mut atlas = TextAtlas::new(&gfx.device, &gfx.queue, &cache, TextureFormat::Bgra8Unorm);
        let text_renderer =
            TextRenderer::new(&mut atlas, &gfx.device, MultisampleState::default(), None);
        let viewport = Viewport::new(&gfx.device, &cache);
        Self {
            atlas,
            attrs: Arc::new(Attrs::new().family(Family::SansSerif)),
            buffer: Buffer::new_empty(Metrics::new(30.0, 40.0)),
            font_system: FontSystem::new(),
            swash_cache: SwashCache::new(),
            text_renderer,
            viewport,
        }
    }

    pub fn draw(&mut self, system: &mut System, text: &str, x: f32, y: f32) {
        // Pretend we're static since we actually do outlive the pass.
        let static_self: &'static mut Self = unsafe { &mut *(self as *mut _) };
        let Self {
            ref mut atlas,
            attrs,
            buffer,
            ref mut font_system,
            ref mut swash_cache,
            text_renderer,
            viewport,
        } = static_self;
        let Some(RenderFrame {
            pass: Some(ref mut pass),
            ..
        }) = system.frame
        else {
            panic!()
        };
        let MaybeGraphics::Graphics(Graphics {
            ref device,
            ref queue,
            ref window,
            ..
        }) = system.display.graphics
        else {
            panic!()
        };
        let size = window.inner_size();
        buffer.set_text(font_system, text, **attrs, Shaping::Advanced);
        buffer.shape_until_scroll(font_system, false);
        viewport.update(
            &queue,
            Resolution {
                width: size.width,
                height: size.height,
            },
        );
        text_renderer
            .prepare(
                &device,
                &queue,
                font_system,
                atlas,
                &viewport,
                [TextArea {
                    buffer: &buffer,
                    left: x,
                    top: y,
                    scale: 1.0,
                    bounds: TextBounds {
                        left: 0,
                        top: 0,
                        right: size.width as i32,
                        bottom: size.height as i32,
                    },
                    default_color: Color::rgb(255, 255, 255),
                }],
                swash_cache,
            )
            .unwrap();
        text_renderer.render(atlas, viewport, pass).unwrap();
    }

    pub fn measure_text(&mut self, text: &str) {
        let Self {
            attrs,
            buffer,
            font_system,
            ..
        } = self;
        // let fonts = font_system.get_font_matches(attrs);
        // let default_families = [&attrs.family];
        buffer.set_text(font_system, text, **attrs, Shaping::Advanced);
        let mut font_id = ID::dummy();
        for run in buffer.layout_runs() {
            for glyph in run.glyphs {
                if glyph.font_id != font_id {
                    font_id = glyph.font_id;
                    let font = font_system.get_font(glyph.font_id).unwrap();
                    let metrics = font.as_swash().metrics(&[]);
                    dbg!(glyph, metrics);
                }
            }
            dbg!(LayoutRun { glyphs: &[], ..run });
        }
    }
}
