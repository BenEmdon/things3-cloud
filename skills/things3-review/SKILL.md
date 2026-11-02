---
name: things3-review
description: Read-only Things 3 access via the things3 CLI. Use for day reviews, checking Today/Upcoming/Inbox/Logbook, searching for tasks, finding overdue items, or listing areas, projects, and tags. Never writes.
---

# Things 3: Review

## Purpose
Read-only access to Things 3 through the things3-cloud CLI, for day reviews,
search, and situational awareness.

## Tooling
- Binary: `things3`. Install it from this repo with `cargo install --path .`
  and authenticate once with `things3 set-auth`.
- Add `--json` for parseable output.
- Every command syncs with Things Cloud automatically, so results are live.
- Never print, copy, or commit credentials. Never run `things3 webserver`.

## Read
- Views: `things3 today` (default), `upcoming`, `anytime`, `someday`,
  `inbox`, `logbook`.
- Search: `things3 find "<query>" --json`
  - Case-insensitive substring match on titles.
  - `-d` shows notes. `-n` also searches notes. `-k` also searches
    checklist items.
  - `-T` Today view, `-I` inbox, `-s` someday, `-e` evening.
  - `-a <area>` / `-p <project>` / `-t <tag>`: filter by title substring
    or UUID prefix, repeatable.
  - `-l '<today'` for overdue deadlines. Other filters use the same date
    syntax, for example `-S "=2026-11-02"` for tasks scheduled on that date.
  - `-i` incomplete only (default), `-c` completed only, `-A` any status.
- Structure: `things3 areas --json`, `things3 projects --json`,
  `things3 tags --json`. Drill in with `things3 area <name>` and
  `things3 project <name>`.

## Operating rules
1. Read-only. Never create, edit, mark, schedule, or delete from this skill.
   Writes go through the create or enrich skills, and only with the user's
   explicit go-ahead.
2. Do not reorganize the user's areas, projects, or tags, and do not suggest
   doing so unprompted.
3. When reporting, lead with what is due or overdue, keep it scannable, and
   say how confident you are in any inference.
4. Today view blind spot: `things3 today` can omit tasks scheduled for today
   that sit in the anytime bucket. When you need a complete list for a date,
   enumerate with the scheduled-date filter instead:
   `things3 find -S "=<YYYY-MM-DD>" --json`.
5. Reconcile state before reporting a task missing. Search with an
   independent filter before you say a task is gone, and never verify a move
   by re-reading the same view that produced the original list.
