"""Protocol v1 reference worker. No network, project access, model loading or AI."""
import json
import sys
import time

MAX_LINE = 65536


def run(inp, out, sleep=time.sleep):
    raw = inp.readline(MAX_LINE + 1)
    if len(raw) > MAX_LINE:
        return 2
    try:
        request = json.loads(raw)
        if set(request) != {"protocol", "job_id", "provider"}:
            return 2
        if request["protocol"] != 1 or request["provider"] != "dummy":
            return 2
        job_id = request["job_id"]
        if not isinstance(job_id, str) or not 1 <= len(job_id) <= 64:
            return 2
    except (ValueError, TypeError):
        return 2
    for progress in (0, 20, 40, 60, 80, 100):
        out.write(json.dumps({"protocol": 1, "job_id": job_id,
                              "kind": "progress", "progress": progress}) + "\n")
        out.flush()
        sleep(0.3)
    out.write(json.dumps({"protocol": 1, "job_id": job_id,
                          "kind": "completed", "progress": 100}) + "\n")
    out.flush()
    return 0


if __name__ == "__main__":
    sys.exit(run(sys.stdin, sys.stdout))
