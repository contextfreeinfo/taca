const std = @import("std");
// const print = std.debug.print;
const d = @import("./data.zig");
const g = @cImport({
    @cInclude("wgpu.h");
    @cInclude("webgpu-headers/webgpu.h");
});

pub fn buildPipeline(
    device: g.WGPUDevice,
    format: g.WGPUTextureFormat,
    bind_group_layout: g.WGPUBindGroupLayout,
) g.WGPURenderPipeline {
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
                    .code = @embedFile("./shader.opt.wgsl"),
                },
            ),
            .label = null,
            .hintCount = 0,
            .hints = null,
        },
    ) orelse unreachable;
    const vertex_attributes = [_]g.WGPUVertexAttribute{
        .{
            .format = g.WGPUVertexFormat_Float32x3,
            .offset = 0,
            .shaderLocation = 0,
        },
        .{
            .format = g.WGPUVertexFormat_Float32x3,
            .offset = d.vertex_norm_offset * @sizeOf(f32),
            .shaderLocation = 1,
        },
        .{
            .format = g.WGPUVertexFormat_Float32x3,
            .offset = d.vertex_color_offset * @sizeOf(f32),
            .shaderLocation = 2,
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
            .bindGroupLayoutCount = 1,
            .bindGroupLayouts = &[_]g.WGPUBindGroupLayout{bind_group_layout},
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
        .depthStencil = &std.mem.zeroInit(g.WGPUDepthStencilState, .{
            .format = g.WGPUTextureFormat_Depth24Plus,
            .depthWriteEnabled = true,
            .depthCompare = g.WGPUCompareFunction_Less,
            .stencilFront = std.mem.zeroInit(g.WGPUStencilFaceState, .{
                .compare = g.WGPUCompareFunction_Always,
            }),
            .stencilBack = std.mem.zeroInit(g.WGPUStencilFaceState, .{
                .compare = g.WGPUCompareFunction_Always,
            }),
            // TODO Tutorial encouraged these defaults but then wants zeros?
            // .stencilReadMask = 0xFFFFFFFF,
            // .stencilWriteMask = 0xFFFFFFFF,
        }),
        .multisample = .{
            .nextInChain = null,
            .count = 1,
            .mask = 0xFFFFFFFF,
            .alphaToCoverageEnabled = false,
        },
    }) orelse unreachable;
    return pipeline;
}
