use naga::{
    back::glsl,
    front::spv::{self, Options},
    proc::{BoundsCheckPolicies, BoundsCheckPolicy},
    valid::{Capabilities, ModuleInfo, ValidationFlags, Validator},
    Module,
};
use wasm_bindgen::prelude::*;

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
        ShaderStage::Fragment => glsl,
    }
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
