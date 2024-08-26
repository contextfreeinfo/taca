module fly @private;
import std::math;
import taca;

fn void main() @public @export("_start")
{
    taca::print("Hi from C3!");
    taca::title_set("Flyin'");
    // String formatting needs allocation which isn't yet working for c3 wasm.
    // string::new_format("hi");
    // taca::print(string::new_format("hi"));
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
    Matrix4f view = vector::matrix4f_look_at({0, 0, 5}, {0, 0, 0}, {0, 1, 0});
    Matrix4f projection = matrix4f_perspective(fov, 16f / 9f, 0.1, 100);
    taca::draw(0, PLAYER_INDEX.len, 1);
}
