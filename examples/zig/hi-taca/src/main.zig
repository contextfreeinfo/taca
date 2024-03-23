const c = @cImport({
    @cInclude("taca.h");
});
const std = @import("std");

pub fn main() !void {
    c.taca_windowSetTitle("Taca-Simplified WebGPU");
}
