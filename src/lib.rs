pub use core;

use libskyrim::{skse_message, skse_warning, skse_fatal};
use libskyrim::plugin_api::{SksePluginVersionData, SkseInterface, Message, register_listener};

// ── SKSEPlugin_Version ───────────────────────────────────────────────────────
libskyrim::plugin_api::plugin_version_data! {
    author:           "YourName",
    email:            "you@example.com",
    version_indep_ex: SksePluginVersionData::VINDEPEX_NO_STRUCT_USE,
    version_indep:    SksePluginVersionData::VINDEP_ADDRESS_LIBRARY_POST_AE,
    compat_versions:  []
}

// ── SKSE Message listener ────────────────────────────────────────────────────
fn on_skse_message(msg: &Message) {
    match msg.msg_type {
        Message::SKSE_POST_LOAD =>
            skse_message!("PostLoad"),

        Message::SKSE_POST_POST_LOAD =>
            skse_message!("PostPostLoad"),

        Message::SKSE_INPUT_LOADED =>
            skse_message!("InputLoaded"),

        Message::SKSE_DATA_LOADED =>
            skse_message!("DataLoaded"),


        Message::SKSE_PRE_LOAD_GAME =>
            skse_message!("PreLoadGame"),

        Message::SKSE_POST_LOAD_GAME =>
            skse_message!("PostLoadGame"),

        Message::SKSE_SAVE_GAME =>
            skse_message!("SaveGame"),

        Message::SKSE_DELETE_GAME =>
            skse_message!("DeleteGame"),

        Message::SKSE_NEW_GAME =>
            skse_message!("NewGame"),

        other =>
            skse_warning!("Unhandled message: {}", other),
    }
}

// ── Entry point ──────────────────────────────────────────────────────────────
#[no_mangle]
pub fn skse_plugin_rust_entry(skse: &SkseInterface) -> Result<(), ()> {

    use libskyrim::runtime::{CURRENT_VERSION, is_ae, is_se};

    skse_message!("init CommonLib-NG (Rust Edition)");
    skse_message!("Running on Skyrim version: {}", *CURRENT_VERSION);

    if is_ae() {
        skse_message!("Anniversary Edition runtime!");
    } else if is_se() {
        skse_message!("Special Edition (1.5.97) runtime!");
    }

    skse_message!("Registering SKSE listeners...");

    register_listener(Message::SKSE_POST_LOAD,      on_skse_message);
    register_listener(Message::SKSE_POST_POST_LOAD, on_skse_message);
    register_listener(Message::SKSE_INPUT_LOADED,   on_skse_message);
    register_listener(Message::SKSE_DATA_LOADED,    on_skse_message);
    register_listener(Message::SKSE_PRE_LOAD_GAME,  on_skse_message);
    register_listener(Message::SKSE_POST_LOAD_GAME, on_skse_message);
    register_listener(Message::SKSE_SAVE_GAME,      on_skse_message);
    register_listener(Message::SKSE_DELETE_GAME,    on_skse_message);
    register_listener(Message::SKSE_NEW_GAME,       on_skse_message);

    skse_message!("Done.");
    Ok(())
}
