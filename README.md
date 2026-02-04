# tsk

A fast, minimal terminal todo manager.

## Install

```bash
cargo install --path .
```

Or copy the binary:
```bash
cargo build --release
cp target/release/tsk ~/.local/bin/
```

## Usage

```bash
# Add tasks
tsk add "Buy groceries"
tsk add "Call client" -p 1 -t 11am @work +urgent
tsk a "Quick note" -p 2 -t tomorrow @personal

# List tasks
tsk                    # open tasks, sorted by priority
tsk --by time          # sorted by deadline
tsk -T work            # filter by tag
tsk -P work            # filter by project
tsk today              # due today
tsk week               # due this week

# Projects
tsk project work       # all tasks in project
tsk projects           # list all projects

# Complete & delete
tsk done 1             # mark as done
tsk d 1 2 3            # bulk complete
tsk delete 4           # delete task
tsk rm 5 6             # bulk delete

# Edit tasks
tsk edit 2 "New text"
tsk edit 2 -p 1 -t friday
tsk e 2 +newtag -oldtag
tsk edit 2 @newproject
tsk edit 2 --clear-project

# Other
tsk all                # show completed too
tsk stats              # progress overview
tsk clear              # remove completed
```

## Priority

- `-p 1` → `!!!` (high, red)
- `-p 2` → `!!` (medium, yellow)
- `-p 3` → `!` (low, blue)

## Projects & Tags

- `@project` - assign to a project (one per task, shown in magenta)
- `+tag` - add a tag (multiple allowed, shown in cyan)

## Time formats

`11am`, `3:30pm`, `tomorrow`, `friday`, `fri 5pm`, `12/25`, `2024-12-25`, `in 2 hours`

## Storage

Tasks stored in `~/.tsk/todos.json`
