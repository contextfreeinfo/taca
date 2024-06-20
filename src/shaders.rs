use miniquad::{UniformDesc, UniformType};
use naga::{
    back::glsl::{self, WriterFlags},
    front::spv::{self, Options},
    proc::BoundsCheckPolicies,
    valid::{Capabilities, ModuleInfo, ValidationFlags, Validator},
    AddressSpace, GlobalVariable, Handle, Module, ResourceBinding, Scalar, ScalarKind, ShaderStage,
    StructMember, Type, TypeInner, VectorSize,
};

pub struct Shader {
    pub module: Module,
    pub info: ModuleInfo,
    pub uniform_names: Vec<String>,
}

impl Shader {
    pub fn new(bytes: &[u8]) -> Shader {
        let mut module = spv::parse_u8_slice(bytes, &Options::default()).unwrap();
        let mut validator = Validator::new(ValidationFlags::all(), Capabilities::empty());
        let info = validator.validate(&module).unwrap();
        // TODO Some way to know the uniform name, but names are always invented on output. See naga::back::glsl::Writer::write_interface_block
        // TODO And type suffixes are also invented on output. See naga::proc::namer::call
        let mut types: Vec<Option<&Type>> = vec![
            None;
            module
                .types
                .iter()
                .map(|ty| ty.0.index())
                .max()
                .map_or(0, |max| max + 1)
        ];
        for ty in module.types.iter() {
            types[ty.0.index()] = Some(ty.1);
        }
        // println!("{:?}", &types);
        let mut uniforms = Vec::<UniformDesc>::new();
        for var in module.global_variables.iter_mut() {
            let var = var.1;
            if var.space == AddressSpace::Uniform {
                dig_uniforms(&mut uniforms, var, &types);
                match &types[var.ty.index()].as_ref().unwrap().inner {
                    naga::TypeInner::Struct { members, .. } => {
                        assert_eq!(1, members.len());
                        println!(
                            "{:?} {:?} {:?}",
                            var,
                            members[0].name,
                            types[members[0].ty.index()]
                        );
                        match &types[members[0].ty.index()].as_ref().unwrap().inner {
                            _ => {}
                        }
                    }
                    _ => panic!(),
                }
            }
        }
        // Extract sorted uniform names.
        let mut uniforms: Vec<_> = module
            .global_variables
            .iter()
            .filter(|var| var.1.space == AddressSpace::Uniform)
            .map(|var| var.1)
            .collect();
        uniforms.sort_by_key(|uniform| {
            let binding = uniform.binding.as_ref().unwrap();
            (binding.group, binding.binding)
        });
        // crate::wasmic::print(&format!("{:?}", uniforms));
        let uniform_names: Vec<String> = uniforms
            .iter()
            .map(|var| var.name.as_ref().map_or_else(|| "_", |name| name).into())
            .collect();
        // Done.
        Shader {
            module,
            info,
            uniform_names,
        }
    }

    pub fn to_glsl(&self, shader_stage: ShaderStage, entry_point: String) -> String {
        // crate::wasmic::print(&format!("{shader_stage:?} {}", &entry_point));
        translate_to_glsl(&self.module, &self.info, shader_stage, entry_point)
    }
}

fn dig_uniforms(uniforms: &mut Vec<UniformDesc>, var: &GlobalVariable, types: &[Option<&Type>]) {
    // Allocates a lot but expected to be rarely used.
    let name = if let Some(name) = &var.name {
        name.clone()
    } else if let Some(ResourceBinding { group, binding }) = var.binding {
        // Default naming convention in naga.
        format!("_group_{group}_binding_{binding}")
    } else {
        panic!()
    };
    dig_uniforms_typed(uniforms, &name, var.ty, types);
}

fn dig_uniforms_typed(
    uniforms: &mut Vec<UniformDesc>,
    name: &str,
    ty: Handle<Type>,
    types: &[Option<&Type>],
) {
    let ty = types[ty.index()].as_ref().unwrap();
    match &ty.inner {
        TypeInner::Matrix { .. } | TypeInner::Scalar(_) | naga::TypeInner::Vector { .. } => {
            uniforms.push(UniformDesc::new(&name, interpret_uniform_type(ty)))
        }
        naga::TypeInner::Struct { members, .. } => {
            dig_uniform_members(uniforms, &name, &members, types);
        }
        _ => panic!(),
    }
}

fn dig_uniform_members(
    uniforms: &mut Vec<UniformDesc>,
    base_name: &str,
    members: &[StructMember],
    types: &[Option<&Type>],
) {
    // Allocates a lot but expected to be rarely used.
    for (index, member) in members.iter().enumerate() {
        let member_name = if let Some(name) = &member.name {
            name.clone()
        } else {
            match index {
                0 => "member".into(),
                _ => format!("member_{index}"),
            }
        };
        let name = format!("{base_name}.{member_name}");
        dig_uniforms_typed(uniforms, &name, member.ty, types);
    }
}

const SCALAR_UINT: Scalar = Scalar {
    kind: ScalarKind::Uint,
    width: 4,
};

const SCALAR_FLOAT: Scalar = Scalar {
    kind: ScalarKind::Float,
    width: 4,
};

fn interpret_uniform_type(ty: &Type) -> UniformType {
    match &ty.inner {
        naga::TypeInner::Matrix {
            columns: VectorSize::Quad,
            rows: VectorSize::Quad,
            scalar: SCALAR_FLOAT,
        } => UniformType::Mat4,
        naga::TypeInner::Scalar(SCALAR_FLOAT) => UniformType::Float1,
        naga::TypeInner::Scalar(SCALAR_UINT) => UniformType::Int1,
        naga::TypeInner::Vector {
            size,
            scalar: SCALAR_FLOAT,
        } => match size {
            VectorSize::Bi => UniformType::Float2,
            VectorSize::Tri => UniformType::Float3,
            VectorSize::Quad => UniformType::Float4,
        },
        naga::TypeInner::Vector {
            size,
            scalar: SCALAR_UINT,
        } => match size {
            VectorSize::Bi => UniformType::Int2,
            VectorSize::Tri => UniformType::Int3,
            VectorSize::Quad => UniformType::Int4,
        },
        _ => panic!(),
    }
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
    // TODO Rename "uniform type_4_block_0Vertex { type_4 _group_0_binding_0_vs; };"
    // TODO to avoid _[fv]s suffixes.
    // const positionLocation = gl.getUniformLocation(program, 'u_light.position');
    // const colorLocation = gl.getUniformLocation(program, 'u_light.color');
    // const intensityLocation = gl.getUniformLocation(program, 'u_light.intensity');
    // TODO Parse for /^uniform (\w+)/ for uniform names?
    // TODO But miniquad::graphics::gl::load_shader_internal uses glGetUniformLocation not glGetUniformBlockIndex
    // TODO And we get different uniform names for vertex (_vs) vs fragment (_fs) shaders.
    // crate::wasmic::print(&format!("{buffer}"));
    buffer
}
