module taca;

extern fn Shader* buffer_new(
    BufferKind kind,
    SliceRaw slice,
) @extern("taca_RenderingContext_newBuffer");

extern fn void print(
    char[] text,
) @extern("taca_Window_print");

extern fn Shader* shader_new(
    char[] bytes,
) @extern("taca_RenderingContext_newShader");

extern fn void title_set(
    char[] title,
) @extern("taca_Window_setTitle");

distinct Buffer = void;

enum BufferKind : int
{
    VERTEX,
    INDEX,
}

distinct Shader = void;
