module glb @private;
import std::collections::object;
import std::encoding::json;
import taca;

struct Mesh @public {
    float[3][] pos;
    float[3][] norm;
    float[2][] uv;
    ushort[] index;
}

fn Mesh! load(char[] bytes) @public {
    char* ptr = bytes;
    // Header
    Header header = *(Header*)ptr;
    if (header.magic != MAGIC) return Fault.MAGIC?;
    if (header.version != VERSION) return Fault.VERSION?;
    if (header.length != bytes.len) return Fault.LENGTH?;
    char* end = ptr + bytes.len;
    // Json
    ChunkHeader* json_ptr = (ChunkHeader*)(ptr + $sizeof(header));
    if ((char*)(json_ptr + 1) > end) {
        return Fault.LENGTH?;
    }
    if (json_ptr.type != JSON) return Fault.MAGIC?;
    // Binary
    ChunkHeader* bin_ptr = (ChunkHeader*)(
        (char*)json_ptr + $sizeof(*json_ptr) + json_ptr.length
    );
    if (bin_ptr > (void*)(ptr + header.length)) return Fault.LENGTH?;
    char* bin_bytes = (char*)(bin_ptr + 1);
    if (bin_ptr == (void*)(ptr + header.length)) {
        // No bin chunk at all, and we want one.
        return Fault.LENGTH?;
    } else if (bin_bytes + bin_ptr.length != end) {
        // Bad bin chunk length.
        return Fault.LENGTH?;
    }
    // Read json data.
    char* json_bytes = (char*)(json_ptr + 1);
    String json_string = (String)json_bytes[:json_ptr.length];
    // taca::print(json_string);
    Object* spec = json::parse_string(json_string)!;
    defer spec.free();
    return build_mesh(spec, bin_bytes[:bin_ptr.length])!;
}

struct Accessor {
    ComponentType component_type;
    usz count;
    MultiType type;
    usz view;
}

fn Accessor! build_accessor(Object* acc) {
    return {};
}

fn Mesh! build_mesh(Object* spec, char[] buffer) {
    // Spec out buffer. Expect very constrained options for now.
    Object* buffer_specs = spec.get("buffers")!;
    if (buffer_specs.get_len() != 1) return Fault.LENGTH?;
    usz buffer_spec_len = spec.get("buffers").get_at(0).get_int("byteLength")!;
    if (buffer_spec_len != buffer.len) return Fault.LENGTH?;
    // Spec out meshes.
    Object* meshes = spec.get("meshes")!;
    if (meshes.get_len() != 1) return Fault.LENGTH?;
    Object* primitives_array = meshes.get_at(0).get("primitives")!;
    if (primitives_array.get_len() != 1) return Fault.LENGTH?;
    // Primitives
    Object* primitives = primitives_array.get_at(0);
    Object* attributes = primitives.get("attributes")!;
    usz pos_idx = attributes.get_int("POSITION")!;
    usz norm_idx = attributes.get_int("NORMAL")!;
    usz uv_idx = attributes.get_int("TEXCOORD_0")!;
    usz index_idx = primitives.get_int("indices")!;
    // Accessors
    Object* accessors = spec.get("accessors")!;
    Accessor pos_acc = build_accessor(accessors.get_at(pos_idx))!;
    Accessor norm_acc = build_accessor(accessors.get_at(norm_idx))!;
    Accessor uv_acc = build_accessor(accessors.get_at(uv_idx))!;
    Accessor index_acc = build_accessor(accessors.get_at(index_idx))!;
    // Views
    Object* views = spec.get("bufferViews")!;
    // taca::print(string::tformat("indices at %s", indices));
    return {};
}

fault Fault {
    LENGTH,
    MAGIC,
    VERSION,
}

// Be lazy and just presume we're little endian. Always true within wasm.
// TODO Otherwise use lots of IntLE here or such like.
// TODO Would need our own bitstructs for float?

const uint BIN = 0x004e4942; // *(int*)(char*)"BIN\0"
const uint JSON = 0x4e4f534a; // *(int*)(char*)"JSON"
const uint MAGIC = 0x46546c67; // *(int*)(char*)"glTF"
const uint VERSION = 2;

enum ComponentType : int (int code) {
    FLOAT = 5126,
    USHORT = 5123,
}

enum MultiType : int {
    VEC2,
    VEC3,
    SCALAR,
}

// struct Chunk {
//     inline ChunkHeader header;
//     char[] bytes;
// }

struct ChunkHeader {
    uint length;
    uint type;
}

struct Header {
    uint magic;
    uint version;
    uint length;
}
