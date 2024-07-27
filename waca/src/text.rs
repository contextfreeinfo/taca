use std::sync::Arc;

use glyphon::{Attrs, Buffer, Family, FontSystem, Metrics, Shaping, SwashCache};

pub struct TextEngine {
    pub attrs: Arc<Attrs<'static>>,
    pub buffer: Buffer,
    pub font_system: FontSystem,
    pub swash_cache: SwashCache,
}

impl TextEngine {
    pub fn new() -> Self {
        Self {
            attrs: Arc::new(Attrs::new().family(Family::SansSerif)),
            buffer: Buffer::new_empty(Metrics::new(12.0, 15.0)),
            font_system: FontSystem::new(),
            swash_cache: SwashCache::new(),
        }
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
        for run in buffer.layout_runs() {
            for glyph in run.glyphs {
                let font = font_system.get_font(glyph.font_id);
                let metrics = font.unwrap().as_swash().metrics(&[]);
                dbg!(metrics);
            }
            dbg!(run);
        }
    }
}
