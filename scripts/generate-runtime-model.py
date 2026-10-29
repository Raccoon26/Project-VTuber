"""Reproduce the tiny CC0 synthetic ONNX fixture using only the standard library.

ONNX IR8 / opset13: Y = Relu(MatMul(X, W) + B). No training data or artwork.
This is an execution diagnostic, not character analysis or a trained rigging model.
"""
import hashlib
from pathlib import Path
import struct


def varint(n):
    result = bytearray()
    while n >= 128:
        result.append((n & 127) | 128)
        n >>= 7
    return bytes(result) + bytes([n])


def integer(field, value):
    return varint(field << 3) + varint(value)


def data(field, value):
    if isinstance(value, str):
        value = value.encode("utf-8")
    return varint((field << 3) | 2) + varint(len(value)) + value


def tensor(name, dims, values):
    return (b"".join(integer(1, d) for d in dims) + integer(2, 1)
            + data(8, name) + data(9, struct.pack("<" + "f" * len(values), *values)))


def value(name, dims):
    shape = b"".join(data(1, integer(1, d)) for d in dims)
    return data(1, name) + data(2, data(1, integer(1, 1) + data(2, shape)))


def node(op, inputs, output):
    return b"".join(data(1, x) for x in inputs) + data(2, output) + data(4, op)


def build():
    graph = (data(1, node("MatMul", ["X", "W"], "M"))
             + data(1, node("Add", ["M", "B"], "A"))
             + data(1, node("Relu", ["A"], "Y")) + data(2, "synthetic-affine-relu-v1")
             + data(5, tensor("W", [3, 2], [1, -1, 2, 0.5, -1, 2]))
             + data(5, tensor("B", [2], [0.25, -0.5]))
             + data(11, value("X", [1, 3])) + data(12, value("Y", [1, 2])))
    return integer(1, 8) + data(2, "Project-VTuber synthetic fixture") + data(7, graph) + data(8, integer(2, 13))


if __name__ == "__main__":
    root = Path(__file__).resolve().parents[1]
    target = root / "models" / "synthetic-affine-v1.onnx"
    target.parent.mkdir(exist_ok=True)
    result = build()
    if target.exists() and target.read_bytes() != result:
        raise SystemExit("Refusing to overwrite a different immutable model version")
    target.write_bytes(result)
    print(len(result), hashlib.sha256(result).hexdigest())
