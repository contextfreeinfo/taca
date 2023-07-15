#pragma once

#include <webgpu.h>

struct taca_gpu_BufferImpl;
typedef struct taca_gpu_BufferImpl* taca_gpu_Buffer;

struct taca_gpu_ShaderImpl;
typedef struct taca_gpu_ShaderImpl* taca_gpu_Shader;

taca_EXPORT taca_gpu_Shader taca_gpu_shaderCreate(const char* wgsl);

taca_EXPORT taca_gpu_Buffer taca_gpu_indexBufferCreate(size_t size, const void* data, WGPUIndexFormat format, taca_gpu_Buffer vertex);
taca_EXPORT taca_gpu_Buffer taca_gpu_uniformBufferCreate(size_t size);
taca_EXPORT taca_gpu_Buffer taca_gpu_vertexBufferCreate(size_t size, const void* data, const WGPUVertexBufferLayout* layout);

// Presume full refill of same buffer size by default.
taca_EXPORT void taca_gpu_bufferWrite(taca_gpu_Buffer buffer, const void* data);
taca_EXPORT void taca_gpu_draw(taca_gpu_Buffer buffer);
taca_EXPORT void taca_gpu_present(void);

// WGPU_EXPORT void wgpuRenderPassEncoderSetBindGroup(WGPURenderPassEncoder renderPassEncoder, uint32_t groupIndex, WGPUBindGroup group, uint32_t dynamicOffsetCount, uint32_t const * dynamicOffsets);
// c.wgpuRenderPassEncoderSetBindGroup(render_pass, 0, state.bind_group, 0, null);
// taca_EXPORT void taca_gpu_Bind(...);
