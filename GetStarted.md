# Getting Started

## MSRV (Minimum supported Rust version)
- at least 1.82.0

## Running the bot (server)
```bash
sh start.sh
```

## Running the bot (debug)
```bash
cargo run
```

You can use `-r` flag for release build, and `-- -y` to always cache files on ctrl-c

There are also a couple of helper binaries

`cache_now`: cache all the layout in case `cached_stats.json` is empty
- note: `cached_stats.json` cannot be missing or invalid
```bash
cargo run --bin cache_now
```

`sync_from_cmini`: convert `authors.json`, `likes.json`, and `layouts/` to rmini format
- copy `authors.json`, `likes.json`, and `layouts/` into `./cmini/input/`
- (or let `sync_from_cmini` generate for you)
- retrieve `authors.json`, `likes.json`, and `layouts.json` from `./cmini/output/`
```bash
cargo run --bin sync_from_cmini
```

## Discord bot token
Put your discord bot token in token.txt in root (same level as Cargo.toml)
```
├── ...
├── Cargo.toml
└── token.txt
```

## Admin permissions
See `admins.json`, the admins are stored as user ids
- Note:
  - owner is a required field
  - admins are optional
  - owner must not appear in admins
- If you run the bot for AKL, you may change the owner id to your id 

# Implementation

## Layout
Layouts are stored in a string, for every 4 characters represents a key and the position

Example of the qwerty layout (first row)
```
q000w011e022r033t043y056u066i077o088p099[0a9]0b9\\0c9
```

### Interpretation
from left to right: [key, row, column, finger]
```
q000
│││└─finger
││└─column
│└─row
└─key
```
- `key` can be any UTF-8 character
- `row` is `0..=3` (represents at most 4 rows)
- `col` is `[0-9][a-z]` (represents at most 36 columns)
- `finger` is `0..=9` (LP = 0, LR = 1, ... LT = 4, RT = 5, ... RR = 8, RP = 9)

## Cached stats
A stat is stored in a string, for every 3 characters represents a metric frequency in base 64

Example of qwerty stats in shai
```
C6EADKCU/EuzBVMBewAGrBCaAJkAUJAWtE2IEJGAo2
```
The metrics are in the same order of the `Metric` enum variants

### Interpretation
since all values are ratios `0.0 ..= 1.0`, we can map them to 100000 values (max is actually 64^3 but I used 100000 for convenience)
then sumprod from left to right: `[64^2, 64, 1]`
```
C6E
││└─4 +
│└─32 * 64 +
└─2 * 64^2
sum: 10244 / 100 000 = 0.10244 (or 10.244%)
```

### Rounding errors
The error is 1 / 100 000, or 0.001%

assuming stats are displayed as percentages with 2 decimal places, it starts accumulating errors after at least 5 additions

# Notes
Sorry if the code is messy or undocumented, as the project is a bit rushed. I will add them in the future. 
