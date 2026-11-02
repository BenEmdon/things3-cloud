---
name: things3-create-task
description: Create, schedule, or complete Things 3 tasks via the things3 CLI. Use when the user wants to add a todo, file a task in an area or project, set a when date or deadline, or mark a task done. Proposes the task and waits for approval before writing.
---

# Things 3: Create Task

## Purpose
Create tasks in Things 3 through the things3-cloud CLI, following the user's
filing conventions.

## Tooling
- Binary: `things3`. Install it from this repo with `cargo install --path .`
  and authenticate once with `things3 set-auth`.
- Add `--json` for parseable output.
- Every command syncs with Things Cloud automatically, so results are live.
- Never print, copy, or commit credentials. Never run `things3 webserver`.

## Create
- `things3 new "<title>" -i <area> -w today --json`
  - `-i, --in`: container. Use an area or project UUID, or a UUID prefix,
    from `things3 areas --json` / `things3 projects --json`. Area titles do
    not work here. Also accepts `inbox` or `clear`.
  - `-w, --when`: `today`, `evening`, `anytime`, `someday`, or `YYYY-MM-DD`.
  - `-d, --deadline YYYY-MM-DD`: deadline date.
  - `-t, --tags`: comma-separated tag titles.
  - `-n, --notes`: notes text.
- Schedule or reschedule an existing task with
  `things3 schedule <id> -w today -d YYYY-MM-DD`. Use `-D` to clear a
  deadline.
- Complete a task with `things3 mark <id> -d`. Use `-i` to mark incomplete
  and `-c` to mark canceled.

## Operating rules
1. Propose the task first: show the exact title, area or project, and date,
   and create only after the user says go.
2. Default to `-w today` so a new task lands in the Today view, unless the
   user names another date.
3. File under the right area or project, by content. Get the UUID from
   `areas --json` or `projects --json` at the time you create the task, and
   ask if the filing is ambiguous.
4. Report what was created in the same reply: title, area or project, when,
   and deadline if set.
5. Never delete tasks, and never mark tasks done, on your own initiative.
6. Never trust exit status or stdout alone. A create can print an error and
   still exit 0, and a successful create can print output that does not
   parse. Verify every write by reading it back with `find`, and check with
   `find` before retrying, so a blind retry does not create a duplicate.
7. If a write fails with an auth error, do not tell the user their password
   changed. Reads can still work while writes fail. Retry the write once,
   verify with `find`, and if it still fails report it as a write auth
   failure to investigate.
