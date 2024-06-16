#[cfg(not(target_arch = "wasm32"))]
use std::{fs::File, io::Read};

use anyhow::Result;
#[cfg(not(target_arch = "wasm32"))]
use clap::{Args, Parser, Subcommand};
use miniquad::*;

mod platform;
mod shaders;
mod wasmic;

#[cfg(not(target_arch = "wasm32"))]
#[derive(clap::Parser)]
#[command(about, version, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[cfg(not(target_arch = "wasm32"))]
#[derive(Subcommand)]
enum Commands {
    Run(BuildArgs),
}

#[cfg(not(target_arch = "wasm32"))]
#[derive(Args)]
pub struct BuildArgs {
    pub app: String,
}

#[repr(C)]
struct Vertex {
    pos: [f32; 2],
    color: [f32; 4],
}

struct Stage {
    pipeline: Pipeline,
    bindings: Bindings,
    ctx: Box<dyn RenderingBackend>,
    drawn: i32,
}

impl Stage {
    pub fn new() -> Result<Stage> {
        let mut ctx: Box<dyn RenderingBackend> = window::new_rendering_backend();

        #[rustfmt::skip]
        let vertices: [Vertex; 3] = [
            Vertex { pos : [ -0.5, -0.5 ], color: [1., 0., 0., 1.] },
            Vertex { pos : [  0.5, -0.5 ], color: [0., 1., 0., 1.] },
            Vertex { pos : [  0.0,  0.5 ], color: [0., 0., 1., 1.] },
        ];
        let vertex_buffer = ctx.new_buffer(
            BufferType::VertexBuffer,
            BufferUsage::Immutable,
            BufferSource::slice(&vertices),
        );

        let indices: [u16; 3] = [0, 1, 2];
        let index_buffer = ctx.new_buffer(
            BufferType::IndexBuffer,
            BufferUsage::Immutable,
            BufferSource::slice(&indices),
        );

        let bindings = Bindings {
            vertex_buffers: vec![vertex_buffer],
            index_buffer: index_buffer,
            images: vec![],
        };

        let glsl = shaders::shaders()?;
        let shader = ctx
            .new_shader(
                //
                match ctx.info().backend {
                    Backend::OpenGl => ShaderSource::Glsl {
                        vertex: &glsl.vertex,
                        fragment: &glsl.fragment,
                    },
                    Backend::Metal => ShaderSource::Msl {
                        program: shader::METAL,
                    },
                },
                shader::meta(),
            )
            .unwrap();

        let pipeline = ctx.new_pipeline(
            //
            &[BufferLayout::default()],
            &[
                VertexAttribute::new("_p2vs_location0", VertexFormat::Float2),
                VertexAttribute::new("_p2vs_location1", VertexFormat::Float4),
            ],
            shader,
            PipelineParams::default(),
        );

        Ok(Stage {
            pipeline,
            bindings,
            ctx,
            drawn: 0,
        })
    }
}

impl EventHandler for Stage {
    fn update(&mut self) {}

    fn draw(&mut self) {
        // if self.drawn > 2 {
        //     return;
        // }
        self.ctx.begin_default_pass(Default::default()); //
        self.ctx.apply_pipeline(&self.pipeline); //
        self.ctx.apply_bindings(&self.bindings); //
        self.ctx.draw(0, 3, 1); //
        self.ctx.end_render_pass(); //
        self.ctx.commit_frame(); //
        self.drawn += 1;
    }
}

fn main() {
    // TODO Push all variation into wasmic?
    #[cfg(target_arch = "wasm32")]
    {
        // TODO Different loading for browser.
        wasmic::wasmish().expect("Bad wasm");
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let cli = Cli::parse();
        match &cli.command {
            Commands::Run(args) => {
                run(args.app.clone());
            }
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub fn run(path: String) {
    let mut buf = Vec::<u8>::new();
    let mut conf = conf::Conf::default();
    conf.platform.apple_gfx_api = conf::AppleGfxApi::Metal;
    conf.platform.webgl_version = conf::WebGLVersion::WebGL2;
    conf.window_title = "Taca".into();
    miniquad::start(conf, move || {
        File::open(path)
            .expect("Bad open")
            .read_to_end(&mut buf)
            .expect("Bad read");
        wasmic::wasmish(&buf).expect("Bad wasm");
        Box::new(Stage::new().expect("Bad init"))
    });
}

pub fn say_hi() {
    wasmic::print("Hi there!");
}

mod shader {
    use miniquad::*;

    // TODO Convert this with naga also.
    pub const METAL: &str = r#"
    #include <metal_stdlib>

    using namespace metal;

    struct Vertex
    {
        float2 in_pos   [[attribute(0)]];
        float4 in_color [[attribute(1)]];
    };

    struct RasterizerData
    {
        float4 position [[position]];
        float4 color [[user(locn0)]];
    };

    vertex RasterizerData vertexShader(Vertex v [[stage_in]])
    {
        RasterizerData out;

        out.position = float4(v.in_pos.xy, 0.0, 1.0);
        out.color = v.in_color;

        return out;
    }

    fragment float4 fragmentShader(RasterizerData in [[stage_in]])
    {
        return in.color;
    }"#;

    pub fn meta() -> ShaderMeta {
        ShaderMeta {
            images: vec![],
            uniforms: UniformBlockLayout { uniforms: vec![] },
        }
    }
}
