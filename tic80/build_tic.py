#!/usr/bin/env python3
"""Build TIC-80 .tic cartridges from Lua source files.

Usage:
    python3 build_tic.py micro              # build one game
    python3 build_tic.py --all              # build all games
    python3 build_tic.py hello.lua out.tic  # build arbitrary file

The .tic binary format uses 4-byte chunk headers with bitfields:
  bits 0-4:  chunk type (5 bits)
  bits 5-7:  bank number (3 bits)
  bits 8-23: data size (16 bits, 0 = 65536)
  bits 24-31: reserved
"""

import struct
import sys
import os

SCRIPT_DIR = os.path.dirname(os.path.abspath(__file__))

GAMES = {
    "micro": "Nano Wizards",
    "space": "Neon Defender",
    "shadow": "Shadow Blade",
    "arena": "Arena Blitz",
    "dragon": "Dragon Fury",
    "mariolike": "Pixel Knight",
    "cyber": "Chrome Viper",
    "neon": "Neon Runner",
    "nova": "Nova Evader",
}


def make_chunk(chunk_type: int, bank: int, data: bytes) -> bytes:
    size = len(data) if len(data) < 65536 else 0
    word = (chunk_type & 0x1F) | ((bank & 0x7) << 5) | ((size & 0xFFFF) << 8)
    return struct.pack("<I", word) + data


def build_tic(lua_path: str, tic_path: str) -> bool:
    if not os.path.isfile(lua_path):
        print(f"  SKIP: {lua_path} not found")
        return False

    with open(lua_path, "r") as f:
        code = f.read().encode("utf-8")

    if len(code) > 65536:
        print(f"  ERROR: {lua_path} is {len(code)} bytes, exceeds 65536 limit")
        return False

    data = b""
    data += make_chunk(17, 0, b"")  # DEFAULT chunk — sets Sweetie 16 palette
    data += make_chunk(5, 0, code)  # CODE chunk

    with open(tic_path, "wb") as f:
        f.write(data)

    print(f"  {os.path.basename(tic_path)}: {os.path.getsize(tic_path)} bytes ({len(code)} code)")
    return True


def main():
    if len(sys.argv) < 2:
        print("Usage:")
        print("  python3 build_tic.py <game>        # build one game")
        print("  python3 build_tic.py --all          # build all games")
        print("  python3 build_tic.py in.lua out.tic # arbitrary file")
        print(f"\nGames: {', '.join(GAMES.keys())}")
        sys.exit(1)

    if sys.argv[1] == "--all":
        built = 0
        for game in GAMES:
            lua = os.path.join(SCRIPT_DIR, game, f"{game}.lua")
            tic = os.path.join(SCRIPT_DIR, game, f"{game}.tic")
            print(f"--- {GAMES[game]} ({game}) ---")
            if build_tic(lua, tic):
                built += 1
        print(f"\nBuilt {built}/{len(GAMES)} cartridges")

    elif sys.argv[1].endswith(".lua") and len(sys.argv) >= 3:
        build_tic(sys.argv[1], sys.argv[2])

    else:
        game = sys.argv[1]
        if game in GAMES:
            lua = os.path.join(SCRIPT_DIR, game, f"{game}.lua")
            tic = os.path.join(SCRIPT_DIR, game, f"{game}.tic")
            print(f"--- {GAMES[game]} ({game}) ---")
            build_tic(lua, tic)
        else:
            print(f"Unknown game: {game}")
            print(f"Available: {', '.join(GAMES.keys())}")
            sys.exit(1)


if __name__ == "__main__":
    main()
