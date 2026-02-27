# Eshara

A terminal-based narrative survival game inspired by [Lifeline](https://en.wikipedia.org/wiki/Lifeline_(video_game)), built with Rust and [ratatui](https://ratatui.rs).

You are the only person who can hear Elara — a stranger stranded in a hostile, collapsing landscape after a mysterious signal disrupted everything. Through a crackling radio link, you guide her choices: where to go, who to trust, when to rest, and when to run. Your decisions shape her survival, her trust in you, and the ending she reaches.

---

## Features

- **Branching narrative** across 5 acts, 77 story nodes, and 5 distinct endings
- **Real-time delays** — Elara sometimes goes dark while traveling or resting; come back later
- **Typewriter chat UI** with a typing indicator, message history, and scroll
- **Tracked stats** — trust, health, and supplies shift based on your choices
- **Flag-based branching** — 22 story flags gate paths and unlock dialogue
- **Death system** — if Elara's health drops to zero, the story ends
- **Bilingual** — fully playable in English and French (switchable mid-game)
- **Save system** — auto-saves after every choice; resume where you left off
- **Data-driven story** — the entire narrative is defined in a single JSON file (`data/story.json`), with the engine reading structure, stats, branches, and endings from it

## Screenshot

```
            E S H A R A

  Elara: Hello? Is anyone there? The signal is weak...
  Elara: I don't know where I am. Everything is dust and static.

                           I can hear you. Are you hurt? >

  > Are you okay? What happened to you?
    Where exactly are you?

                                               [Esc] Menu
```

## Getting started

### Requirements

- [Rust](https://rustup.rs/) 1.70+ (edition 2021)

### Install from GitHub

```sh
cargo install --git https://github.com/YannickHerrero/Eshara
eshara
```

### Build from source

```sh
git clone https://github.com/YannickHerrero/Eshara.git
cd Eshara
cargo run
```

Or build a release binary:

```sh
cargo build --release
./target/release/eshara
```

### CLI options

| Flag | Description |
|------|-------------|
| `--reset` | Delete save file and exit |
| `--lang en` / `--lang fr` | Override starting language |

### Debug mode

Set `ESHARA_DEBUG=1` to reduce all real-time delays to 5 seconds:

```sh
ESHARA_DEBUG=1 cargo run
```

## Controls

| Key | Action |
|-----|--------|
| `Up` / `k` | Move selection up |
| `Down` / `j` | Move selection down |
| `Enter` | Confirm selection |
| Any key | Skip typewriter animation |
| `Esc` | Open pause menu |

## Project structure

```
src/
  main.rs          Entry point, CLI parsing, app setup
  lib.rs           Shared crate root (Ctrl+C handler)
  tui.rs           Ratatui UI: rendering, event loop, game flow
  game.rs          GameState, Stats, save/load, flags
  story/mod.rs     Story data structures, JSON loading, validation
  i18n.rs          Localization (en/fr system messages)
  time.rs          Real-time delay scheduling
data/
  story.json       The complete story (nodes, branches, endings, stats)
```

## Story format

The story is a single JSON file with this structure:

```jsonc
{
  "meta": { "title": "Eshara", "start_node": "a1_first_contact", ... },
  "stats": { "trust": { "initial": 3, "min": 0, "max": 10 }, ... },
  "flags": { "has_shielding": "Found shielding material", ... },
  "endings": { "still_here": { "title": {"en": "Still Here", "fr": "..."}, "type": "good" }, ... },
  "death_check": { "override_next_node": "ending_gone_dark" },
  "nodes": {
    "a1_first_contact": {
      "id": "a1_first_contact",
      "act": 1,
      "messages": [{"en": "Hello?", "fr": "Allô ?"}],
      "choices": [
        { "label": {"en": "Are you okay?", "fr": "Ça va ?"}, "next_node": "a1_respond_ok",
          "on_choose": { "trust_change": 1 } }
      ],
      "on_enter": { "health_change": -1, "flags_set": ["flag_name"] },
      "branch": [
        { "condition": { "min_trust": 7 }, "next_node": "high_trust_path" },
        { "condition": { "default": true }, "next_node": "low_trust_path" }
      ],
      "delay": { "seconds": 300, "message": {"en": "Elara is resting...", "fr": "..."} },
      "ending": "still_here"
    }
  }
}
```

The JSON is embedded at compile time and can be overridden by placing a `data/story.json` file next to the binary at runtime.

## Save data

Game state is saved to `~/.eshara/save.json`. Use `--reset` to delete it.

## License

MIT
