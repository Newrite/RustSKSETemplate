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

---

## Hooks

Install hooks inside the `SKSE_DATA_LOADED` handler — at that point all game
data and vtables are fully initialized and safe to patch.

```rust
fn on_skse_message(msg: &Message) {
    match msg.msg_type {
        Message::SKSE_DATA_LOADED => {
            hooks::install_hooks();
        }
        _ => {}
    }
}
```

---

### VTable Hook

Replaces a virtual function in a class vtable. The original function is saved
and can be called via `original(...)`.

```rust
use libskyrim::re::character::Character;
use libskyrim::re::player_character::PlayerCharacter;
use libskyrim::{skse_message, define_vtable_hook};
use libskyrim::trampoline::alloc_trampoline;

define_vtable_hook! {
    pub NpcUpdateHook {
        vtable: Character::VTABLE,
        offset: Character::VFUNC_UPDATE_IDX,
        fn hook(this: *mut Character, delta: f32) {
            skse_message!("NPC update, delta: {}", delta);
            original(this, delta); // call original — always do this unless intentionally blocking
        }
    }
}

define_vtable_hook! {
    pub PlayerUpdateHook {
        vtable: PlayerCharacter::VTABLE,
        offset: PlayerCharacter::VFUNC_UPDATE_IDX,
        fn hook(this: *mut PlayerCharacter, delta: f32) {
            skse_message!("Player update, delta: {}", delta);
            original(this, delta);
        }
    }
}

pub fn install_hooks() {
    alloc_trampoline(128); // allocate trampoline memory before installing hooks
    NpcUpdateHook::install();
    PlayerUpdateHook::install();
}
```

**`define_vtable_hook!` parameters:**

| Parameter | Description |
|---|---|
| `vtable` | VTable address — `SomeClass::VTABLE[0]` (index 0 = primary vtable) |
| `offset` | Virtual function index — `SomeClass::VFUNC_NAME_IDX` constant |
| `fn hook(...)` | Your hook function — signature must match the original exactly |
| `original(...)` | Calls the original function saved before patching |

---

### Call Hook (Trampoline)

Patches a `CALL` instruction inside an existing function at a specific offset.
Use when you need to intercept a specific call site, not the whole virtual method.

```rust
use libskyrim::{skse_message, define_call_hook};
use libskyrim::relocation::VariantID;
use libskyrim::trampoline::alloc_trampoline;

define_call_hook! {
    pub MyCallHook {
        address: VariantID::new(37633, 38586, 0), // SE id, AE id, VR id
        offset: 0x3A,                              // byte offset of the CALL instruction
        size: 5,                                   // 5 = short CALL, 6 = long CALL
        fn hook(arg1: *mut SomeType, arg2: u32) -> bool {
            skse_message!("Call hook fired");
            original(arg1, arg2) // call original at hook site
        }
    }
}

pub fn install_hooks() {
    alloc_trampoline(64);
    MyCallHook::install();
}
```

**`define_call_hook!` parameters:**

| Parameter | Description |
|---|---|
| `address` | `VariantID` of the **containing function** (not the call target) |
| `offset` | Byte offset of the `CALL` instruction within that function |
| `size` | `5` for a standard near call, `6` for a call with REX prefix |
| `fn hook(...)` | Your hook — signature must match the called function exactly |

---

### `alloc_trampoline(size)`

Must be called **once** before any `install()` calls. Allocates a memory region
near the game executable for trampoline stubs used by call/branch hooks.

```rust
alloc_trampoline(128); // size in bytes — 64–256 is typical
```

VTable hooks do not use the trampoline, but calling `alloc_trampoline` before
all hooks is the safe convention.

---

### Hook Types Summary

| Macro | Use case |
|---|---|
| `define_vtable_hook!` | Override a virtual method for all instances of a class |
| `define_call_hook!` | Intercept a specific `CALL` instruction inside a function |
