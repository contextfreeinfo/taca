use naga::{
    back::glsl::{self, WriterFlags},
    front::spv::{self, Options},
    proc::BoundsCheckPolicies,
    valid::{Capabilities, ModuleInfo, ValidationFlags, Validator},
    Module, ShaderStage,
};

pub struct ShaderTranslation {
    buffer: String,
    module: Module,
    info: ModuleInfo,
}

#[repr(C)]
pub enum Stage {
    Vertex,
    Fragment,
}

#[no_mangle]
#[export_name = "shaderTranslationNew"]
pub fn shader_translation(source: &[u8]) -> *mut ShaderTranslation {
    let module = spv::parse_u8_slice(source, &Options::default()).unwrap();
    let mut validator = Validator::new(ValidationFlags::all(), Capabilities::empty());
    let info = validator.validate(&module).unwrap();
    let translation = ShaderTranslation {
        buffer: String::new(),
        module,
        info,
    };
    Box::into_raw(Box::new(translation))
}

#[no_mangle]
#[export_name = "shaderTranslationToGlsl"]
pub fn translate_to_glsl(translation: *mut ShaderTranslation, stage: Stage, entry_point: &str) {
    let mut translation = unsafe { &mut *translation };
    let stage = match stage {
        Stage::Vertex => ShaderStage::Vertex,
        Stage::Fragment => ShaderStage::Fragment,
    };
    translate_stage_to_glsl(&mut translation, stage, entry_point);
}

#[no_mangle]
#[export_name = "shaderTranslationBufferLen"]
pub fn translation_buffer_len(translation: *const ShaderTranslation) -> usize {
    let translation = unsafe { &*translation };
    translation.buffer.len()
}

#[no_mangle]
#[export_name = "shaderTranslationBufferPtr"]
pub fn translation_buffer_ptr(translation: *const ShaderTranslation) -> *const u8 {
    let translation = unsafe { &*translation };
    translation.buffer.as_ptr()
}

#[no_mangle]
#[export_name = "shaderTranslationClose"]
pub fn translation_close(translation: *mut ShaderTranslation) {
    unsafe {
        drop(Box::from_raw(translation));
    }
}

fn translate_stage_to_glsl(
    translation: &mut ShaderTranslation,
    shader_stage: naga::ShaderStage,
    entry_point: &str,
) {
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
        entry_point: entry_point.into(),
        multiview: None,
    };
    let mut writer = glsl::Writer::new(
        &mut translation.buffer,
        &translation.module,
        &translation.info,
        &options,
        &pipeline_options,
        // TODO What bounds checks???
        BoundsCheckPolicies::default(),
    )
    .unwrap();
    writer.write().unwrap();
}

#[cfg(test)]
mod tests {
    // use super::*;
    // #[test]
    // fn it_works() {
    //     let result = add(2, 2);
    //     assert_eq!(result, 4);
    // }
}
