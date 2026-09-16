# Spec-Driven Development (SDD) — mh-aio

Level: **spec-anchored**. The spec stays alive: every significant change updates the spec first, then the code.

Canonical project docs remain at the repo root: `AGENTS.md` (agent rules), `roadmap.md` (phases), `STATUS.md` (current status), `docs/fidelity-report.md` (data audit). This `spec/` folder is the persistent brain / anchor of truth for AI-assisted work. If `spec/` and root docs disagree, flag it and fix the spec first.

## The SDD cycle

1. **Constitution (once per project):** `spec/constitution/` — general rules.
2. **Specify:** `features/NNN-name/spec.md` — what to build + acceptance criteria.
3. **Plan:** `features/NNN-name/plan.md` — how to build (approach, files, data).
4. **Tasks:** `features/NNN-name/tasks.md` — small verifiable checklist.
5. **Implement:** agent executes tasks one by one.
6. **Verify:** validate against acceptance criteria. If it fails, adjust spec or code and repeat.

```text
Specification (what) => Plan (how) => Tasks (steps, repeat) => Implementation (execute) => Verification (validate)
```

## Multi-agent flow

Used to avoid overflowing the main context (Separation of Concerns):

- **Coordinator:** does not write code. Reads the task, splits it, decides order.
- **Implementer:** receives one concrete subtask and executes it. Several may run in parallel, each on its own part.
- **Verifier:** reviews implementer output. Runs tests, checks acceptance criteria.

In this environment map to: main agent = Coordinator, `Task` subagents = Implementers (research / implement), verification via commands below = Verifier. Subagents return only a summary to the Coordinator.

## Loop engineering

Design autonomous feedback loops: the agent iterates alone until exit conditions hold.

```text
Act => Observe => Fix (repeat until criteria pass)
```

Example: "Fix all tests" → run tests, see failures, fix, re-run until green.

Order of maturity: Prompt engineering (what you ask) → Context engineering (what it knows) → Harness engineering (its environment) → Loop engineering (how it iterates).

Golden rule: the AI executes, but **you** own the project. Power without control is useless.

## Verification gate (mandatory)

```bash
npm run check
cargo test --manifest-path src-tauri/Cargo.toml
cargo build --manifest-path src-tauri/Cargo.toml
```

Plus: check `.github/workflows/*.yml` for extra jobs before any commit/push. Default is NO git writes (no commit/push/tag/PR unless explicitly requested).

## How to add a feature

1. Copy `spec/_templates/spec.md`, `plan.md`, `tasks.md` into `spec/features/NNN-short-name/`.
2. Fill `spec.md` first, get human validation of intent.
3. Fill `plan.md`, then `tasks.md`.
4. Implement task by task, verifying against `spec.md`.
