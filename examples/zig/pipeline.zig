const std = @import("std");
// const print = std.debug.print;
const d = @import("./data.zig");
const c = @cImport({
    @cInclude("taca.h");
    @cInclude("wgpu.h");
    @cInclude("webgpu-headers/webgpu.h");
});

pub fn buildPipeline(
    device: c.WGPUDevice,
    format: c.WGPUTextureFormat,
    bind_group_layout: c.WGPUBindGroupLayout,
) c.WGPURenderPipeline {
    // From:
    // https://github.com/eliemichel/LearnWebGPU-Code/blob/b089aa69e27965af04045098287f02a23b2a8845/main.cpp
    const shader_module = c.wgpuDeviceCreateShaderModule(
        device,
        &c.WGPUShaderModuleDescriptor{
            .nextInChain = @ptrCast(
                *const c.WGPUChainedStruct,
                &c.WGPUShaderModuleWGSLDescriptor{
                    .chain = .{
                        .next = null,
                        .sType = c.WGPUSType_ShaderModuleWGSLDescriptor,
                    },
                    .code = @embedFile("./shader.opt.wgsl"),
                },
            ),
            .label = null,
            .hintCount = 0,
            .hints = null,
        },
    ) orelse unreachable;
    const vertex_attributes = [_]c.WGPUVertexAttribute{
        .{
            .format = c.WGPUVertexFormat_Float32x3,
            .offset = 0,
            .shaderLocation = 0,
        },
        .{
            .format = c.WGPUVertexFormat_Float32x3,
            .offset = d.vertex_norm_offset * @sizeOf(f32),
            .shaderLocation = 1,
        },
        .{
            .format = c.WGPUVertexFormat_Float32x3,
            .offset = d.vertex_color_offset * @sizeOf(f32),
            .shaderLocation = 2,
        },
    };
    const vertex_buffer_layout = c.WGPUVertexBufferLayout{
        .arrayStride = d.vertex_stride,
        .stepMode = c.WGPUVertexStepMode_Vertex,
        .attributeCount = vertex_attributes.len,
        .attributes = &vertex_attributes,
    };
    const pipeline_layout = c.wgpuDeviceCreatePipelineLayout(
        device,
        &c.WGPUPipelineLayoutDescriptor{
            .nextInChain = null,
            .label = null,
            .bindGroupLayoutCount = 1,
            .bindGroupLayouts = &[_]c.WGPUBindGroupLayout{bind_group_layout},
        },
    ) orelse unreachable;
    const pipeline = c.wgpuDeviceCreateRenderPipeline(device, &c.WGPURenderPipelineDescriptor{
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
        .fragment = &c.WGPUFragmentState{
            .nextInChain = null,
            .module = shader_module,
            .entryPoint = "fs_main",
            .constantCount = 0,
            .constants = null,
            .targetCount = 1,
            .targets = &[_]c.WGPUColorTargetState{
                .{
                    .nextInChain = null,
                    .format = format,
                    .blend = null,
                    .writeMask = c.WGPUColorWriteMask_All,
                },
            },
        },
        .primitive = .{
            .nextInChain = null,
            .topology = c.WGPUPrimitiveTopology_TriangleList,
            .stripIndexFormat = c.WGPUIndexFormat_Undefined,
            .frontFace = c.WGPUFrontFace_CCW,
            .cullMode = c.WGPUCullMode_None,
        },
        .depthStencil = &std.mem.zeroInit(c.WGPUDepthStencilState, .{
            .format = c.WGPUTextureFormat_Depth24Plus,
            .depthWriteEnabled = true,
            .depthCompare = c.WGPUCompareFunction_Less,
            .stencilFront = std.mem.zeroInit(c.WGPUStencilFaceState, .{
                .compare = c.WGPUCompareFunction_Always,
            }),
            .stencilBack = std.mem.zeroInit(c.WGPUStencilFaceState, .{
                .compare = c.WGPUCompareFunction_Always,
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
