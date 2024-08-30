module glb @private;
import std::collections::object;
import std::encoding::json;
import taca;

fault Fault {
    LENGTH,
    MAGIC,
    VERSION,
}

fn void! load(char[] bytes) @public {
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
    ChunkHeader bin_header;
    if (bin_ptr == (void*)(ptr + header.length)) {
        bin_ptr = null;
    } else if ((char*)(bin_ptr + 1) + bin_ptr.length != end) {
        return Fault.LENGTH?;
    } else {
        bin_header = *bin_ptr;
    }
    char* json_bytes = (char*)(json_ptr + 1);
    String json_string = (String)json_bytes[:json_ptr.length];
    // taca::print(json_bytes);
    // Read json data.
    Object* json = json::parse_string(json_string)!;
    Object* primitives =
        json.get("meshes").get_at(0).get("primitives").get_at(0)!;
    int indices = primitives.get_int("indices")!;
    taca::print(string::tformat("indices at %s", indices));
}

// Be lazy and just presume we're little endian. Always true within wasm.
// TODO Otherwise use lots of IntLE here or such like.
// TODO Would need our own bitstructs for float?

const uint BIN = 0x004e4942; // *(int*)(char*)"BIN\0"
const uint JSON = 0x4e4f534a; // *(int*)(char*)"JSON"
const uint MAGIC = 0x46546c67; // *(int*)(char*)"glTF"
const uint VERSION = 2;

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
