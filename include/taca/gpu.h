#pragma once

#include <webgpu.h>

typedef enum tac_DrawMode {
    tac_DrawMode_Default = 0,
    /// Fragment shader only for flat surface drawing.
    tac_DrawMode_FlatFragment = 0,
} tac_DrawMode;

typedef struct tac_GpuConfig {
    tac_DrawMode drawMode;
    size_t uniformSize;
    // TODO Infer mode by null vertex buffer layout?
    const WGPUVertexBufferLayout* vertexBufferLayout;
    /// Vertex shader vs_main and fragment shader fs_main.
    const char* wgsl;
} tac_GpuConfig;

tac_EXPORT void tac_gpuInit(const tac_GpuConfig* config);
// TODO tac_gpuUniformBufferWrite
// TODO tac_gpuVertexBufferWrite
