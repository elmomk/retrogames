# After-Action Report: Porting Retro Arcade to the Miyoo Mini Plus

**Date:** 2026-04-05
**Status:** Concluded — ChaiLove/RetroArch adopted as primary Miyoo target

---

## Objective

Port the 9 retro arcade games (Nano Wizards, Neon Defender, Shadow Blade, Arena Blitz, Dragon Fury, Pixel Knight, Nova Evader, Chrome Viper, Neon Runner) from browser HTML5 to run natively on the Miyoo Mini Plus handheld.

The Miyoo Mini Plus runs a custom Linux (OnionOS) on an ARM Cortex-A7 with a 640×480 display driven by SigmaStar hardware. It ships with a patched SDL2 that uses a custom `mmiyoo` framebuffer backend in place of X11.

---

## Approach 1: Rust + Macroquad (Miniquad)

**Rationale.** The original Miyoo ports were written in Rust using [Macroquad 0.4](https://macroquad.rs/), which wraps Miniquad for cross-platform rendering. Rust's cross-compilation story is mature and the games were already written — the plan was to cross-compile the existing code for `armv7-unknown-linux-gnueabihf` and copy the binaries to the device.

### What happened

**Problem 1: glibc version mismatch.**
CI builds linked against glibc 2.34 (Ubuntu 22.04). The Miyoo ships glibc 2.28. Running the binary produced an immediate `GLIBC_2.29 not found` error.

*Fix:* Switched to `cargo-zigbuild`, which uses Zig's cross-linker to target a specific glibc version:
```bash
cargo zigbuild --target armv7-unknown-linux-gnueabihf --release \
  -Z build-std=std,panic_abort
```
Targeting glibc 2.28 worked — the binary started.

**Problem 2: Miniquad requires X11.**
Miniquad's Linux backend opens a display via `XOpenDisplay()`. The Miyoo has no X server. The binary exited immediately with:
```
Error: failed to open display
```

*Fix attempt 1:* The parasyte runtime at `/mnt/SDCARD/.tmp_update/lib/parasyte/` ships a complete set of user-space libs (glibc 2.28, SDL2, libX11, libEGL, libGLES) intended for PICO-8 and Drastic. Setting `LD_LIBRARY_PATH` to include that directory allowed the dynamic linker to load `libX11.so.6` — but then Miniquad needed `libxkbcommon.so.0`, then `libXi.so.6`, then more transitive dependencies.

*Fix attempt 2:* Manually resolved each missing library from the parasyte bundle, adding them one by one to `LD_LIBRARY_PATH`. All symbols eventually loaded.

**Final outcome.** With every library resolved, `XOpenDisplay()` was still called and still failed — because there is no X server process running on the device. The parasyte X11 library is a stub designed for PICO-8's own display path; it does not conjure an X server. Miniquad's architecture assumes X11 and offers no escape hatch on Linux.

**Status: FAILED.** Fundamental architecture mismatch. Macroquad/Miniquad on Linux requires X11; the Miyoo has none.

---

## Approach 2: Rust + SDL2 Native

**Rationale.** The Miyoo's custom `mmiyoo` SDL2 video driver bypasses X11 entirely — it writes directly to the SigmaStar framebuffer. If the game used SDL2 directly instead of Miniquad, it could use that driver and avoid the X11 requirement altogether.

### What was built

A shared rendering crate `miyoo/retro-sdl2/` was written from scratch with 8 modules:

| Module | Responsibility |
|--------|---------------|
| `lib.rs` | Public API and game loop |
| `renderer.rs` | SDL2 canvas, texture management |
| `sprite.rs` | Procedural pixel-art sprite system |
| `font.rs` | Bitmap font renderer |
| `input.rs` | D-pad + button mapping |
| `effects.rs` | CRT scanlines, vignette |
| `color.rs` | Palette and color utilities |
| `timing.rs` | Fixed timestep, delta time |

`miyoo/micro/` (Nano Wizards) was ported to use this crate.

Cross-compilation used `cargo-zigbuild` for the glibc 2.28 constraint, plus a linker stub at `miyoo/sdl2-stub/` (a pair of `.so` files with the correct soname) to satisfy the SDL2 link-time dependency without needing the actual Miyoo SDL2 at build time.

Required environment on device:
```bash
export SDL_VIDEODRIVER=mmiyoo
export SDL_AUDIODRIVER=mmiyoo
export EGL_VIDEODRIVER=mmiyoo
# Kill the stock display processes to free the framebuffer
killall /dev/l disp_init 2>/dev/null
```

All Miyoo lib paths were added to `LD_LIBRARY_PATH`:
```
/config/lib:/mnt/SDCARD/.tmp_update/lib/parasyte:/usr/lib:/lib
```

### What happened

SDL2 initialized without error. `SDL_CreateWindow` succeeded. `SDL_CreateRenderer` succeeded. Textures were created and `SDL_RenderCopy` returned 0 (success). But the screen stayed black — no pixels appeared.

Variations attempted:
- `SDL_WINDOW_FULLSCREEN` flag
- `SDL_RENDERER_SOFTWARE` (bypassing GPU)
- `SDL_RENDERER_ACCELERATED`
- Explicit `640×480` window size matching Miyoo's native resolution
- `SDL_RenderPresent` followed by `SDL_Delay` to rule out timing
- Writing directly to a pixel buffer via `SDL_LockTexture`

None produced visible output.

**Root cause (hypothesis).** The `mmiyoo` SDL2 driver likely requires a proprietary initialization sequence that touches SigmaStar hardware registers or calls into `libmi_ao.so` / `libmi_disp.so` before the first present. This sequence is undocumented and was reverse-engineered by the OnionOS team for specific launchers (MainUI, GameSwitcher). Without performing that initialization, SDL2's internal state and the hardware framebuffer are out of sync even though the API reports success.

**Status: FAILED.** SDL2 init succeeds and the API behaves correctly, but nothing renders. The `mmiyoo` driver's initialization contract is undocumented.

---

## Approach 3: Zig + SDL2 Native

**Rationale.** Parallel exploration — same mmiyoo backend, different language. Zig's C interop is direct (no FFI ceremony) and `zig build` handles cross-compilation natively.

### What was built

- `zig/common/` — shared SDL2 module (window, renderer, input, sprite, font)
- `zig/micro/` — full Nano Wizards port, 2033 lines, compiles for ARM

**Status: NOT TESTED on device.** The same mmiyoo initialization problem would apply. This approach was shelved once the Rust SDL2 failure was diagnosed.

The Zig port remains valid for other Linux handhelds that use standard SDL2 (Anbernic RG35XX, Powkiddy X55, etc.).

---

## Approach 4: ChaiLove via RetroArch

**Rationale.** RetroArch is the canonical way to run software on the Miyoo under OnionOS — it abstracts every hardware detail behind the libretro API. [ChaiLove](https://github.com/RobLoach/chailove) is a libretro core that exposes a LÖVE2D-like API implemented in [ChaiScript](http://chaiscript.com/). Games are written in ChaiScript (a C++-like scripting language) and packaged as `.chailove` zip files.

The key insight: RetroArch owns the display, audio, and input. The core never touches SDL2 or the framebuffer directly. The mmiyoo initialization problem is completely bypassed.

Additional advantages:
- No cross-compilation. ChaiScript is interpreted at runtime.
- No glibc version targeting.
- No linker stubs or library path gymnastics.
- Instant deployment: zip the `.chai` files, SCP to the device.

### What happened

A hello-world test (`chailove/test/main.chai`) — a red rectangle with white text — rendered correctly on the first attempt. Input events arrived correctly mapped to the D-pad and face buttons.

**Status: SUCCESS.** Full game ports are proceeding in `chailove/`.

---

## Artifacts Built Along the Way

Everything below is retained and functional, even though it is not the primary Miyoo path.

| Artifact | Location | Notes |
|----------|----------|-------|
| Shared Rust SDL2 rendering crate | `miyoo/retro-sdl2/` | 8 modules; valid for standard SDL2 Linux targets |
| Nano Wizards Rust SDL2 port | `miyoo/micro/` | Compiles; would work on Anbernic/Powkiddy |
| ARM SDL2 linker stub | `miyoo/sdl2-stub/` | `libSDL2.so` + `libSDL2-2.0.so.0` with correct soname |
| Miyoo test Docker container | `miyoo/test/` | Simulates parasyte runtime environment |
| OnionOS Ports installer | `miyoo/install/` | SCP deployment scripts for OnionOS |
| Shared Zig SDL2 module | `zig/common/` | C interop wrappers for SDL2 primitives |
| Nano Wizards Zig port | `zig/micro/` | 2033 lines; compiles for ARM |
| Rust tutorial | `miyoo/LEARN_RUST.md` | 2943 lines; ownership, lifetimes, async, macros |
| Zig tutorial | `zig/LEARN_ZIG.md` | 2487 lines; comptime, C interop, build system |
| ChaiLove tutorial | `chailove/LEARN_CHAILOVE.md` | In progress |

---

## Key Learnings

**The mmiyoo SDL2 driver is a black box.** Even when SDL2 reports success at every API call, nothing renders without the undocumented SigmaStar initialization sequence. The OnionOS source code and community forums document the environment variables but not the hardware handshake.

**The parasyte runtime is purpose-built, not general-purpose.** The libs at `/mnt/SDCARD/.tmp_update/lib/parasyte/` are tailored for PICO-8 and Drastic DS. Their X11 and SDL2 stubs satisfy those specific apps' link-time requirements; they are not a general-purpose user-space environment.

**Cross-compilation for embedded ARM is a solved problem — `cargo-zigbuild` works.** The glibc version targeting, soname issues, and ARM linker setup are all manageable. The harder problem is the runtime environment on the device.

**RetroArch cores are the blessed path on Miyoo.** Every display, audio, and input concern is handled by the RetroArch layer. Any language that can produce a libretro core (C, C++, Rust via `libretro-rs`, or interpreted via ChaiLove) will work.

**Hello-world first, always.** Three of the four approaches above would have been abandoned sooner with a minimal rendering test before any full game port was attempted.

**`cargo-zigbuild` is excellent.** For any future work that needs to target a specific glibc version without a Docker sysroot, `cargo-zigbuild` with `--target armv7-unknown-linux-gnueabihf.2.28` is the right tool.

---

## Recommendations

**For Miyoo Mini Plus specifically:**
- Use ChaiLove for scripted games — zero build friction, instant deployment.
- For performance-critical games, write a custom libretro core in C or Rust (via `libretro-rs`).
- Do not attempt direct SDL2 or OpenGL — the mmiyoo driver requires undocumented initialization.

**For other ARM Linux handhelds (Anbernic, Powkiddy):**
- The `miyoo/retro-sdl2/` Rust crate and `zig/micro/` port are directly usable.
- Standard SDL2 on standard Linux works as expected.

**If someone cracks the mmiyoo init sequence:**
- The `miyoo/micro/` Rust SDL2 port would work on Miyoo with minimal changes.
- The rendering quality (CRT overlay, vignette, pixel-perfect scaling) is already implemented.
