#pragma once

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>

// Enums

// Note that size in C requires C23.
typedef enum /* : uint32_t */ {
    taca_EventKind_Frame,
    taca_EventKind_Key,
    taca_EventKind_TasksDone,
} taca_EventKind;

typedef enum {
    taca_Key_None,
    taca_Key_ArrowUp,
    taca_Key_ArrowDown,
    taca_Key_ArrowLeft,
    taca_Key_ArrowRight,
    taca_Key_Space,
    taca_Key_Escape,
} taca_Key;

typedef enum {
    taca_SoundRateKind_Semitones,
    taca_SoundRateKind_Factor,
} taca_SoundRateKind;

typedef enum {
    taca_Step_Vertex,
    taca_Step_Instance,
} taca_Step;

// Handles

typedef size_t taca_Pipeline;
typedef size_t taca_Shader;
typedef size_t taca_Sound;
typedef size_t taca_SoundPlay;

// Supports

#define taca_span_define(name, item_type) \
typedef struct { \
    const item_type* data; \
    size_t size; \
} name

taca_span_define(taca_BytesView, uint8_t);
taca_span_define(taca_StringView, char);

typedef struct {
    float x;
    float y;
} taca_Vec2;

// Primaries

typedef struct {
    size_t shader_location;
    size_t value_offset;
} taca_AttributeInfo;

taca_span_define(taca_AttributeInfos, char);

typedef struct {
    size_t first_attribute;
    taca_Step step;
    size_t stride;
} taca_BufferInfo;

taca_span_define(taca_BufferInfos, char);

typedef struct {
    bool pressed;
    taca_Key key;
    uint8_t text[4];
} taca_KeyEvent;

typedef struct {
    taca_StringView entry;
    taca_Shader shader;
} taca_PipelineShaderInfo;

typedef struct {
    bool depth_test;
    taca_PipelineShaderInfo fragment;
    taca_PipelineShaderInfo vertex;
    taca_AttributeInfos vertex_attributes;
    taca_BufferInfos vertex_buffers;
} taca_PipelineInfo;

typedef struct {
    taca_Sound sound;
    float rate;
    taca_SoundRateKind rate_kind;
} taca_SoundPlayInfo;

typedef struct {
    taca_Vec2 pointer;
    uint32_t press;
    taca_Vec2 size;
} taca_WindowState;

// Functions

#ifdef __cplusplus
extern "C" {
#endif

// clang-format off

__attribute__((import_name("taca_key_event")))
taca_KeyEvent taca_key_event(void);

__attribute__((import_name("taca_pipeline_new")))
taca_Pipeline taca_pipeline_new(const taca_PipelineInfo* info);

// __attribute__((import_module("taca"), import_name("print")))
__attribute__((import_name("taca_print")))
void taca_print(taca_StringView text);

__attribute__((import_name("taca_shader_new")))
taca_Shader taca_shader_new(taca_BytesView bytes);

__attribute__((import_name("taca_sound_decode")))
taca_Sound taca_sound_decode(taca_BytesView bytes);

__attribute__((import_name("taca_sound_play")))
// TODO Without explicit pointer, and if only one field, this gets passed as the field value.
taca_SoundPlay taca_sound_play(const taca_SoundPlayInfo* info);

__attribute__((import_name("taca_title_update")))
void taca_title_update(taca_StringView text);

__attribute__((import_name("taca_window_state")))
taca_WindowState taca_window_state(void);

// clang-format on

#ifdef __cplusplus
}
#endif
