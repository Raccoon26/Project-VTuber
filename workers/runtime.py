"""Trusted DEV-03 bootstrap / provider. JSONL v2, no arbitrary provider code.

The bootstrap may GET pinned HTTPS wheels only after consent. The inference
subprocess has no project path, disables ORT telemetry and denies Python socket
operations. This is NOT an OS network/filesystem sandbox.
"""
import ctypes
import hashlib
import json
import math
import os
from pathlib import Path
import platform
import re
import shutil
import stat
import subprocess
import sys
import time
import urllib.error
import urllib.parse
import urllib.request
import uuid
import venv

MAX_JSON = 65536
MAX_DOWNLOAD = 64 * 1024 * 1024
PROVIDER = "onnx-synthetic-v1"
MODEL_HASH = "56f731422c39f2576a74604122b4ddb14bb4d86869e0011750e17b1196deb765"


class Failure(Exception):
    pass


def require(ok, code):
    if not ok:
        raise Failure(code)


def digest(data):
    return hashlib.sha256(data).hexdigest()


def plain(path):
    path = Path(path)
    require(path.is_absolute(), "unsafe_path")
    for item in [*reversed(path.parents), path]:
        try:
            info = item.lstat()
        except FileNotFoundError:
            continue
        require(not stat.S_ISLNK(info.st_mode) and not getattr(info, "st_file_attributes", 0) & 0x400,
                "unsafe_path")
    return path


def read_bounded(path, limit):
    plain(path)
    with open(path, "rb") as handle:
        require(os.fstat(handle.fileno()).st_size <= limit, "runtime_integrity")
        result = handle.read(limit + 1)
    require(len(result) <= limit, "runtime_integrity")
    return result


def read_json(path):
    return json.loads(read_bounded(path, MAX_JSON))


def publish(path, value):
    plain(path.parent)
    temp = path.with_suffix("." + uuid.uuid4().hex + ".pending")
    with open(temp, "xb") as out:
        out.write(json.dumps(value, allow_nan=False, sort_keys=True).encode("utf-8"))
        out.flush()
        os.fsync(out.fileno())
    os.link(temp, path)  # atomic, no overwrite
    temp.unlink()


def check_url(url, hosts):
    parsed = urllib.parse.urlsplit(url)
    require(parsed.scheme == "https" and parsed.hostname in hosts
            and parsed.port in (None, 443) and not parsed.username and not parsed.password
            and not parsed.fragment and not parsed.query, "download_origin")


class SafeRedirect(urllib.request.HTTPRedirectHandler):
    def __init__(self, hosts):
        self.hosts = hosts

    def redirect_request(self, req, fp, code, msg, headers, newurl):
        check_url(newurl, self.hosts)
        return super().redirect_request(req, fp, code, msg, headers, newurl)


def verify_file(path, item):
    payload = read_bounded(path, item["bytes"])
    require(len(payload) == item["bytes"] and digest(payload) == item["sha256"], "model_hash")
    return payload


def download(item, directory, hosts, consent, progress, opener=None):
    require(consent is True, "download_consent")
    check_url(item["url"], hosts)
    require(re.fullmatch(r"[A-Za-z0-9_.-]+\.(whl|onnx|safetensors)", item["filename"]) is not None
            and 0 < item["bytes"] <= MAX_DOWNLOAD
            and re.fullmatch(r"[a-f0-9]{64}", item["sha256"]) is not None, "runtime_manifest")
    plain(directory)
    target = directory / item["filename"]
    if target.exists():
        try:
            verify_file(target, item)
            return target
        except Failure as error:
            if str(error) not in ("model_hash", "runtime_integrity"):
                raise
            # Retain a corrupt cache file, then retry only under this consent.
            target.rename(directory / (uuid.uuid4().hex + ".rejected"))
    require(shutil.disk_usage(directory).free >= item["bytes"] + 32 * 1024 * 1024, "disk_full")
    partial = directory / (uuid.uuid4().hex + ".partial")
    opener = opener or urllib.request.build_opener(urllib.request.ProxyHandler({}), SafeRedirect(hosts))
    total = 0
    sha = hashlib.sha256()
    started = time.monotonic()
    with opener.open(item["url"], timeout=15) as response, open(partial, "xb") as out:
        check_url(response.geturl(), hosts)
        require(response.status == 200, "download_failed")
        size = response.headers.get("Content-Length")
        require(size is None or size == str(item["bytes"]), "model_size")
        require(response.headers.get("Content-Encoding", "identity") == "identity", "download_failed")
        while True:
            require(time.monotonic() - started < 180, "worker_timeout")
            block = response.read(min(65536, item["bytes"] - total + 1))
            if not block:
                break
            total += len(block)
            require(total <= item["bytes"], "model_size")
            out.write(block)
            sha.update(block)
            progress(total, item["bytes"])
        out.flush()
        os.fsync(out.fileno())
    require(total == item["bytes"], "model_size")
    require(sha.hexdigest() == item["sha256"], "model_hash")
    os.link(partial, target)
    partial.unlink()
    return target


def available_ram():
    if sys.platform != "win32":
        return None
    class Memory(ctypes.Structure):
        _fields_ = [("length", ctypes.c_ulong), ("load", ctypes.c_ulong)] + [
            (name, ctypes.c_ulonglong) for name in
            ("total", "available", "page_total", "page_available", "virtual_total", "virtual_available", "extended")]
    info = Memory()
    info.length = ctypes.sizeof(info)
    require(ctypes.windll.kernel32.GlobalMemoryStatusEx(ctypes.byref(info)) != 0, "resource_unknown")
    return info.available


def preflight(root, backend="onnx_cpu", ram=None, disk=None, vram=0):
    require(backend == "onnx_cpu", "backend_unverified")
    ram = available_ram() if ram is None else ram
    require(ram is not None and ram >= 512 * 1024 * 1024, "ram_insufficient")
    disk = shutil.disk_usage(root).free if disk is None else disk
    require(disk >= 512 * 1024 * 1024, "disk_full")
    return {"available_ram_bytes": ram, "free_disk_bytes": disk, "required_ram_mib": 512,
            "required_vram_mib": vram, "backend": backend}


def file_inventory(directory):
    result = {}
    for path in sorted(directory.rglob("*")):
        plain(path)
        if path.is_file():
            require(path.stat().st_size < 256 * 1024 * 1024, "runtime_integrity")
            result[path.relative_to(directory).as_posix()] = digest(path.read_bytes())
            require(len(result) <= 10000, "runtime_integrity")
    return result


def environment(root, lock_hash):
    active = root / "active"
    plain(active)
    records = sorted(active.glob("*.json")) if active.exists() else []
    require(len(records) <= 128, "runtime_integrity")
    if not records:
        raise Failure("runtime_not_installed")
    record = read_json(records[-1])
    require(set(record) == {"generation", "receipt_hash", "lock_hash"}
            and record["lock_hash"] == lock_hash
            and re.fullmatch(r"[a-f0-9]{32}", record["generation"]) is not None, "runtime_integrity")
    generation = root / "environments" / record["generation"]
    receipt_bytes = read_bounded(generation / "receipt.json", 4 * 1024 * 1024)
    require(digest(receipt_bytes) == record["receipt_hash"], "runtime_integrity")
    receipt = json.loads(receipt_bytes)
    env = generation / "venv"
    require(receipt["files"] == file_inventory(env), "runtime_integrity")
    model = generation / "model.onnx"
    require(digest(read_bounded(model, 216)) == MODEL_HASH, "model_hash")
    return env / "Scripts" / "python.exe", model


def child_env(root):
    env = {k: v for k, v in os.environ.items() if k.upper() in
           {"SYSTEMROOT", "WINDIR", "PATH", "COMSPEC", "PATHEXT", "SYSTEMDRIVE"}}
    temp = plain(root / "tmp")
    temp.mkdir(exist_ok=True)
    env.update(TEMP=str(temp), TMP=str(temp), PYTHONDONTWRITEBYTECODE="1",
               PIP_DISABLE_PIP_VERSION_CHECK="1", PIP_NO_INDEX="1", PIP_CONFIG_FILE=os.devnull,
               PIP_CACHE_DIR=str(temp), OMP_NUM_THREADS="1", OPENBLAS_NUM_THREADS="1",
               HF_HUB_OFFLINE="1", TRANSFORMERS_OFFLINE="1")
    return env


def run_child(command, root, timeout=90):
    result = subprocess.run(command, stdin=subprocess.DEVNULL, stdout=subprocess.PIPE,
                            stderr=subprocess.DEVNULL, timeout=timeout, env=child_env(root),
                            creationflags=0x08000000 if sys.platform == "win32" else 0)
    require(result.returncode == 0, "runtime_setup_failed")
    require(len(result.stdout) <= MAX_JSON, "worker_protocol")
    return result.stdout


def install(request, emit):
    require(request.get("consent") is True, "download_consent")
    require(sys.version_info[:2] == (3, 12) and sys.platform == "win32"
            and platform.machine().lower() in ("amd64", "x86_64"), "runtime_platform")
    root = plain(Path(request["root"]))
    root.mkdir(parents=True, exist_ok=True)
    preflight(root)
    lock = request["lock"]
    require(lock["version"] == 1 and lock["id"] == "onnx-cpu-win-py312-v1"
            and lock["allowed_hosts"] == ["files.pythonhosted.org"], "runtime_manifest")
    for name in ("downloads", "environments", "active"):
        plain(root / name).mkdir(exist_ok=True)
    files = lock["files"]
    require(len(files) == 5, "runtime_manifest")
    for i, item in enumerate(files):
        last = [-1]
        def progress(received, total):
            percent = 5 + int(50 * (i + received / total) / len(files))
            if percent > last[0]:
                emit(percent)
                last[0] = percent
        download(item, root / "downloads", lock["allowed_hosts"], True, progress)
        emit(5 + (i + 1) * 10)
    generation_id = uuid.uuid4().hex
    generation = root / "environments" / generation_id
    generation.mkdir()
    env = generation / "venv"
    # No global packages, system-site-packages or arbitrary source distributions.
    venv.EnvBuilder(with_pip=True, symlinks=False).create(env)
    emit(65)
    requirements = generation / "wheels.txt"
    requirements.write_text("\n".join(f'{x["name"]}=={x["version"]} --hash=sha256:{x["sha256"]}' for x in files), encoding="utf-8")
    python = env / "Scripts" / "python.exe"
    run_child([str(python), "-I", "-B", "-m", "pip", "--isolated", "install", "--no-index",
               "--no-compile", "--no-deps", "--only-binary=:all:", "--require-hashes",
               "--find-links", str(root / "downloads"), "-r", str(requirements), "--quiet"], root)
    emit(80)
    model = bytes.fromhex(request["model_hex"])
    require(len(model) == 216 and digest(model) == MODEL_HASH, "model_hash")
    with open(generation / "model.onnx", "xb") as model_file:
        model_file.write(model)
        model_file.flush()
        os.fsync(model_file.fileno())
    # Quarantined subprocess load and actual operator test before publication.
    result = json.loads(run_child([str(python), "-I", "-B", str(Path(__file__).resolve()),
                                  "--smoke", str(generation / "model.onnx")], root))
    require(result["output"] == [2.25, 5.5], "inference_mismatch")
    emit(90)
    publish(generation / "receipt.json", {"version": 1, "lock_hash": request["lock_hash"],
                                         "files": file_inventory(env), "probe": result})
    receipt_hash = digest(read_bounded(generation / "receipt.json", 4 * 1024 * 1024))
    records = sorted((root / "active").glob("*.json"))
    require(len(records) < 128, "runtime_integrity")
    number = int(records[-1].stem) + 1 if records else 1
    publish(root / "active" / f"{number:08}.json", {"generation": generation_id,
            "lock_hash": request["lock_hash"], "receipt_hash": receipt_hash})
    environment(root, request["lock_hash"])
    return {"state": "runnable", "backend": "onnx_cpu", "probe": result}


def infer(model, inputs):
    require(digest(read_bounded(model, 216)) == MODEL_HASH, "model_hash")
    require(len(inputs) == 3 and all(type(x) in (int, float) and math.isfinite(x) and abs(x) <= 100 for x in inputs), "runtime_input")
    # Cache the trusted stdlib OS query before denying child execution. On some
    # Windows installations platform.uname falls back to the local `ver` command.
    platform.uname()
    def audit(event, args):
        # gethostname is a local OS query used by platform.system(), not traffic.
        if (event.startswith("socket.") and event != "socket.gethostname") or event in ("subprocess.Popen", "os.system"):
            raise Failure("network_denied")
    sys.addaudithook(audit)
    import numpy as np
    import onnxruntime as ort
    require(ort.__version__ == "1.31.1" and np.__version__ == "2.5.4", "runtime_integrity")
    ort.disable_telemetry_events()
    options = ort.SessionOptions()
    options.intra_op_num_threads = 1
    options.inter_op_num_threads = 1
    options.enable_mem_pattern = False
    options.execution_mode = ort.ExecutionMode.ORT_SEQUENTIAL
    started = time.perf_counter()
    session = ort.InferenceSession(model.read_bytes(), sess_options=options, providers=["CPUExecutionProvider"])
    session.disable_fallback()
    require(session.get_providers() == ["CPUExecutionProvider"], "backend_unverified")
    loaded = time.perf_counter()
    output = session.run(["Y"], {"X": np.array([inputs], dtype=np.float32)})[0]
    ended = time.perf_counter()
    expected = [max(0, inputs[0] + 2 * inputs[1] - inputs[2] + 0.25),
                max(0, -inputs[0] + 0.5 * inputs[1] + 2 * inputs[2] - 0.5)]
    require(output.shape == (1, 2) and np.isfinite(output).all()
            and np.allclose(output[0], expected, rtol=1e-6, atol=1e-5), "inference_mismatch")
    result = {"output": output[0].tolist(), "backend": "onnx_cpu", "device": "CPU",
              "load_ms": (loaded - started) * 1000, "inference_ms": (ended - loaded) * 1000,
              "cpu_seconds": time.process_time(), "peak_ram_bytes": peak_ram(), "model_hash": MODEL_HASH}
    del session
    return result


def peak_ram():
    if sys.platform != "win32":
        return None
    class Counters(ctypes.Structure):
        _fields_ = [("cb", ctypes.c_ulong), ("faults", ctypes.c_ulong)] + [(n, ctypes.c_size_t) for n in
                    ("peak", "working", "quota_peak_paged", "quota_paged", "quota_peak_nonpaged", "quota_nonpaged", "pagefile", "peak_pagefile")]
    counters = Counters()
    counters.cb = ctypes.sizeof(counters)
    kernel = ctypes.windll.kernel32
    kernel.GetCurrentProcess.restype = ctypes.c_void_p
    api = ctypes.windll.psapi.GetProcessMemoryInfo
    api.argtypes = [ctypes.c_void_p, ctypes.c_void_p, ctypes.c_ulong]
    require(api(kernel.GetCurrentProcess(), ctypes.byref(counters), counters.cb) != 0, "resource_unknown")
    return counters.peak


def main():
    request = json.loads(sys.stdin.buffer.readline(MAX_JSON + 1))
    require(request.get("protocol") == 2 and request.get("provider") == PROVIDER, "worker_protocol")
    job = request["job_id"]
    require(str(uuid.UUID(job)) == job, "worker_protocol")
    def event(kind, progress, data=None):
        print(json.dumps({"protocol": 2, "job_id": job, "provider": PROVIDER,
                          "kind": kind, "progress": progress, "data": data}, allow_nan=False), flush=True)
    try:
        event("progress", 0)
        action = request["action"]
        root = plain(Path(request["root"]))
        if action == "install":
            result = install(request, lambda p: event("progress", p))
        elif action == "probe":
            resources = preflight(root)
            python, model = environment(root, request["lock_hash"])
            result = json.loads(run_child([str(python), "-I", "-B", str(Path(__file__).resolve()), "--smoke", str(model)], root))
            result["resources"] = resources
        elif action == "infer":
            preflight(root)
            python, model = environment(root, request["lock_hash"])
            require(request["input"] == [1.0, 2.0, 3.0], "runtime_input")
            event("progress", 25)
            # The base interpreter is already inside the OS Job Object before it
            # receives stdin. Launch venv redirectors only from that owned process.
            result = json.loads(run_child([str(python), "-I", "-B", str(Path(__file__).resolve()),
                                          "--smoke", str(model)], root))
            event("progress", 80)
            # Fixed filename only; caller supplies a fresh private staging directory.
            stage = plain(Path(request["stage"]))
            artifact = {"version": 1, "kind": "runtime_diagnostic", "job_id": job,
                        "provider": PROVIDER, "model_version": "1.0.0", "model_hash": MODEL_HASH,
                        "source_hash": request.get("source_hash"), "input": request["input"],
                        "preprocessing": "none", "seed": None, "uncertainty": "unknown", **result}
            publish(stage / "result.json", artifact)
            result = {"artifact": "result.json", "sha256": digest(read_bounded(stage / "result.json", MAX_JSON))}
        else:
            raise Failure("worker_protocol")
        event("result", 100, result)
        event("completed", 100)
    except Failure as error:
        event("error", 0, {"code": str(error)})
        raise SystemExit(1)
    except MemoryError:
        event("error", 0, {"code": "ram_insufficient"})
        raise SystemExit(1)
    except urllib.error.URLError:
        event("error", 0, {"code": "download_failed"})
        raise SystemExit(1)
    except OSError as error:
        event("error", 0, {"code": "disk_full" if error.errno == 28 or getattr(error, "winerror", None) == 112 else "runtime_io"})
        raise SystemExit(1)
    except Exception:
        event("error", 0, {"code": "runtime_failed"})
        raise SystemExit(1)


if __name__ == "__main__":
    if len(sys.argv) == 3 and sys.argv[1] == "--smoke":
        print(json.dumps(infer(Path(sys.argv[2]), [1, 2, 3]), allow_nan=False))
    else:
        main()
