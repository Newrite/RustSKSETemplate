# RustSKSETemplate

A current template for SKSE plugins built on the modern `libskyrim` high-level SDK.

This template is intentionally updated for the current realities of `libskyrim`:
- current `SKSEPlugin_Version` declaration via `plugin_declaration!`
- current Rust entrypoint ABI via `skse_plugin_rust_entry(&LoadInterface)`
- high-level plugin lifecycle and SKSE messaging through `sdk::plugin`
- high-level hooks through `sdk::hooks`
- high-level events through `sdk::events`
- high-level cosave persistence through `sdk::plugin::serialization`
- low-level escape hatches still available through `skse`, `relocation`, and raw hook macros

By default the template stays minimal and safe:
- it initializes `libskyrim`
- registers lifecycle callbacks
- registers a working serialization model
- keeps sample hooks in `src/sample_hooks.rs` as reference code, but does not install them automatically

---

## Requirements

- Rust stable with `x86_64-pc-windows-msvc`
- MSVC build tools
- `xmake`
- Git

```powershell
rustup target add x86_64-pc-windows-msvc
winget install xmake
```

---

## Quick Start

```powershell
git clone https://github.com/Newrite/RustSKSETemplate
cd RustSKSETemplate
cargo build --release --target x86_64-pc-windows-msvc
```

Output DLL:

```text
target/x86_64-pc-windows-msvc/release/RustSKSETemplate.dll
```

Copy it to `Data/SKSE/Plugins/`.

---

## Runtime Features

In `Cargo.toml`:

```toml
[dependencies]
libskyrim = { git = "https://github.com/Newrite/CommonLib-SkyrimLib.git", branch = "CommonLib-LibSkyrim", features = ["se", "ae"] }
```

Typical choices:
- `features = ["se", "ae"]` for one universal SE/AE build
- `features = ["ae"]` for AE-only
- `features = ["se"]` for 1.5.97-only
- `features = ["vr"]` for VR-only

If you add real hook targets, make sure their relocation IDs and offsets are valid for every runtime you enable.

---

## Project Layout

```text
src/
  lib.rs           current entrypoint + lifecycle wiring
  persistence.rs   working high-level serialization example
  sample_hooks.rs  sample hook examples you can wire in when ready
build.rs           DLL resource metadata
Cargo.toml         package metadata and runtime selection
rust-toolchain.toml pinned stable toolchain + MSVC target for IDEs
.cargo/config.toml default cargo target for rust-analyzer / RustRover
```

---

## Current Entry Point

The template uses the current `libskyrim` ABI:

```rust
#[unsafe(no_mangle)]
pub unsafe fn skse_plugin_rust_entry(skse: &LoadInterface) -> Result<(), ()> {
    plugin::init(skse);
    plugin::set_level(LogLevel::Info);
    Ok(())
}
```

`plugin::init(skse)` is the normal high-level bootstrap path. After that you can use SDK lifecycle, events, hooks, tasks, and serialization helpers.

## IDE Notes

The template is set up to behave better in RustRover and VSCode:
- `crate-type = ["cdylib", "rlib"]` keeps the final plugin output while also giving the IDE a normal Rust library target to analyze
- `rust-toolchain.toml` pins stable and adds `x86_64-pc-windows-msvc`
- `.cargo/config.toml` makes the MSVC target the default for cargo-based analysis

If IntelliSense still looks stale after pulling template updates:
- reopen the project root
- run `cargo check`
- refresh the Rust toolchain / cargo workspace in the IDE

The plugin version export is also current:

```rust
libskyrim::plugin_declaration! {
    version: Version::new(0, 1, 0, 0),
    name: "RustSKSETemplate",
    author: "YourName",
    support_email: "you@example.com",
    struct_compatibility: StructCompatibility::Independent,
    runtime_compatibility: RuntimeCompatibility::from_version_independence(
        VersionIndependence::AddressLibrary,
        false,
    ),
    minimum_skse_version: Version::new(0, 0, 0, 0),
}
```

---

## High-Level SDK Surfaces

### `sdk::plugin`

Use this for the common bootstrap workflow.

Available in practice:
- `plugin::init(...)`
- `plugin::init_with_log(...)`
- `plugin::alloc_trampoline(...)`
- `plugin::set_level(...)`
- `plugin::add_task(...)`
- `plugin::add_ui_task(...)`
- lifecycle helpers such as:
  - `plugin::on_post_load(...)`
  - `plugin::on_input_loaded(...)`
  - `plugin::on_data_loaded(...)`
  - `plugin::on_pre_load_game(...)`
  - `plugin::on_post_load_game(...)`
  - `plugin::on_save_game(...)`
  - `plugin::on_delete_game(...)`
  - `plugin::on_new_game(...)`

The template already wires these in `src/lib.rs`.

### `sdk::events`

Use this for engine and SKSE event domains.

Available domains:
- `sdk::events::game`
- `sdk::events::ui`
- `sdk::events::input`
- `sdk::events::source`
- `sdk::events::skse::dispatchers`
- `sdk::events::skse::messages`
- `sdk::events::bus`

Useful pieces:
- `EventBatch`
- `EventFlow`
- `install_all!(&mut batch, ...)`
- attribute events:
  - `#[events::game_event]`
  - `#[events::ui_event]`
  - `#[events::dispatcher_event]`
  - `#[events::input_event]`
  - `#[events::message_event]`
  - `#[events::bus_event]`

Example input event:

```rust
use libskyrim::sdk::events::{self, InputEvents};

#[events::input_event(prepend)]
fn on_input(mut events: InputEvents<'_>) {
    for button in events.keyboard_mut() {
        let _ = button;
    }
}

let mut batch = events::EventBatch::new();
events::install_all!(&mut batch, on_input_event)?;
```

### `sdk::hooks`

This is the modern high-level hook layer.

Supported attributed hooks:
- `#[hooks::function_hook]`
- `#[hooks::call_hook]`
- `#[hooks::vtable_hook]`
- `#[hooks::vcall_hook]`

Important features:
- typed arguments like `&T`, `Option<&T>`, `&mut T`, and `Resolved<T>`
- `Original<fn(...) -> ...>`
- guard presets via `hooks::guards::*`
- batch installation via `hooks::install_all![...]`

Example high-level call hook:

```rust
use libskyrim::re::Actor;
use libskyrim::relocation::{RelocationID, VariantOffset};
use libskyrim::sdk::hooks::{self, Original};

#[hooks::call_hook(
    target = RelocationID::new(123, 456),
    offset = VariantOffset::new(0x15, 0x18, 0x15),
    size = 5,
    guard = hooks::guards::original(),
)]
fn patch_call(
    original: Original<fn(&Actor, u32)>,
    actor: &Actor,
    value: u32,
) {
    original.call(actor, value);
}
```

Example high-level vtable hook on the player update slot:

```rust
use libskyrim::re::{Actor, PlayerCharacter};
use libskyrim::sdk::hooks::{self, Original};

#[hooks::vtable_hook(
    vtable = PlayerCharacter::VTABLE[0],
    offset = Actor::VFUNC_UPDATE,
    guard = hooks::guards::original(),
)]
fn patch_player_update(
    original: Original<fn(&mut PlayerCharacter, f32)>,
    player: &mut PlayerCharacter,
    delta: f32,
) {
    original.call(player, delta);
}
```

The template includes two worked examples in `src/sample_hooks.rs`:
- a `call_hook` adaptation of `ApplyAttackSpells`
- a `vtable_hook` on `PlayerCharacter` using the inherited `Actor::VFUNC_UPDATE` slot

Wire them in intentionally after you replace addresses and behavior for your own plugin.

### `sdk::plugin::serialization`

This is the high-level cosave API.

The template already uses it in `src/persistence.rs`.

It provides:
- `Model`
- `register_model::<T>()`
- `schema_fields!`
- `record_id!(...)`
- `unique_id!(...)`
- `Cosave`, `CosaveEncode`, `CosaveDecode`
- container support such as `Vec<T>`, `BoundedVec<T, N>`, `BTreeMap<K, V>`

Current template example:

```rust
#[derive(Debug, Default, Cosave)]
pub struct SaveState {
    pub save_counter: u32,
    pub runtime_events: u32,
}

impl Model for SaveState {
    const UNIQUE_ID: serialization::UniqueId = serialization::unique_id!("RSTT");

    fn schema(schema: &mut Schema<Self>) {
        serialization::schema_fields!(schema, {
            serialization::record_id!("SAVE") => 1 => save_counter,
            serialization::record_id!("EVNT") => 1 => runtime_events,
        });
    }
}
```

---

## Low-Level Surfaces Still Available

The template is oriented toward the SDK, but the raw layers are still there when needed.

### Raw SKSE / relocation layer

You can still use:
- `libskyrim::skse::*`
- `libskyrim::relocation::*`
- raw trampoline allocation
- low-level registration families
- low-level raw message registration

### Raw hook escape hatch

For unsupported or intentionally low-level cases you still have:
- `hook! { function ... }`
- `hook! { detour ... }`
- `hook! { call ... }`
- `hook! { vcall ... }`
- `hook! { vtable ... }`
- legacy `define_call_hook!` / `define_vtable_hook!`

Example raw low-level call hook:

```rust
libskyrim::hook! {
    pub call SomeSite {
        target: RelocationID::new(123, 456),
        offset: VariantOffset::new(0x15, 0x18, 0x15),
        size: 5,
        fn detour(arg1: *mut SomeType, arg2: u32) {
            original(arg1, arg2)
        }
    }
}
```

Use these when the attribute SDK layer is too restrictive or you need exact raw ABI control.

---

## Example Hook in This Template

`src/sample_hooks.rs` contains two hook examples:
- a high-level call-hook adaptation of this CommonLib-style pattern:
- hook one specific call site
- if `InventoryEntryData*` is null, synthesize fallback data
- otherwise pass the original argument through
- a `PlayerCharacter` vtable hook on the inherited `Actor::Update(float)` slot

Together they show:
- `#[hooks::call_hook(...)]`
- `#[hooks::vtable_hook(...)]`
- `Original<fn(...)>`
- `Resolved<Actor>`
- `Option<&mut InventoryEntryData>`
- `VariantOffset`
- `RelocationID`
- inherited vtable offsets like `Actor::VFUNC_UPDATE`
- concrete vtable targets like `PlayerCharacter::VTABLE[0]`

The sample uses these addresses:
- containing function: `RelocationID::new(37673, 38627)`
- call-site offset: `VariantOffset::new(0x185, 0x194, 0x185)`
- player update vtable slot: `Actor::VFUNC_UPDATE`

Treat this as a worked example, not as a universal production hook. Replace addresses and fallback logic for your own plugin.

---

## Suggested Workflow

1. Rename the crate and plugin metadata in `Cargo.toml`, `build.rs`, and `src/lib.rs`.
2. Keep `src/lib.rs` small: bootstrap, logging, lifecycle wiring.
3. Put persistence in `src/persistence.rs`.
4. Put real hooks into their own modules.
5. Install hooks from `on_data_loaded(...)` unless a different lifecycle phase is clearly required.
6. Prefer `sdk::hooks` / `sdk::events` / `sdk::plugin::serialization` first.
7. Drop to raw `skse` / `relocation` only when the SDK layer is too narrow.

---

## Notes

- The template is working by default without installing the sample hooks.
- The sample hooks are reference code only; real hook addresses and behavior are always plugin-specific.
- If you target VR too, audit every hook target and every runtime-specific offset before enabling VR builds.
