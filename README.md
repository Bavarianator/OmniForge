# OmniForge

Native Linux “AI Creator Studio” for local fine-tuning, RAG, and GGUF export — GTK4 + Libadwaita + Relm4 frontend, Rust core orchestrator, Python training bridge.

## Systemabhängigkeiten (Linux)

**Debian / Ubuntu** — Libadwaita + pkg-config (GTK4-Dev hast du oft schon):

```bash
sudo apt update
sudo apt install libadwaita-1-dev pkg-config
```

**Fedora:** `sudo dnf install libadwaita-devel pkgconf-pkg-config`  
**Arch:** `sudo pacman -S libadwaita`

Prüfen: `pkg-config --modversion libadwaita-1` sollte eine Versionsnummer ausgeben.

## Build

```bash
cargo build -p omniforge-gui
```

Run the scaffold UI:

```bash
cargo run -p omniforge-gui
```

Point the trainer at the repo root (for `python/train_lora.py` resolution) or export `OMNIFORGE_ROOT`.

**Event flow:** GTK / Relm4 → `blocking_send(GuiCommand)` → core thread (Tokio) → `BackendEvent` → bridge thread → `async-channel` → `MainContext::spawn_local` → `AppMsg::Backend`.

## Layout

See `crates/` for `omniforge-gui`, `omniforge-core`, and `omniforge-common`, plus `python/` for the bundled training environment.

## Tests

Core smoke tests live in `crates/omniforge-core/tests/smoke.rs`.
