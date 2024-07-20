use lz4_flex::frame::FrameDecoder;
use naga::{
    back::glsl,
    front::spv::{self, Options},
    proc::{BoundsCheckPolicies, BoundsCheckPolicy},
    valid::{Capabilities, ModuleInfo, ValidationFlags, Validator},
    Module,
};
use std::io::Read;
use wasm_bindgen::prelude::*;

// #[wasm_bindgen]
// extern "C" {
//     fn alert(s: &str);
// }

// #[wasm_bindgen]
// pub fn greet(name: &str) {
//     alert(&format!("Hello, {}!", name));
// }

// TODO Rename this for js access?
#[wasm_bindgen(js_name = "lz4Decompress")]
pub fn lz4_decompress(source: &[u8]) -> Vec<u8> {
    let mut dest = vec![0u8; 0];
    FrameDecoder::new(source).read_to_end(&mut dest).unwrap();
    dest
}

#[wasm_bindgen]
pub struct Shader {
    module: Module,
    info: ModuleInfo,
}

#[wasm_bindgen]
pub enum ShaderStage {
    Vertex,
    Fragment,
}

#[wasm_bindgen(js_name = "shaderNew")]
pub fn shader_new(bytes: &[u8]) -> Shader {
    let module = spv::parse_u8_slice(bytes, &Options::default()).unwrap();
    let mut validator = Validator::new(ValidationFlags::all(), Capabilities::empty());
    let info = validator.validate(&module).unwrap();
    Shader { module, info }
}

#[wasm_bindgen(js_name = "shaderToGlsl")]
pub fn shader_to_glsl(shader: &Shader, stage: ShaderStage, entry_point: &str) -> String {
    let glsl = translate_to_glsl(
        &shader.module,
        &shader.info,
        match stage {
            ShaderStage::Vertex => naga::ShaderStage::Vertex,
            ShaderStage::Fragment => naga::ShaderStage::Fragment,
        },
        entry_point.into(),
    );
    match stage {
        ShaderStage::Vertex => glsl,
        ShaderStage::Fragment => munge_fragment(&glsl),
    }
}

fn munge_fragment(glsl: &str) -> String {
    // TODO Move this munging to js?
    let mut result = String::new();
    for line in glsl.split('\n') {
        if line.starts_with("uniform ") {
            // Put in our own uniform first.
            result.push_str("struct taca_uniform_struct { vec2 size; };\n");
            result.push_str("uniform taca_uniform_block { taca_uniform_struct taca; };\n");
        }
        // TODO Ensure word boundaries! Or figure out how to modify naga modules directly.
        let line = line.replace(
            "gl_FragCoord",
            "vec4(gl_FragCoord.x, taca.size.y - gl_FragCoord.y, gl_FragCoord.zw)",
        );
        result.push_str(&line);
        result.push('\n');
    }
    result
}

fn translate_to_glsl(
    module: &Module,
    info: &ModuleInfo,
    shader_stage: naga::ShaderStage,
    entry_point: String,
) -> String {
    let options = glsl::Options {
        // TODO Bind to specific locations if miniquad could support explicit locations?
        // binding_map: todo!(),
        version: glsl::Version::Embedded {
            version: 300,
            is_webgl: true,
        },
        ..glsl::Options::default()
    };
    let pipeline_options = glsl::PipelineOptions {
        shader_stage,
        entry_point,
        multiview: None,
    };
    let mut buffer = String::new();
    let mut writer = glsl::Writer::new(
        &mut buffer,
        module,
        info,
        &options,
        &pipeline_options,
        BoundsCheckPolicies {
            // Be safe by default until I know better.
            index: BoundsCheckPolicy::Restrict,
            buffer: BoundsCheckPolicy::Restrict,
            image_load: BoundsCheckPolicy::Restrict,
            image_store: BoundsCheckPolicy::Restrict,
            binding_array: BoundsCheckPolicy::Restrict,
        },
    )
    .unwrap();
    writer.write().unwrap();
    buffer
}
