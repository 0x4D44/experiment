const std = @import("std");

pub fn build(b: *std.Build) void {
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});

    // Main executable
    const exe = b.addExecutable(.{
        .name = "blackjack",
        .root_source_file = b.path("src/main.zig"),
        .target = target,
        .optimize = optimize,
    });

    b.installArtifact(exe);

    // Test suite
    const test_exe = b.addTest(.{
        .root_source_file = b.path("src/game_test.zig"),
        .target = target,
        .optimize = optimize,
    });

    const run_test = b.addRunArtifact(test_exe);

    // Build step
    const build_step = b.step("build", "Build the blackjack game");
    build_step.dependOn(&exe.step);

    // Test step
    const test_step = b.step("test", "Run all tests");
    test_step.dependOn(&run_test.step);

    // Run step
    const run_step = b.step("run", "Run the game");
    const run_cmd = b.addRunArtifact(exe);
    run_step.dependOn(&run_cmd.step);

    // Default step
    b.default_step.dependOn(&build_step);
}
