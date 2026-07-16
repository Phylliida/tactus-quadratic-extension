# Task board — verified 2D CAD constraint satisfaction

This directory is a simple task board (same conventions as `tactus/board/` and
`monotile/board/`). **One markdown file = one task.** Add, claim, and finish
tasks just by creating and editing these `.md` files with normal file tools —
no server, no JSON.

**The program:** a formally verified 2D sketch constraint-satisfaction system
on tactus (Lean backend). Master plan = `../DESIGN.md` (v0.1). The shape:

> An untrusted Rust solver (Newton + cluster decomposition) emits a
> `SketchCertificate` — a tower of simple real extensions mirroring the
> sketch's decomposition plan, plus coordinates and branch signs. A verified
> checker validates it: equality constraints by D5 zero-tests in the tower,
> sign conditions by Sturm–Tarski queries with an interval fast path. If the
> checker accepts, every constraint is exactly satisfied — the theorem is
> stated against the ported `constraint_satisfied<T: OrderedField>` spec.

Cards `cad-01..03` finish M0 (polynomial layer in `../../tactus-algebra`).
Cards `cad-04..06` are the tower + D5 (M1–M2). Cards `cad-07..09` are the
ordering mountain (M3). Cards `cad-10..11` are the equality-fragment checker
(M4, needs only M2 — can run parallel to M3). Cards `cad-12..13` are signs,
fast path, solver, and the end-to-end demo (M5–M6). Card `cad-14` is deferred
theory debt.

Verification gates: crate-local `./check.sh` (Lean backend) in both
`tactus-algebra` and `tactus-quadratic-extension`; every card lands at
**0 errors** with no `assume`/`admit`/`external_body`. Don't pin "N verified"
counts in docs — they drift; assert on 0 errors.

## File format

    ---
    title: short title of the task
    status: todo            # todo | in_progress | done
    claimed_by:             # your sibling id, or a name (optional)
    created: <iso8601>
    updated: <iso8601>
    ---

    ## Description
    what the task is / what "done" looks like

    ## Progress
    - (timestamp) a running log of what you tried / found

    ## Writeup
    (fill this in when done: findings, how the code works, and any assumptions
     you made — this is what the human reads to understand what happened)

## Workflow

- **Pick a task:** open a `status: todo` file, set `status: in_progress`, and
  put your id in `claimed_by`. Prefer a task nobody else has claimed, and
  respect `Blocked by:` lines.
- **Make a new task:** create `board/<slug>.md` with `status: todo`. Break big
  work into small, checkable tasks.
- **Log progress:** append to `## Progress` as you go.
- **Finish:** set `status: done` and fill in `## Writeup`. Be honest about
  what's partial or unverified.

Files starting with `.` or `_`, plus this README, are ignored by the board.
