# Project VTuber AI development

- Product: local **authoring** desktop application, not a live broadcast tracker. Professional automatic rigging is a goal, not an achieved claim.
- Preserve user changes and source bytes. Never silently resize/recompress originals, overwrite master data, or certify untested exporters.
- This user's workspace is `E:\Project-VTuber`. New project downloads, caches, temporary files, builds, and app data must stay on E. Dot-source `scripts/env.ps1` before commands. Existing installed tools on other drives may be read; do not relocate/delete user installations without authorization.
- Start every DEV by reading `docs/PROJECT_STATUS.md`, `docs/REQUIREMENTS.md`, `docs/FAILURE_REGISTER.md`, and the relevant roadmap entry. Check branch/status before edits.
- Keep Python/model-specific code behind versioned adapters. No paid API, heavy model download, sustained GPU workload, or user-asset upload without applicable authorization.
- Run `scripts/test.ps1` with a working Python 3.12 path and the appropriate real-app checks. Record commands, logs, failures, not-run tests, and exact evidence scope in `docs/TEST_RUNS.md`.
- Failure process: reproduce → collect evidence → narrow hypotheses → minimal fix → regression → related checks → failure register. Do not repeatedly change versions without evidence.
- Keep requirements JSON/Markdown, status, ADRs, and the DEV roadmap consistent. Planned/Implemented/Verified and pass/fail/unverified/not_applicable have different meanings.
- Current user instruction authorizes per-DEV commit/push/PR to main and merge only after tests, CI, protection/review requirements and conflict checks pass. Never force merge, bypass required reviews, force push, or hard reset. Verify remote main and report PR/commit/merge results.
