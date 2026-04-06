const std = @import("std");

pub fn build(b: *std.Build) void {
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});

    const sdl_mod = b.createModule(.{
        .root_source_file = b.path("../common/sdl.zig"),
        .target = target,
        .optimize = optimize,
    });

    const font_mod = b.createModule(.{
        .root_source_file = b.path("../common/font.zig"),
        .target = target,
        .optimize = optimize,
    });
    font_mod.addImport("sdl", sdl_mod);

    const sprite_mod = b.createModule(.{
        .root_source_file = b.path("../common/sprite.zig"),
        .target = target,
        .optimize = optimize,
    });
    sprite_mod.addImport("sdl", sdl_mod);

    const exe = b.addExecutable(.{
        .name = "micro_miyoo",
        .root_module = b.createModule(.{
            .root_source_file = b.path("src/main.zig"),
            .target = target,
            .optimize = optimize,
            .imports = &.{
                .{ .name = "sdl", .module = sdl_mod },
                .{ .name = "font", .module = font_mod },
                .{ .name = "sprite", .module = sprite_mod },
            },
        }),
    });

    exe.linkSystemLibrary("SDL2");
    exe.linkLibC();
    exe.addIncludePath(.{ .cwd_relative = "/usr/include" });
    b.installArtifact(exe);
}
