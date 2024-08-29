module glb @private;
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
    // taca::print(string::tformat("%x vs %x", header.magic, magic));
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
