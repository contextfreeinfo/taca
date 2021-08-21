const c = @cImport({
    @cInclude("hello.h");
});

const z = struct {
    extern fn hello() void;
};

pub fn main() void {
    c.hello();
    z.hello();
}
