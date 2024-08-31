module glb @private;
import std::collections::object;
import std::encoding::json;
import taca;

struct Mesh @public {
    char[] pos; // float[3][]
    char[] norm; // float[3][]
    char[] uv; // float[2][]
    char[] idx; // ushort[]
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
    if (bin_ptr.type != BIN) return Fault.MAGIC?;
    // Read json data.
    char* json_bytes = (char*)(json_ptr + 1);
    String json_string = (String)json_bytes[:json_ptr.length];
    // taca::print(json_string);
    Object* spec = json::parse_string(json_string)!;
    defer spec.free();
    return build_mesh(spec, bin_bytes[:bin_ptr.length])!;
}

struct Accessor {
    usz count;
    ComponentType ctype;
    MultiType type;
    usz view;
}

fn Accessor! build_accessor(
    Object* accessors, usz idx, MultiType type, ComponentType ctype
) {
    if (idx >= accessors.get_len()) return Fault.LENGTH?;
    Object* acc = accessors.get_at(idx);
    ComponentType found_ctype = {|
        switch (acc.get_uint("componentType")!) {
            case ComponentType.FLOAT.code: return ComponentType.FLOAT;
            case ComponentType.USHORT.code: return ComponentType.USHORT;
            default: return Fault.TYPE?;
        }
    |}!;
    if (found_ctype != ctype) return Fault.TYPE?;
    MultiType found_type = {|
        switch (acc.get_string("type")!) {
            case "SCALAR": return MultiType.SCALAR;
            case "VEC2": return MultiType.VEC2;
            case "VEC3": return MultiType.VEC3;
            default: return Fault.TYPE?;
        }
    |}!;
    if (found_type != type) return Fault.TYPE?;
    return {
        .ctype = ctype,
        .count = acc.get_uint("count"),
        .type = type,
        .view = acc.get_uint("bufferView"),
    };
}

fn Mesh! build_mesh(Object* spec, char[] buffer) {
    // taca::print(string::tformat("%s", spec));
    // Spec out buffer. Expect very constrained options for now.
    Object* buffer_specs = spec.get("buffers")!;
    if (!buffer_specs.is_array()) return Fault.TYPE?;
    if (buffer_specs.get_len() != 1) return Fault.LENGTH?;
    usz buffer_spec_len = spec.get("buffers").get_at(0).get_int("byteLength")!;
    if (buffer_spec_len != buffer.len) return Fault.LENGTH?;
    // Spec out meshes.
    Object* meshes = spec.get("meshes")!;
    if (!meshes.is_array()) return Fault.TYPE?;
    if (meshes.get_len() != 1) return Fault.LENGTH?;
    Object* primitives_array = meshes.get_at(0).get("primitives")!;
    if (primitives_array.get_len() != 1) return Fault.LENGTH?;
    if (!primitives_array.is_array()) return Fault.TYPE?;
    // Primitives
    Object* primitives = primitives_array.get_at(0);
    Object* attributes = primitives.get("attributes")!;
    usz pos_idx = attributes.get_uint("POSITION")!;
    usz norm_idx = attributes.get_uint("NORMAL")!;
    usz uv_idx = attributes.get_uint("TEXCOORD_0")!;
    usz idx_idx = primitives.get_uint("indices")!;
    // Accessors
    Object* accessors = spec.get("accessors")!;
    if (!accessors.is_array()) return Fault.TYPE?;
    Accessor pos_acc = build_accessor(
        accessors, pos_idx, MultiType.VEC3, ComponentType.FLOAT
    )!;
    Accessor norm_acc = build_accessor(
        accessors, norm_idx, MultiType.VEC3, ComponentType.FLOAT
    )!;
    Accessor uv_acc = build_accessor(
        accessors, uv_idx, MultiType.VEC2, ComponentType.FLOAT
    )!;
    Accessor idx_acc = build_accessor(
        accessors, idx_idx, MultiType.SCALAR, ComponentType.USHORT
    )!;
    if (!(pos_acc.count == norm_acc.count && norm_acc.count == uv_acc.count)) {
        return Fault.LENGTH?;
    }
    // Views
    Object* views = spec.get("bufferViews")!;
    if (!views.is_array()) return Fault.TYPE?;
    Mesh mesh = {
        .pos = build_view(views, pos_acc, buffer)!,
        .norm = build_view(views, norm_acc, buffer)!,
        .uv = build_view(views, uv_acc, buffer)!,
        .idx = build_view(views, idx_acc, buffer)!,
    };
    // taca::print(string::tformat("==> %s vs %s", mesh.pos.len, pos.len));
    // TODO Print some pos and idx contents?
    return mesh;
}

fn char[]! build_view(Object* views, Accessor acc, char[] buffer) {
    if (acc.view >= views.get_len()) return Fault.LENGTH?;
    Object* view = views.get_at(acc.view);
    if (view.get_uint("buffer")! != 0) return Fault.LENGTH?;
    usz offset = view.get_uint("byteOffset")!;
    usz length = view.get_uint("byteLength")!;
    if (offset + length > buffer.len) return Fault.LENGTH?;
    // TODO Validate length vs count/type/ctype?
    // taca::print(string::tformat("%s %s", offset, length));
    return buffer[offset:length];
}

fault Fault {
    LENGTH,
    MAGIC,
    TYPE,
    VERSION,
}

const uint BIN = 0x004e4942; // *(int*)(char*)"BIN\0"
const uint JSON = 0x4e4f534a; // *(int*)(char*)"JSON"
const uint MAGIC = 0x46546c67; // *(int*)(char*)"glTF"
const uint VERSION = 2;

enum ComponentType : int (int code) {
    FLOAT = 5126,
    USHORT = 5123,
}

enum MultiType : int {
    SCALAR,
    VEC2,
    VEC3,
}

// Be lazy and just presume we're little endian. Always true within wasm.
// TODO Otherwise use lots of IntLE here or such like.
// TODO Would need our own bitstructs for float?

struct ChunkHeader {
    uint length;
    uint type;
}

struct Header {
    uint magic;
    uint version;
    uint length;
}
