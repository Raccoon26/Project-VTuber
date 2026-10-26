"""Synthetic test image; no user artwork or third-party content."""
from pathlib import Path
import struct
import zlib


def chunk(kind, data):
    return struct.pack('>I', len(data)) + kind + data + struct.pack('>I', zlib.crc32(kind + data) & 0xffffffff)


pixels = b''.join(b'\0' + bytes([60, 170, 210, 255]) * 64 for _ in range(64))
image = b'\x89PNG\r\n\x1a\n' + chunk(b'IHDR', struct.pack('>IIBBBBB', 64, 64, 8, 6, 0, 0, 0)) + chunk(b'IDAT', zlib.compress(pixels)) + chunk(b'IEND', b'')
dest = Path(__file__).resolve().parents[1] / 'samples/diagnostic.png'
dest.parent.mkdir(exist_ok=True)
dest.write_bytes(image)
