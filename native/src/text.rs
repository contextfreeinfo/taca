use std::sync::Arc;

use glyphon::{
    fontdb::ID, Attrs, Buffer, Cache, Color, Family, FontSystem, Metrics, Resolution, Shaping,
    SwashCache, TextArea, TextAtlas, TextBounds, TextRenderer, Viewport,
};
use wgpu::{MultisampleState, TextureFormat};

use crate::{
    app::System,
    display::{Graphics, MaybeGraphics},
    gpu::RenderFrame,
};

pub struct Font {
    pub color: Color,
    pub name: String,
    pub size: f32,
}

#[derive(Clone, Copy, Debug)]
pub enum TextAlignX {
    Left,
    Center,
    Right,
}

#[derive(Clone, Copy, Debug)]
pub enum TextAlignY {
    Baseline,
    Top,
    Middle,
    Bottom,
}

pub struct TextEngine {
    pub align_x: TextAlignX,
    pub align_y: TextAlignY,
    pub atlas: TextAtlas,
    pub attrs: Arc<Attrs<'static>>,
    pub buffer: Buffer,
    pub font: Font,
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
            align_x: TextAlignX::Left,
            align_y: TextAlignY::Baseline,
            atlas,
            attrs: Arc::new(Attrs::new().family(Family::SansSerif)),
            buffer: Buffer::new_empty(Metrics::new(30.0, 40.0)),
            font: Font {
                color: Color::rgb(255, 255, 255),
                name: "sans-serif".into(),
                size: 30.0,
            },
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
            align_x,
            align_y,
            ref mut atlas,
            attrs,
            buffer,
            ref font,
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
        // Metrics
        let metrics_text = match text.len() {
            0 => DEFAULT_TEXT,
            _ => text,
        };
        buffer.set_text(font_system, metrics_text, **attrs, Shaping::Advanced);
        let text_size = adjust_metrics(buffer, font, font_system);
        // Text
        buffer.set_text(font_system, text, **attrs, Shaping::Advanced);
        buffer.shape_until_scroll(font_system, false);
        // Render
        let size = window.inner_size();
        viewport.update(
            &queue,
            Resolution {
                width: size.width,
                height: size.height,
            },
        );
        // dbg!(&align_x, &align_y);
        text_renderer
            .prepare(
                &device,
                &queue,
                font_system,
                atlas,
                &viewport,
                [TextArea {
                    buffer: &buffer,
                    left: x - text_size.0
                        * match align_x {
                            TextAlignX::Left => 0.0,
                            TextAlignX::Center => 0.5,
                            TextAlignX::Right => 1.0,
                        },
                    top: y - text_size.1
                        * match align_y {
                            TextAlignY::Top => 0.0,
                            TextAlignY::Middle => 0.5,
                            // TODO Baseline.
                            TextAlignY::Baseline | TextAlignY::Bottom => 1.0,
                        },
                    scale: 1.0,
                    bounds: TextBounds {
                        left: 0,
                        top: 0,
                        right: size.width as i32,
                        bottom: size.height as i32,
                    },
                    default_color: font.color,
                }],
                swash_cache,
            )
            .unwrap();
        text_renderer.render(atlas, viewport, pass).unwrap();
    }
}

fn adjust_metrics(buffer: &mut Buffer, font: &Font, font_system: &mut FontSystem) -> (f32, f32) {
    // The line height doesn't really matter, but pick something reasonable anyway.
    buffer.set_metrics(font_system, Metrics::new(font.size, font.size * 1.5));
    let mut max_height = 0f32;
    let mut font_id = ID::dummy();
    let mut width = 0f32;
    for run in buffer.layout_runs() {
        for glyph in run.glyphs {
            // TODO Always just use the first???
            if glyph.font_id != font_id {
                font_id = glyph.font_id;
                let font = font_system.get_font(glyph.font_id).unwrap();
                let metrics = font.as_swash().metrics(&[]);
                let height = (metrics.ascent + metrics.descent) / metrics.units_per_em as f32;
                max_height = max_height.max(height);
            }
        }
        // dbg!(LayoutRun { glyphs: &[], ..run });
        width += run.line_w;
    }
    let height = font.size * max_height;
    return (width, height);
    // buffer.set_metrics(font_system, Metrics::new(font.size / max_height, font_size * 1.5));
}

pub fn to_text_align_x(x: u32) -> TextAlignX {
    match x {
        1 => TextAlignX::Center,
        2 => TextAlignX::Right,
        _ => TextAlignX::Left,
    }
}

pub fn to_text_align_y(y: u32) -> TextAlignY {
    match y {
        1 => TextAlignY::Top,
        2 => TextAlignY::Middle,
        3 => TextAlignY::Bottom,
        _ => TextAlignY::Baseline,
    }
}

const DEFAULT_TEXT: &str = "The quick brown fox jumped over the yellow dog.";
