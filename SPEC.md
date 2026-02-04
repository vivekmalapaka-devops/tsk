# tsk — Terminal Todo Manager

## v1.0 Specification

---

## Overview

**tsk** is a fast, minimal todo manager that lives in your terminal. No bloat, no sync, no accounts — just a single binary and a local JSON file.

### Design Principles

1. **Fast** — Sub-10ms startup. Rust, single binary, no runtime.
2. **Minimal** — Do one thing well. No feature creep.
3. **Ergonomic** — Short commands, smart defaults, flexible input.
4. **Readable** — Clean output, good alignment, optional colors.
5. **Portable** — Single JSON file, easy to backup/sync manually.

---

## Installation

```bash
cargo install tsk
# or
cargo build --release && cp target/release/tsk ~/.local/bin/
```

---

## Commands Reference

### Add a Task

```bash
tsk add <text> [options]
tsk a <text> [options]          # alias
```

| Option | Description | Example |
|--------|-------------|---------|
| `-p <1\|2\|3>` | Priority (1=high, 2=med, 3=low) | `-p 1` |
| `-t <time>` | Deadline | `-t 5pm` |
| `+tag` | Add tag(s) | `+work +urgent` |

**Examples:**
```bash
tsk add "Buy milk"
tsk add "Call client" -p 1 -t 11am
tsk add "Fix login bug" -p 2 -t friday +work +backend
tsk a "Quick task"
```

---

### List Tasks

```bash
tsk                             # list open tasks (default: by priority)
tsk ls                          # alias
tsk all                         # include completed tasks
```

**Sorting:**
```bash
tsk --by priority               # sort by priority (default)
tsk --by time                   # sort by deadline
tsk --by created                # sort by creation date
```

**Filtering:**
```bash
tsk +work                       # tasks with +work tag
tsk +work +urgent               # tasks with BOTH tags
tsk today                       # due today
tsk week                        # due within 7 days
tsk overdue                     # past deadline
```

**Display:**
```bash
tsk --color                     # force color output
tsk --no-color                  # force plain output (default: auto-detect TTY)
```

---

### Complete a Task

```bash
tsk done <id> [id...]
tsk d <id> [id...]              # alias
```

**Examples:**
```bash
tsk done 3
tsk d 1 2 5                     # bulk complete
```

---

### Delete a Task

```bash
tsk delete <id> [id...]
tsk rm <id> [id...]             # alias
```

**Examples:**
```bash
tsk delete 4
tsk rm 2 3 6                    # bulk delete
```

---

### Edit a Task

```bash
tsk edit <id> [new text] [options]
tsk e <id> [new text] [options] # alias
```

| Option | Description |
|--------|-------------|
| `-p <1\|2\|3>` | Change priority |
| `-t <time>` | Change deadline |
| `+tag` | Add tag |
| `-tag` | Remove tag |
| `--clear-time` | Remove deadline |
| `--clear-priority` | Remove priority |

**Examples:**
```bash
tsk edit 2 "Updated task text"
tsk edit 3 -p 1                 # change priority only
tsk edit 4 -t monday            # change deadline only
tsk edit 5 +urgent              # add tag
tsk edit 5 -work                # remove tag
tsk e 6 --clear-time            # remove deadline
```

---

### Clear Completed

```bash
tsk clear                       # remove all completed tasks
```

---

### Stats

```bash
tsk stats
```

**Output:**
```
Open:        7
Completed:   23
Done today:  3
Done week:   12
Oldest:      5 days ("clean garage")
Top tag:     +work (12 tasks)
```

---

## Data Model

```rust
struct Todo {
    id: u32,
    text: String,
    done: bool,
    priority: Option<u8>,           // 1, 2, or 3
    deadline: Option<DateTime<Local>>,
    tags: Vec<String>,
    created_at: DateTime<Local>,
    completed_at: Option<DateTime<Local>>,
}
```

### JSON Storage Format

**Location:** `~/.tsk/todos.json`

```json
{
  "version": 1,
  "next_id": 5,
  "todos": [
    {
      "id": 1,
      "text": "Call client",
      "done": false,
      "priority": 1,
      "deadline": "2024-12-20T11:00:00",
      "tags": ["work"],
      "created_at": "2024-12-19T09:30:00",
      "completed_at": null
    }
  ]
}
```

---

## Priority System

| Value | Meaning | Display | Color |
|-------|---------|---------|-------|
| 1 | High | `!!!` | Red |
| 2 | Medium | `!!` | Yellow |
| 3 | Low | `!` | Blue |
| None | No priority | ` ` | Default |

---

## Time Input Formats

The `-t` flag accepts flexible time input:

| Input | Interpreted As |
|-------|----------------|
| `11am`, `11:00AM`, `11:00` | Today at 11:00 |
| `3pm`, `3:30pm` | Today at that time |
| `tomorrow`, `tomorrow 5pm` | Tomorrow (at 5pm or midnight) |
| `monday`, `mon`, `fri 3pm` | Next occurrence of that day |
| `12/25`, `12-25` | Dec 25 of current year |
| `2024-12-25` | Specific date |
| `in 2 hours`, `in 3 days` | Relative time |

**Implementation:** Use `chrono` with custom parser for natural language.

---

## Time Display Formats

| Condition | Display |
|-----------|---------|
| Overdue | `⚠ 2h ago`, `⚠ yesterday` |
| Today | `today 11:00am` |
| Tomorrow | `tomorrow 5:00pm` |
| This week | `fri 3:00pm` |
| Beyond | `dec 25` or `2025-01-15` |
| No deadline | `—` |

---

## Output Format

### Default List View

```
$ tsk
  1  !!!  Call client             today 11:00am      +work
  3  !!   Review PR               tomorrow           +work +code
  2  !    Buy groceries           fri
  4       Walk the dog            —
```

**Column Layout:**
```
[id]  [priority]  [text]              [deadline]         [tags]
```

- ID: right-aligned, 3 chars
- Priority: 4 chars fixed width (`!!!`, `!! `, `!  `, `   `)
- Text: left-aligned, truncated at 25 chars with `…`
- Deadline: right-aligned, 18 chars
- Tags: remaining space

### With Completed (tsk all)

```
$ tsk all
  1  !!!  Call client             today 11:00am      +work
✓ 5       Send invoice            done 2h ago        +work
  3  !!   Review PR               tomorrow           +work
```

### Overdue Highlighting

```
$ tsk
  1  !!!  Call client             ⚠ 2h ago           +work
  3  !!   Review PR               today 5:00pm       +work
```

---

## Color Scheme

| Element | Color | Condition |
|---------|-------|-----------|
| Priority `!!!` | Red | Always |
| Priority `!!` | Yellow | Always |
| Priority `!` | Blue | Always |
| Deadline | Red | Overdue |
| Deadline | Yellow | Due today |
| Deadline | Default | Future |
| Checkmark `✓` | Green | Completed |
| Tags | Cyan | Always |
| ID | Dim | Always |

**Auto-detection:** Colors enabled when stdout is TTY, disabled when piped.

---

## File Structure

```
tsk/
├── Cargo.toml
├── README.md
├── SPEC.md                    # this file
├── src/
│   ├── main.rs                # entry point, CLI setup
│   ├── cli.rs                 # clap argument definitions
│   ├── commands/
│   │   ├── mod.rs
│   │   ├── add.rs
│   │   ├── list.rs
│   │   ├── done.rs
│   │   ├── delete.rs
│   │   ├── edit.rs
│   │   ├── clear.rs
│   │   └── stats.rs
│   ├── todo.rs                # Todo struct, methods
│   ├── store.rs               # JSON persistence
│   ├── time.rs                # time parsing & formatting
│   └── display.rs             # output formatting, colors
```

---

## Dependencies

```toml
[package]
name = "tsk"
version = "1.0.0"
edition = "2021"
description = "A fast, minimal terminal todo manager"
license = "MIT"

[dependencies]
clap = { version = "4", features = ["derive"] }
chrono = { version = "0.4", features = ["serde"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
dirs = "5"
colored = "2"
atty = "0.2"

[profile.release]
lto = true
strip = true
codegen-units = 1
```

---

## Error Handling

| Error | Message |
|-------|---------|
| Task not found | `Error: Task #5 not found` |
| Invalid priority | `Error: Priority must be 1, 2, or 3` |
| Invalid time | `Error: Could not parse time "xyz"` |
| No tasks | `No open tasks. Use 'tsk add' to create one.` |
| Storage error | `Error: Could not read ~/.tsk/todos.json` |

---

## Exit Codes

| Code | Meaning |
|------|---------|
| 0 | Success |
| 1 | General error |
| 2 | Invalid arguments |

---

## Example Session

```bash
# Start fresh
$ tsk
No open tasks. Use 'tsk add' to create one.

# Add some tasks
$ tsk add "Call client about project" -p 1 -t 11am +work
Added #1: Call client about project [!!!] today 11:00am +work

$ tsk add "Buy groceries" -p 3 -t 6pm
Added #2: Buy groceries [!] today 6:00pm

$ tsk add "Review PR #42" -p 2 -t tomorrow +work +code
Added #3: Review PR #42 [!!] tomorrow +work +code

$ tsk add "Walk the dog"
Added #4: Walk the dog

# List tasks
$ tsk
  1  !!!  Call client about pr…   today 11:00am      +work
  3  !!   Review PR #42           tomorrow           +work +code
  2  !    Buy groceries           today 6:00pm
  4       Walk the dog            —

# Filter by tag
$ tsk +work
  1  !!!  Call client about pr…   today 11:00am      +work
  3  !!   Review PR #42           tomorrow           +work +code

# View by deadline
$ tsk --by time
  1  !!!  Call client about pr…   today 11:00am      +work
  2  !    Buy groceries           today 6:00pm
  3  !!   Review PR #42           tomorrow           +work +code
  4       Walk the dog            —

# Complete a task
$ tsk done 1
Completed #1: Call client about project

# Edit a task
$ tsk edit 3 -p 1 -t today
Updated #3: Review PR #42 [!!!] today

# Bulk operations
$ tsk d 2 4
Completed #2: Buy groceries
Completed #4: Walk the dog

# View all including completed
$ tsk all
  3  !!!  Review PR #42           today              +work +code
✓ 1       Call client about pr…   done 10m ago       +work
✓ 2       Buy groceries           done just now
✓ 4       Walk the dog            done just now

# Clear completed
$ tsk clear
Cleared 3 completed tasks.

# Stats
$ tsk stats
Open:        1
Completed:   3
Done today:  3
Done week:   3
Oldest:      just now
Top tag:     +work (2 tasks)
```

---

## Future Considerations (v2+)

Not in v1, but designed to accommodate:

- `tsk undo` — restore last action
- `tsk export --md` — export to markdown
- `tsk archive` — move completed to separate file
- `tsk search <query>` — full text search
- Recurring tasks
- Interactive TUI mode
- Shell completions (bash, zsh, fish)
- Config file for defaults

---

## License

MIT

---

*End of specification.*
