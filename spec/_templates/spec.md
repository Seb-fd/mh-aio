# Feature spec template

> Copy this file to `spec/features/NNN-short-name/spec.md` and fill it before writing any code.

## Context

- Problem:
- Current state (tables / routes / commands involved):
- Related docs:

## User stories

- As a [user], I want [goal] so that [benefit].

## Scope

### In scope

- [ ]

### Out of scope

- [ ]

## Acceptance criteria

- [ ] Criterion 1 (measurable, verifiable via command/query/UI step)
- [ ] Criterion 2
- [ ] `npm run check` passes with 0 errors, 0 warnings
- [ ] `cargo test --manifest-path src-tauri/Cargo.toml` passes
- [ ] `cargo build --manifest-path src-tauri/Cargo.toml` passes

## Data contracts

- Tables / columns / indexes:
- Seed idempotency (UNIQUE target for every `INSERT OR IGNORE`):
- Game-faithful ordering required (if any):

## Constraints

- Idempotent non-destructive seed (never DELETE, no `clear_game`, no count early-return).
- FK order: parents before children.
- Theming via `var(--theme-*)` only.
- English-primary UI.
