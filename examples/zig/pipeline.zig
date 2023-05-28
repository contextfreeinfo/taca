// const print = @import("std").debug.print;
const d = @import("./data.zig");
const g = @cImport({
    @cInclude("wgpu.h");
    @cInclude("webgpu-headers/webgpu.h");
});

pub fn buildPipeline(device: g.WGPUDevice, format: g.WGPUTextureFormat) g.WGPURenderPipeline {
    // From:
    // https://github.com/eliemichel/LearnWebGPU-Code/blob/b089aa69e27965af04045098287f02a23b2a8845/main.cpp
    const shader_module = g.wgpuDeviceCreateShaderModule(
        device,
        &g.WGPUShaderModuleDescriptor{
            .nextInChain = @ptrCast(
                *const g.WGPUChainedStruct,
                &g.WGPUShaderModuleWGSLDescriptor{
                    .chain = .{
                        .next = null,
                        .sType = g.WGPUSType_ShaderModuleWGSLDescriptor,
                    },
                    .code = @embedFile("./shader.wgsl"),
                },
            ),
            .label = null,
            .hintCount = 0,
            .hints = null,
        },
    ) orelse unreachable;
    const vertex_attributes = [_]g.WGPUVertexAttribute{
        .{
            .format = g.WGPUVertexFormat_Float32x2,
            .offset = 0,
            .shaderLocation = 0,
        },
        .{
            .format = g.WGPUVertexFormat_Float32x3,
            .offset = d.vertex_color_offset * @sizeOf(f32),
            .shaderLocation = 1,
        },
    };
    const vertex_buffer_layout = g.WGPUVertexBufferLayout{
        .arrayStride = d.vertex_stride,
        .stepMode = g.WGPUVertexStepMode_Vertex,
        .attributeCount = vertex_attributes.len,
        .attributes = &vertex_attributes,
    };
    const pipeline_layout = g.wgpuDeviceCreatePipelineLayout(
        device,
        &g.WGPUPipelineLayoutDescriptor{
            .nextInChain = null,
            .label = null,
            .bindGroupLayoutCount = 0,
            .bindGroupLayouts = null,
        },
    ) orelse unreachable;
    const pipeline = g.wgpuDeviceCreateRenderPipeline(device, &g.WGPURenderPipelineDescriptor{
        .nextInChain = null,
        .label = null,
        .layout = pipeline_layout,
        .vertex = .{
            .nextInChain = null,
            .module = shader_module,
            .entryPoint = "vs_main",
            .constantCount = 0,
            .constants = null,
            .bufferCount = 1,
            .buffers = &vertex_buffer_layout,
        },
        .fragment = &g.WGPUFragmentState{
            .nextInChain = null,
            .module = shader_module,
            .entryPoint = "fs_main",
            .constantCount = 0,
            .constants = null,
            .targetCount = 1,
            .targets = &[_]g.WGPUColorTargetState{
                .{
                    .nextInChain = null,
                    .format = format,
                    .blend = null,
                    .writeMask = g.WGPUColorWriteMask_All,
                },
            },
        },
        .primitive = .{
            .nextInChain = null,
            .topology = g.WGPUPrimitiveTopology_TriangleList,
            .stripIndexFormat = g.WGPUIndexFormat_Undefined,
            .frontFace = g.WGPUFrontFace_CCW,
            .cullMode = g.WGPUCullMode_None,
        },
        .depthStencil = null,
        .multisample = .{
            .nextInChain = null,
            .count = 1,
            .mask = 0xFFFFFFFF,
            .alphaToCoverageEnabled = false,
        },
    }) orelse unreachable;
    return pipeline;
}
