use naga::{back::glsl::{self, WriterFlags}, proc::{BoundsCheckPolicies, BoundsCheckPolicy}, valid::ModuleInfo, Module, ShaderStage};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}

#[no_mangle]
pub fn translate_to_glsl(
    module: &Module,
    info: &ModuleInfo,
    shader_stage: ShaderStage,
    entry_point: String,
) -> String {
    let options = glsl::Options {
        // TODO Bind to specific locations if miniquad could support explicit locations?
        // binding_map: todo!(),
        version: glsl::Version::Embedded {
            version: 300,
            is_webgl: true,
        },
        writer_flags: WriterFlags::empty(),
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
