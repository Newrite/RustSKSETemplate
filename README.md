# RustSKSETemplate

A template for writing [SKSE](https://skse.silverlock.org/) plugins for **The Elder Scrolls V: Skyrim** entirely in Rust, powered by [CommonLib-SkyrimLib](https://github.com/Newrite/CommonLib-SkyrimLib/tree/CommonLib-LibSkyrim) — a Rust port of CommonLibSSE-NG.

Supports **SE (1.5.97)**, **AE (1.6.x+)**, and **VR** runtimes.

---

## Requirements

- [Rust](https://rustup.rs/) (stable, MSVC toolchain)
- MSVC Build Tools (Visual Studio 2022 or standalone) with C++23 and later
- [xmake](https://xmake.io/) >= 3.0.0 — builds the C++ CommonLibSSE-NG bridge
- Git

### Install xmake

```shell
winget install xmake
# or
scoop install xmake
```

### Install the MSVC Rust target

```shell
rustup target add x86_64-pc-windows-msvc
```

---

## Quick Start

```shell
git clone https://github.com/Newrite/RustSKSETemplate
cd RustSKSETemplate
cargo build --release --target x86_64-pc-windows-msvc
```

Output DLL:
```
target/x86_64-pc-windows-msvc/release/<PluginName>.dll
```

Copy to `Data/SKSE/Plugins/` in your Skyrim directory.

---

## Skyrim Runtime Target (Features)

The `libskyrim` dependency must be told which Skyrim runtime(s) to target.
This controls which version of CommonLibSSE-NG gets compiled by xmake.

In `Cargo.toml`, set the `features` field on the `libskyrim` dependency:

```toml
[dependencies]
# SE + AE (default — universal plugin)
libskyrim = { git = "https://github.com/Newrite/CommonLib-SkyrimLib.git", branch = "CommonLib-LibSkyrim", features = ["se", "ae"] }

# AE only
libskyrim = { git = "...", branch = "CommonLib-LibSkyrim", features = ["ae"] }

# SE only (1.5.97)
libskyrim = { git = "...", branch = "CommonLib-LibSkyrim", features = ["se"] }

# VR only
libskyrim = { git = "...", branch = "CommonLib-LibSkyrim", features = ["vr"] }
```

| Feature | Runtime |
|---|---|
| `se` | Special Edition 1.5.97 |
| `ae` | Anniversary Edition 1.6.x+ |
| `vr` | Skyrim VR |

> **Note:** These features are passed through to xmake as `--skyrim_se=y/n`, `--skyrim_ae=y/n`, `--skyrim_vr=y/n` during the C++ build step.

---

## Build

```shell
# Release — optimized, stripped (for distribution)
cargo build --release --target x86_64-pc-windows-msvc

# Debug — unoptimized, with debug symbols (for development)
cargo build --target x86_64-pc-windows-msvc
```

> There is no `--dev` flag in cargo. Omitting `--release` uses the `dev` profile automatically.

---

## Update libskyrim

```shell
cargo update -p libskyrim
```

---

## Plugin Configuration (`src/lib.rs`)

### Version Declaration

```rust
libskyrim::plugin_api::plugin_version_data! {
    author:           "YourName",
    email:            "you@example.com",
    version_indep_ex: SksePluginVersionData::VINDEPEX_NO_STRUCT_USE,
    version_indep:    SksePluginVersionData::VINDEP_ADDRESS_LIBRARY_POST_AE,
    compat_versions:  []
}
```

| Field | Description |
|---|---|
| `author` | Your name shown in SKSE logs |
| `email` | Contact email |
| `version_indep_ex` | `VINDEPEX_NO_STRUCT_USE` — plugin does not use SKSE structs directly |
| `version_indep` | `VINDEP_ADDRESS_LIBRARY_POST_AE` — uses Address Library, compatible with all post-AE versions |
| `compat_versions` | Restrict to specific game versions, or `[]` for all |

### Entry Point

```rust
#[no_mangle]
pub fn skse_plugin_rust_entry(skse: &SkseInterface) -> Result<(), ()> {
    // Your initialization code
    Ok(())   // return Err(()) to abort plugin load
}
```

### SKSE Message Listeners

```rust
register_listener(Message::SKSE_DATA_LOADED, on_skse_message);
```

| Message | When |
|---|---|
| `SKSE_POST_LOAD` | All plugins loaded |
| `SKSE_POST_POST_LOAD` | After PostLoad of all plugins |
| `SKSE_INPUT_LOADED` | Input system ready |
| `SKSE_DATA_LOADED` | Game data fully loaded — **main initialization point** |
| `SKSE_PRE_LOAD_GAME` | Before loading a save |
| `SKSE_POST_LOAD_GAME` | After loading a save |
| `SKSE_SAVE_GAME` | On save |
| `SKSE_NEW_GAME` | New game started |

### Runtime Detection

```rust
use libskyrim::runtime::{CURRENT_VERSION, is_ae, is_se, is_vr};

skse_message!("Skyrim version: {}", *CURRENT_VERSION);
if is_ae() { /* AE-specific code */ }
if is_se() { /* SE-specific code */ }
if is_vr() { /* VR-specific code */ }
```

### Logging

```rust
skse_message!("Info: {}", value);   // [INFO]
skse_warning!("Warn: {}", value);   // [WARN]
skse_fatal!("Fatal: {}", value);    // [FATAL]
```

---

## Build Profiles

| Profile | Command | LTO | Opt | Strip | Use for |
|---|---|---|---|---|---|
| `release` | `cargo build --release` | fat | z | yes | Distribution |
| `dev` | `cargo build` | off | 0 | no | Development |

---

## Project Structure

```
src/
  lib.rs       ← Entry point, version data, message listeners
build.rs       ← DLL linker flags, Windows version resource embedding
Cargo.toml     ← Plugin name, version, libskyrim dependency + features
```
