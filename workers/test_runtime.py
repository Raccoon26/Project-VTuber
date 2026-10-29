"""Synthetic transport/security/resource tests; no external service or artwork."""
import hashlib
import importlib.util
import io
import json
import os
from pathlib import Path
import tempfile
import subprocess
import unittest
from unittest.mock import patch

spec = importlib.util.spec_from_file_location("vtuber_runtime", Path(__file__).with_name("runtime.py"))
runtime = importlib.util.module_from_spec(spec)
spec.loader.exec_module(runtime)


class Response(io.BytesIO):
    status = 200
    headers = {}

    def geturl(self):
        return "https://files.pythonhosted.org/test.whl"


class Opener:
    def __init__(self, response):
        self.response = response
        self.calls = 0

    def open(self, *args, **kwargs):
        self.calls += 1
        return self.response


class RuntimeTests(unittest.TestCase):
    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory()
        self.addCleanup(self.tmp.cleanup)
        self.root = Path(self.tmp.name).resolve()
        self.payload = b"synthetic wheel transport fixture"
        self.item = {"filename": "test.whl", "url": "https://files.pythonhosted.org/test.whl",
                     "bytes": len(self.payload), "sha256": hashlib.sha256(self.payload).hexdigest()}

    def download(self, payload=None, consent=True, progress=lambda n, size: None):
        opener = Opener(Response(self.payload if payload is None else payload))
        value = runtime.download(self.item, self.root, ["files.pythonhosted.org"], consent, progress, opener)
        return value, opener

    def test_consent_before_network_and_valid_immutable_retry(self):
        opener = Opener(Response(self.payload))
        with self.assertRaisesRegex(runtime.Failure, "download_consent"):
            runtime.download(self.item, self.root, ["files.pythonhosted.org"], False, lambda *_: None, opener)
        self.assertEqual(opener.calls, 0)
        path, _ = self.download()
        self.assertEqual(path.read_bytes(), self.payload)
        _, cached = self.download()
        self.assertEqual(cached.calls, 0)
        self.assertFalse(list(self.root.glob("*.partial")))

    def test_bad_hash_short_oversized_and_unexpected_length(self):
        for payload, code in [(b"x" * len(self.payload), "model_hash"), (b"x", "model_size"),
                              (self.payload + b"x", "model_size")]:
            with self.assertRaisesRegex(runtime.Failure, code):
                self.download(payload)
            self.assertFalse((self.root / "test.whl").exists())
        response = Response(self.payload)
        response.headers = {"Content-Length": "9999"}
        with self.assertRaisesRegex(runtime.Failure, "model_size"):
            runtime.download(self.item, self.root, ["files.pythonhosted.org"], True, lambda *_: None, Opener(response))
        self.assertGreaterEqual(len(list(self.root.glob("*.partial"))), 3)

    def test_cancel_disconnect_and_retry_preserve_partial(self):
        def cancel(*args):
            raise runtime.Failure("cancelled")
        with self.assertRaisesRegex(runtime.Failure, "cancelled"):
            self.download(progress=cancel)
        partials = {p.name: p.read_bytes() for p in self.root.glob("*.partial")}
        self.download()
        self.assertEqual(partials, {p.name: p.read_bytes() for p in self.root.glob("*.partial")})
        class Broken(Response):
            def read(self, *args):
                raise ConnectionResetError()
        other = {**self.item, "filename": "second.whl"}
        with self.assertRaises(ConnectionResetError):
            runtime.download(other, self.root, ["files.pythonhosted.org"], True, lambda *_: None, Opener(Broken(b"")))
        self.assertFalse((self.root / "second.whl").exists())

    def test_https_redirect_credentials_and_path_boundaries(self):
        for url in ["http://files.pythonhosted.org/a", "https://evil.example/a",
                    "https://files.pythonhosted.org.evil.example/a", "https://user@files.pythonhosted.org/a",
                    "https://files.pythonhosted.org:444/a", "https://files.pythonhosted.org/a?secret=1"]:
            with self.assertRaises(runtime.Failure):
                runtime.check_url(url, ["files.pythonhosted.org"])
            with self.assertRaises(runtime.Failure):
                runtime.SafeRedirect(["files.pythonhosted.org"]).redirect_request(None, None, 302, "", {}, url)
        self.item["filename"] = "../escape.whl"
        with self.assertRaisesRegex(runtime.Failure, "runtime_manifest"):
            self.download()
        with self.assertRaisesRegex(runtime.Failure, "unsafe_path"):
            runtime.plain(Path("relative"))

    def test_disk_ram_backend_oom_preflight_is_fail_closed(self):
        with patch.object(runtime.shutil, "disk_usage", return_value=type("Usage", (), {"free": 0})()):
            with self.assertRaisesRegex(runtime.Failure, "disk_full"):
                self.download()
        for kwargs, code in [({"ram": 0, "disk": 10**10}, "ram_insufficient"),
                             ({"ram": 10**10, "disk": 0}, "disk_full"),
                             ({"backend": "cuda_offload", "ram": 10**10, "disk": 10**10}, "backend_unverified")]:
            with self.assertRaisesRegex(runtime.Failure, code):
                runtime.preflight(self.root, **kwargs)
        self.assertEqual(runtime.preflight(self.root, ram=10**10, disk=10**10)["backend"], "onnx_cpu")

    def test_pending_install_never_active_and_unknown_receipt_rejected(self):
        (self.root / "environments").mkdir()
        (self.root / "environments" / "abandoned").mkdir()
        with self.assertRaisesRegex(runtime.Failure, "runtime_not_installed"):
            runtime.environment(self.root, "a" * 64)
        (self.root / "active").mkdir()
        (self.root / "active" / "00000001.json").write_text(json.dumps({"generation": "../bad", "lock_hash": "a" * 64, "receipt_hash": "b" * 64}))
        with self.assertRaisesRegex(runtime.Failure, "runtime_integrity"):
            runtime.environment(self.root, "a" * 64)

    def test_publish_never_overwrites_previous_and_inventory_detects_change(self):
        record = self.root / "record.json"
        runtime.publish(record, {"value": 1})
        before = record.read_bytes()
        with self.assertRaises(FileExistsError):
            runtime.publish(record, {"value": 2})
        self.assertEqual(record.read_bytes(), before)
        original = runtime.file_inventory(self.root)
        record.write_text("tampered", encoding="utf-8")
        self.assertNotEqual(runtime.file_inventory(self.root), original)

    def test_corrupt_cache_is_retained_and_repaired_only_after_consent(self):
        target = self.root / self.item["filename"]
        target.write_bytes(b"corrupt")
        with self.assertRaisesRegex(runtime.Failure, "download_consent"):
            self.download(consent=False)
        self.assertEqual(target.read_bytes(), b"corrupt")
        self.download()
        self.assertEqual(target.read_bytes(), self.payload)
        self.assertEqual([p.read_bytes() for p in self.root.glob("*.rejected")], [b"corrupt"])

    def test_environment_model_and_dependency_tampering_blocks_execution(self):
        generation = self.root / "environments" / ("a" * 32)
        env = generation / "venv"
        env.mkdir(parents=True)
        (env / "test.py").write_bytes(b"trusted code")
        model = Path(__file__).resolve().parents[1] / "models/synthetic-affine-v1.onnx"
        (generation / "model.onnx").write_bytes(model.read_bytes())
        runtime.publish(generation / "receipt.json", {"files": runtime.file_inventory(env)})
        (self.root / "active").mkdir()
        runtime.publish(self.root / "active/00000001.json", {"generation": "a" * 32, "lock_hash": "b" * 64,
                        "receipt_hash": runtime.digest((generation / "receipt.json").read_bytes())})
        runtime.environment(self.root, "b" * 64)
        (generation / "model.onnx").write_bytes(b"not a model")
        with self.assertRaisesRegex(runtime.Failure, "model_hash"):
            runtime.environment(self.root, "b" * 64)
        (generation / "model.onnx").write_bytes(model.read_bytes())
        (env / "test.py").write_bytes(b"untrusted code")
        with self.assertRaisesRegex(runtime.Failure, "runtime_integrity"):
            runtime.environment(self.root, "b" * 64)

    def test_real_onnx_varied_inputs_and_python_network_denial(self):
        root = Path(os.environ["VTUBER_RUNTIME_DIR"])
        lock = Path(__file__).with_name("runtime-lock.json").read_bytes()
        python, model = runtime.environment(root, runtime.digest(lock))
        source = Path(__file__).with_name("runtime.py").resolve()
        code = f'''import importlib.util,json,socket
from pathlib import Path
s=importlib.util.spec_from_file_location('tested', {str(source)!r})
r=importlib.util.module_from_spec(s);s.loader.exec_module(r)
outputs=[r.infer(Path({str(model)!r}),x)['output'] for x in [[1,2,3],[0,0,0],[-1,2,-3]]]
denied=False
try: socket.create_connection(('127.0.0.1',9),timeout=0.1)
except r.Failure: denied=True
print(json.dumps(dict(outputs=outputs,network_denied=denied)))'''
        result = subprocess.run([str(python), "-I", "-B", "-c", code], capture_output=True, timeout=30,
                                env=runtime.child_env(self.root))
        self.assertEqual(result.returncode, 0, result.stderr.decode(errors="replace"))
        value = json.loads(result.stdout)
        self.assertEqual(value["outputs"], [[2.25, 5.5], [0.25, 0.0], [6.25, 0.0]])
        self.assertTrue(value["network_denied"])


if __name__ == "__main__":
    unittest.main()
