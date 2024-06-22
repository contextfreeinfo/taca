const std = @import("std");

pub fn build(b: *std.Build) void {
    const target = b.resolveTargetQuery(.{
        .cpu_arch = .wasm32,
        .os_tag = .freestanding,
    });
    // Build exe.
    const optimize = std.builtin.OptimizeMode.ReleaseSmall;
    const exe = b.addExecutable(.{
        // .linkage = .dynamic,
        .name = "hi",
        .optimize = optimize,
        .root_source_file = b.path("src/main.zig"),
        .target = target,
    });
    // exe.addIncludePath(.{ .path = "../../../include" });
    // exe.export_table = false;
    exe.root_module.export_symbol_names = &[_][]const u8{
        "config",
        "listen",
    };
    b.installArtifact(exe);
    // Set up for testing.
    // const lib_unit_tests = b.addTest(.{
    //     .root_source_file = .{ .path = "src/root.zig" },
    //     .target = target,
    //     .optimize = optimize,
    // });
    // const run_lib_unit_tests = b.addRunArtifact(lib_unit_tests);
    // const exe_unit_tests = b.addTest(.{
    //     .root_source_file = .{ .path = "src/main.zig" },
    //     .target = target,
    //     .optimize = optimize,
    // });
    // const run_exe_unit_tests = b.addRunArtifact(exe_unit_tests);
    // const test_step = b.step("test", "Run unit tests");
    // test_step.dependOn(&run_lib_unit_tests.step);
    // test_step.dependOn(&run_exe_unit_tests.step);
}
