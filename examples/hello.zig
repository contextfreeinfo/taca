// const c = @cImport({
//     @cInclude("hello.h");
// });

extern fn hello() void;

pub fn main() void {
    // c.hello();
    hello();
}
