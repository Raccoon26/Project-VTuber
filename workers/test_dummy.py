import io
import json
import unittest
from dummy import run


class WorkerTest(unittest.TestCase):
    def test_protocol_and_progress(self):
        out = io.StringIO()
        code = run(io.StringIO('{"protocol":1,"job_id":"fixture","provider":"dummy"}\n'), out, lambda _: None)
        events = [json.loads(line) for line in out.getvalue().splitlines()]
        self.assertEqual(code, 0)
        self.assertEqual([e["progress"] for e in events[:-1]], [0, 20, 40, 60, 80, 100])
        self.assertEqual(events[-1]["kind"], "completed")

    def test_rejects_bad_or_oversized_requests(self):
        for raw in ('{}', 'null', '[1]', 'x' * 65537,
                    '{"protocol":2,"job_id":"x","provider":"dummy"}',
                    '{"protocol":1,"job_id":"x","provider":"cloud"}'):
            self.assertEqual(run(io.StringIO(raw), io.StringIO()), 2)


if __name__ == "__main__":
    unittest.main()
