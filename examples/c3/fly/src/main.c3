module fly @private;
import taca;

fn void main() @public @export("_start")
{
    taca::print("Hi from C3!");
    taca::title_set("Flyin'");
    Shader* fragment = taca::shader_new($embed("../out/fragment.spv"));
    Shader* vertex = taca::shader_new($embed("../out/vertex.spv"));
    taca::buffer_new(INDEX, { &PLAYER_INDEX, $sizeof(PLAYER_INDEX) });
    taca::buffer_new(VERTEX, { &PLAYER_VERTEX, $sizeof(PLAYER_VERTEX) });
}

fn void listen(int event) @public @export("listen")
{
}

const short[*] PLAYER_INDEX =
{
    0, 1, 2,
};

const float[*] PLAYER_VERTEX =
{
    0, 1, 0, 0,
    1, -1, 0, 0,
    -1, -1, 0, 0,
};
