use miniquad::{UniformDesc, UniformType, VertexFormat};
use naga::{
    back::glsl::{self, WriterFlags},
    front::spv::{self, Options},
    proc::{BoundsCheckPolicies, BoundsCheckPolicy},
    valid::{Capabilities, ModuleInfo, ValidationFlags, Validator},
    AddressSpace, Binding, GlobalVariable, Handle, Module, ResourceBinding, Scalar, ScalarKind,
    ShaderStage, StructMember, Type, TypeInner, VectorSize,
};

pub struct Shader {
    pub autonames: Vec<String>,
    pub fragment_entries: Vec<String>,
    pub info: ModuleInfo,
    pub module: Module,
    pub uniforms: Vec<UniformDesc>,
    pub vertex_entries: Vec<VertexEntry>,
}

#[derive(Clone, Debug)]
pub struct VertexEntry {
    pub name: String,
    pub attributes: Vec<VertexFormat>,
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
        let mut autonames = Vec::<String>::new();
        for var in module.global_variables.iter_mut() {
            let var = var.1;
            if var.space == AddressSpace::Uniform {
                dig_uniforms(&mut uniforms, &mut autonames, var, &types);
            }
        }
        // Input attributes.
        let mut fragment_entries = vec![];
        let mut vertex_entries = vec![];
        for entry in &module.entry_points {
            match entry.stage {
                ShaderStage::Vertex => {
                    let mut attributes: Vec<Option<VertexFormat>> =
                        vec![None; entry.function.arguments.len()];
                    for arg in &entry.function.arguments {
                        if let Some(binding) = &arg.binding {
                            if let Binding::Location { location, .. } = binding {
                                if let Some(ty) = types[arg.ty.index()] {
                                    attributes[*location as usize] =
                                        Some(interpret_vertex_format(ty));
                                }
                            }
                        }
                    }
                    if attributes.iter().all(|attr| attr.is_some()) {
                        vertex_entries.push(VertexEntry {
                            name: entry.name.clone(),
                            attributes: attributes.iter().map(|attr| attr.unwrap()).collect(),
                        });
                    }
                }
                ShaderStage::Fragment => fragment_entries.push(entry.name.clone()),
                ShaderStage::Compute => todo!(),
            }
        }
        // Done.
        Shader {
            autonames,
            fragment_entries,
            info,
            module,
            uniforms,
            vertex_entries,
        }
    }

    pub fn to_glsl(&self, shader_stage: ShaderStage, entry_point: String) -> String {
        // crate::wasmic::print(&format!("{shader_stage:?} {}", &entry_point));
        let mut glsl = translate_to_glsl(&self.module, &self.info, shader_stage, entry_point);
        // Rename from naga conventions to common names across stages for miniquad needs.
        // The goal here is to share uniforms across stages, but types and blocks need to match.
        // Happily, naga seems to match type names for each writing.
        let (_, var_suffix) = match shader_stage {
            ShaderStage::Vertex => ("Vertex", "vs"),
            ShaderStage::Fragment => ("Fragment", "fs"),
            ShaderStage::Compute => todo!(),
        };
        glsl = flatten_out_uniform_blocks(&glsl);
        for name in &self.autonames {
            glsl = glsl.replace(&format!("{name}_{var_suffix}"), name);
        }
        // crate::wasmic::print(&format!("{glsl}"));
        glsl
    }
}

fn dig_uniforms(
    uniforms: &mut Vec<UniformDesc>,
    autonames: &mut Vec<String>,
    var: &GlobalVariable,
    types: &[Option<&Type>],
) {
    // Allocates a lot but expected to be rarely used.
    let name = if let Some(name) = &var.name {
        name.clone()
    } else if let Some(ResourceBinding { group, binding }) = var.binding {
        // Default naming convention in naga.
        let autoname = format!("_group_{group}_binding_{binding}");
        autonames.push(autoname.clone());
        autoname
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

fn flatten_out_uniform_blocks(glsl: &str) -> String {
    let mut result = String::new();
    for line in glsl.split('\n') {
        let start = "uniform ";
        let line = match () {
            _ if line.starts_with(start) => {
                // From `uniform type_4_block_0Vertex { type_4 _group_0_binding_0_vs; };`
                // To `uniform type_4 _group_0_binding_0;`
                result.push_str(start);
                let next = &line[start.len()..];
                let next = &next[next.find('{').unwrap() + 1..];
                let next = &next[next.find(|c| c != ' ').unwrap()..];
                &next[..=next.find(';').unwrap()]
            }
            _ => line,
        };
        result.push_str(line);
        result.push('\n');
    }
    result
}

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

fn interpret_vertex_format(ty: &Type) -> VertexFormat {
    match &ty.inner {
        naga::TypeInner::Matrix {
            columns: VectorSize::Quad,
            rows: VectorSize::Quad,
            scalar: SCALAR_FLOAT,
        } => VertexFormat::Mat4,
        naga::TypeInner::Scalar(SCALAR_FLOAT) => VertexFormat::Float1,
        naga::TypeInner::Scalar(Scalar {
            kind: ScalarKind::Float,
            width,
        }) => match width {
            1 => VertexFormat::Byte1,
            2 => VertexFormat::Short1,
            4 => VertexFormat::Int1,
            _ => panic!(),
        },
        naga::TypeInner::Vector {
            size,
            scalar: SCALAR_FLOAT,
        } => match size {
            VectorSize::Bi => VertexFormat::Float2,
            VectorSize::Tri => VertexFormat::Float3,
            VectorSize::Quad => VertexFormat::Float4,
        },
        naga::TypeInner::Vector {
            size,
            scalar:
                Scalar {
                    kind: ScalarKind::Float,
                    width,
                },
        } => match (size, width) {
            (VectorSize::Bi, 1) => VertexFormat::Byte2,
            (VectorSize::Tri, 1) => VertexFormat::Byte3,
            (VectorSize::Quad, 1) => VertexFormat::Byte4,
            (VectorSize::Bi, 2) => VertexFormat::Short2,
            (VectorSize::Tri, 2) => VertexFormat::Short3,
            (VectorSize::Quad, 2) => VertexFormat::Short4,
            (VectorSize::Bi, 4) => VertexFormat::Int2,
            (VectorSize::Tri, 4) => VertexFormat::Int3,
            (VectorSize::Quad, 4) => VertexFormat::Int4,
            _ => panic!(),
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
