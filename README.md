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
tsk add "Call client" -p 1 -t 11am +work
tsk a "Quick note" -p 2 -t tomorrow +urgent

# List tasks
tsk                    # open tasks, sorted by priority
tsk --by time          # sorted by deadline
tsk +work              # filter by tag
tsk today              # due today
tsk week               # due this week

# Complete & delete
tsk done 1             # mark as done
tsk d 1 2 3            # bulk complete
tsk delete 4           # delete task
tsk rm 5 6             # bulk delete

# Edit tasks
tsk edit 2 "New text"
tsk edit 2 -p 1 -t friday
tsk e 2 +newtag -oldtag

# Other
tsk all                # show completed too
tsk stats              # progress overview
tsk clear              # remove completed
```

## Priority

- `-p 1` → `!!!` (high, red)
- `-p 2` → `!!` (medium, yellow)
- `-p 3` → `!` (low, blue)

## Time formats

`11am`, `3:30pm`, `tomorrow`, `friday`, `fri 5pm`, `12/25`, `2024-12-25`, `in 2 hours`

## Storage

Tasks stored in `~/.tsk/todos.json`
