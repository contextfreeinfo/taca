module taca;

distinct Shader = void;

extern fn void print(char[] text) @extern("taca_Window_print");
extern fn Shader* shader_new(char[] bytes) @extern("taca_RenderingContext_newShader");
extern fn void title_set(char[] title) @extern("taca_Window_setTitle");
