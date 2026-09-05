# **W**hat **A**m **I** **M**issing?

A simple todo utility written in Rust.

## Installation

### From GitHub Releases

Download from [Releases](https://github.com/kasapeli/waim/releases/)

```bash
chmod +x waim-linux-x86_64
sudo mv waim-linux-x86_64 /usr/local/bin/waim
```

### From source

```bash
git clone https://github.com/kasapeli/waim.git (git@github.com:kasapeli/waim.git via SSH)
cd waim
cargo install --path .
```

## Usage

| Command  | Description                                               |
|----------|-----------------------------------------------------------|
| `init`   | Initializes WAIM in the current directory                 |
| `add`    | Adds a task to WAIM                                       |
| `list`   | Lists current tasks (options: `all`, `done`, `undone`)    |
| `delete` | Deletes a task by id                                      |
| `toggle` | Toggles a task's completion state                         |
| `help`   | Print this message or the help of the given subcommand(s) |

## Platforms

Prebuilt binaries are provided for:

- Linux (x86_64, aarch64)
- macOS (x86_64, aarch64 / Apple Silicon)
- Windows (x86_64)

## [License](https://github.com/kasapeli/waim/blob/main/LICENSE)
