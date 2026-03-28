extern crate alloc;

use libskyrim::sdk::plugin::{self, LoadInterface, LogLevel};
use libskyrim::skse::{RuntimeCompatibility, StructCompatibility, VersionIndependence};
use libskyrim::version::Version;
use libskyrim::{skse_error, skse_message};

mod persistence;
mod sample_hooks;

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

#[unsafe(no_mangle)]
pub unsafe fn skse_plugin_rust_entry(skse: &LoadInterface) -> Result<(), ()> {
    plugin::init(skse);
    plugin::set_level(LogLevel::Info);

    skse_message!("RustSKSETemplate starting up");

    if let Err(error) = persistence::register() {
        skse_error!("failed to register serialization model: {}", error);
        return Err(());
    }

    register_lifecycle();

    skse_message!("RustSKSETemplate initialized");
    Ok(())
}

fn register_lifecycle() {
    plugin::on_post_load(|_| {
        skse_message!("SKSE PostLoad");
    });

    plugin::on_input_loaded(|_| {
        skse_message!("Input system initialized");
    });

    plugin::on_data_loaded(|_| {
        skse_message!("Game data loaded");

        plugin::add_task(|| {
            skse_message!("Background task example ran on the SKSE task queue");
        });
        
        sample_hooks::install();
    });

    plugin::on_new_game(|_| {
        persistence::reset_for_new_game();
        skse_message!("New game started");
    });

    plugin::on_post_load_game(|_| {
        persistence::note_runtime_event();
        skse_message!("Save loaded");
    });

    plugin::on_save_game(|_| {
        persistence::increment_save_counter();
        skse_message!("SaveGame callback fired");
    });

    plugin::on_delete_game(|_| {
        skse_message!("DeleteGame callback fired");
    });
}
