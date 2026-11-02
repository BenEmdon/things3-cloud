# Things 3: Enrich Task

## Purpose
Append useful context, such as research, links, or key facts, to a task's
notes, as a way to communicate on the task itself.

## Tooling
- Binary: `things3`. Install it from this repo with `cargo install --path .`
  and authenticate once with `things3 set-auth`.
- Add `--json` for parseable output.
- Every command syncs with Things Cloud automatically, so results are live.
- Never print, copy, or commit credentials. Never run `things3 webserver`.

## Edit notes
- `things3 edit` replaces notes with `-n`. It does not append. Always read
  first:
  1. `things3 find "<query>" --json` to get the task id and current notes.
  2. New notes = existing notes + separator + your addition.
  3. `things3 edit <id> -n "<combined>"` (single task only).
- Other edits: `things3 edit <id> -t "<new title>"` to retitle,
  `things3 edit <id> -m <area-or-project>` to move, plus tag and checklist
  flags. See `things3 edit --help`.

## Operating rules
1. Append only, under a marked, date-stamped section, so the user can tell
   their notes from the agent's and strip the agent's section easily:
   ```
   ---
   Agent · 2026-11-02
   <content>
   ```
2. Never rewrite or delete the user's existing notes.
3. Match effort to the task. Routine items get a light touch or nothing at
   all. Considered items get real context with links and a recommendation.
4. Keep additions scannable: short lines, links on their own lines. Things
   notes are plain text with link detection, not rendered Markdown.
5. Tell the user in the same turn what you added and to which task.
6. If you are unsure the addition is useful, propose it first instead of
   writing it.
7. Verify the edit by reading the task back with `find`.
