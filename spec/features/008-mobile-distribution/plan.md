# 008 — Plan — Mobile & distribution

## Approach

Phase incrementally: (a) document + smoke-test Android dev on arch-matched emulator; (b) wire updater + tray behind flags; (c) align release packaging with `release.yml`. Follow `AGENTS.md` Mobile Support + Release & Versioning exactly.

## Files to touch

- Config: `src-tauri/tauri.conf.json`, `src-tauri/capabilities/default.json` (minimal permission additions only), `vite.config.ts` HMR (`TAURI_DEV_HOST`) if needed.
- CI: `.github/workflows/release.yml` (read first; change only if spec-approved).
- Scripts: `scripts/check-version.js` (already exists; wire into CI gate).
- Docs: `STATUS.md`, this spec.

## Data / migrations

- None.

## Verification

```bash
npm run check
cargo test --manifest-path src-tauri/Cargo.toml
cargo build --manifest-path src-tauri/Cargo.toml
npm run version:check
```

- Android emulator arch check (`x86_64` vs `aarch64` — avoid SIGILL); `npx tauri android dev -t <arch>` smoke only.
