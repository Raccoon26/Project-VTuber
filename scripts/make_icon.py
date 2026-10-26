"""Generate our geometric V app mark as a deterministic Windows ICO (no external art)."""
import pathlib
import struct

size = 32
pixels = bytearray()
for y in range(size - 1, -1, -1):
    for x in range(size):
        mark = 7 <= y <= 25 and (abs(x - (8 + (y - 7) * .43)) < 2 or abs(x - (24 - (y - 7) * .43)) < 2)
        pixels.extend((139, 228, 195, 255) if mark else (34, 26, 21, 255))
dib = struct.pack('<IIIHHIIIIII', 40, size, size * 2, 1, 32, 0, len(pixels), 0, 0, 0, 0) + pixels + bytes(128)
icon = struct.pack('<HHH', 0, 1, 1) + struct.pack('<BBBBHHII', size, size, 0, 0, 1, 32, len(dib), 22) + dib
dest = pathlib.Path(__file__).resolve().parents[1] / 'src-tauri/icons/icon.ico'
dest.parent.mkdir(exist_ok=True)
dest.write_bytes(icon)
