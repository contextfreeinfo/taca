module fly @private;
import std::math;
import taca;

fn void main() @public @export("_start")
{
    taca::print("Hi from C3!");
    taca::title_set("Flyin'");
    // Getting closer to working here.
    taca::print(string::tformat("(%s)", 123));
    Shader fragment = taca::shader_new($embed("../out/fragment.spv"));
    Shader vertex = taca::shader_new($embed("../out/vertex.spv"));
    taca::buffer_new(INDEX, { &PLAYER_INDEX, $sizeof(PLAYER_INDEX) });
    taca::buffer_new(VERTEX, { &PLAYER_VERTEX, $sizeof(PLAYER_VERTEX) });
    taca::pipeline_new({
        .fragment = { .shader = fragment },
        .vertex = { .shader = vertex },
    });
}

fn void listen(int event) @public @export("listen")
{
    float fov = (float)math::deg_to_rad(45);
    Uniforms uniforms = {
        .proj = matrix4f_perspective(fov, 16f / 9f, 0.1, 100),
        .view = vector::matrix4f_look_at({0, 0, 5}, {0, 0, 0}, {0, 1, 0}),
    };
    taca::uniforms_set({ &uniforms, $sizeof(uniforms) });
    taca::draw(0, PLAYER_INDEX.len, 1);
}

struct Uniforms
{
    Matrix4f proj;
    Matrix4f view;
}
