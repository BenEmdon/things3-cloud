# Agent skills for things3-cloud

Generic skills for letting a coding agent read and write Things 3 through the
`things3` CLI. They are intentionally not tied to any one user, area list, or
workflow.

Install the CLI from this repo first:

```
cargo install --path .
```

Then authenticate once:

```
things3 set-auth
```

The CLI handles its own credentials. Never print, copy, or commit a credential,
and never paste one into chat, code, or a task note. Never run
`things3 webserver`.

## Skills

- `things3-review/`: read-only. Check views, search tasks, and list areas,
  projects, and tags.
- `things3-create-task/`: create tasks, file them in the right area or project,
  and schedule them.
- `things3-enrich-task/`: add context to an existing task without rewriting
  the user's notes.

Use the skill that matches the job. For a job that needs both reading and
writing, use the review skill to find the task first, then the create or
enrich skill to change it, and verify the change by reading it back.
