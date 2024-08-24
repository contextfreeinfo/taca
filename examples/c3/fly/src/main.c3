module fly @private;
import taca;

fn void main() @public @export("_start")
{
    taca::print("Hi from C3!");
    taca::title_set("Flyin'");
    Shader* fragment = taca::shader_new($embed("fragment.spv"));
    Shader* vertex = taca::shader_new($embed("vertex.spv"));
}

fn void listen(int event) @public @export("listen")
{
}
