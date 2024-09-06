const std = @import("std");

pub fn build(b: *std.Build) !void {
    const target = b.resolveTargetQuery(.{
        .cpu_arch = .wasm32,
        .os_tag = .freestanding,
    });
    const optimize = std.builtin.OptimizeMode.ReleaseSmall;
    const names = [_][]const u8{ "hi", "hi2" };
    for (names) |name| {
        const root_source_file = try std.fmt.allocPrint(b.allocator, "src/{s}.zig", .{name});
        const exe = b.addExecutable(.{
            .name = name,
            .optimize = optimize,
            .root_source_file = b.path(root_source_file),
            .target = target,
        });
        // exe.addIncludePath(.{ .path = "../../../include" });
        exe.entry = .disabled;
        // exe.export_table = false;
        exe.root_module.export_symbol_names = &[_][]const u8{ "start", "update" };
        b.installArtifact(exe);
    }
}
