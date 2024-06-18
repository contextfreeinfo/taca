use naga::{
    back::glsl::{self, WriterFlags},
    front::spv::{self, Options},
    proc::BoundsCheckPolicies,
    valid::{Capabilities, ModuleInfo, ValidationFlags, Validator},
    Module, ShaderStage,
};

#[cfg(not(target_arch = "wasm32"))]
#[derive(Debug)]
pub struct GlslShaders {
    pub vertex: String,
    pub fragment: String,
}

pub struct Shader {
    pub module: Module,
    pub info: ModuleInfo,
}

impl Shader {
    pub fn new(bytes: &[u8]) -> Shader {
        let module = spv::parse_u8_slice(bytes, &Options::default()).unwrap();
        let mut validator = Validator::new(ValidationFlags::all(), Capabilities::empty());
        let info = validator.validate(&module).unwrap();
        Shader { module, info }
    }

    pub fn to_glsl(&self, shader_stage: ShaderStage, entry_point: String) -> String {
        // crate::wasmic::print(&format!("{shader_stage:?} {}", &entry_point));
        translate_to_glsl(&self.module, &self.info, shader_stage, entry_point)
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub fn shaders() -> GlslShaders {
    let source = include_bytes!("shader.opt.spv");
    // println!("{source}");
    let module = spv::parse_u8_slice(source, &Options::default()).unwrap();
    let mut validator = Validator::new(ValidationFlags::all(), Capabilities::empty());
    let info = validator.validate(&module).unwrap();
    let vertex = translate_to_glsl(&module, &info, ShaderStage::Vertex, "vs_main".into());
    let fragment = translate_to_glsl(&module, &info, ShaderStage::Fragment, "fs_main".into());
    GlslShaders { vertex, fragment }
}

fn translate_to_glsl(
    module: &Module,
    info: &ModuleInfo,
    shader_stage: ShaderStage,
    entry_point: String,
) -> String {
    let options = glsl::Options {
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
        // TODO What bounds checks???
        BoundsCheckPolicies::default(),
    )
    .unwrap();
    writer.write().unwrap();
    // println!("{buffer}");
    buffer
}
