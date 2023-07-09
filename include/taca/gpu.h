#pragma once

#include <webgpu.h>

typedef struct taca_GpuConfig {
    WGPUIndexFormat indexFormat;
    size_t uniformSize;
    const WGPUVertexBufferLayout* vertexBufferLayout;
    /// Vertex shader "vertex_main" and fragment shader "fragment_main".
    const char* wgsl;
} taca_GpuConfig;

typedef enum taca_BufferKind {
    taca_BufferKind_Index = 1,
    taca_BufferKind_Uniform = 2,
    taca_BufferKind_Vertex = 3,
} taca_BufferKind;

struct taca_GpuBufferImpl;
typedef struct taca_GpuBufferImpl* taca_GpuBuffer;

taca_EXPORT taca_GpuBuffer taca_gpuIndexBufferInit();
taca_EXPORT taca_GpuBuffer taca_gpuUniformBufferInit();
taca_EXPORT taca_GpuBuffer taca_gpuVertexBufferInit(taca_GpuBuffer vertexBuffer);

taca_EXPORT void taca_gpuBufferWrite(taca_GpuBuffer buffer, size_t size, const void* data);
taca_EXPORT void taca_gpuDraw(taca_GpuBuffer buffer);
taca_EXPORT void taca_gpuInit(const taca_GpuConfig* config);
taca_EXPORT void taca_gpuPresent(void);

// WGPU_EXPORT void wgpuRenderPassEncoderSetBindGroup(WGPURenderPassEncoder renderPassEncoder, uint32_t groupIndex, WGPUBindGroup group, uint32_t dynamicOffsetCount, uint32_t const * dynamicOffsets);
// c.wgpuRenderPassEncoderSetBindGroup(render_pass, 0, state.bind_group, 0, null);
// taca_EXPORT void taca_gpuBind(...);
