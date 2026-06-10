# rmatrix

A `cmatrix` copy written in rust with more tunable paramaters.

---

## Installation

To run (debug)

```
cargo run
```

System-wide build and installation. remember to add `cargo/bin` to PATH

```
cargo install --path .
```

---

## Usage

```
rmatrix [OPTIONS]
```

### Options

| Flag | Description | Default |
|------|-------------|---------|
| `-r`, `--rainbow` | random colors every frame | `off` |
| `--static-rainbow` | colors assigned once per streak | `off` |
| `-c`, `--color <COLOR>` | Single color | `darkgreen` |
| `--duo <COLOR> <COLOR>` | Two alternating colors per streak | `off` |
| `--set-characters` | Streaks does not change symbols  | `off` |
| `--lambda` | Replace all characters with `λ` | `off` |
| `-s`, `--speed <MS>` | Milliseconds between updates (lower = faster speed) | `35` |
| `--spawn <RATE>` | Streak spawn rate multiplier (`0.0`–`N`) | `1.0` |

### Available colors

`red` `green` `yellow` `blue` `magenta` `cyan` `white` `grey`
`darkred` `darkgreen` `darkyellow` `darkblue` `darkmagenta` `darkcyan`

---

## Controls

| Key | Action |
|-----|--------|
| `q` `Esc` | Quit |

