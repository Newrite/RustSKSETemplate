use libskyrim::sdk::plugin::serialization::{self, Cosave, Model, RegistrationError, Schema};
use libskyrim::skse_message;

#[derive(Debug, Default, Cosave)]
pub struct SaveState {
    pub save_counter: u32,
    pub runtime_events: u32,
}

impl Model for SaveState {
    const UNIQUE_ID: serialization::UniqueId = serialization::unique_id!("RSTT");

    fn schema(mut schema: &mut Schema<Self>) {
        serialization::schema_fields!(schema, {
            serialization::record_id!("SAVE") => 1 => save_counter,
            serialization::record_id!("EVNT") => 1 => runtime_events,
        });
    }

    fn on_revert(&mut self) {
        skse_message!("Save state reverted");
        *self = Self::default();
    }
}

pub fn register() -> Result<(), RegistrationError> {
    skse_message!("Registering save state model");
    serialization::register_model::<SaveState>()
}

pub fn increment_save_counter() {
    let _ = serialization::with_model_mut::<SaveState, _>(|state| {
        skse_message!("Incrementing save counter");
        state.save_counter = state.save_counter.saturating_add(1);
    });
}

pub fn note_runtime_event() {
    let _ = serialization::with_model_mut::<SaveState, _>(|state| {
        skse_message!("Note runtime event");
        state.runtime_events = state.runtime_events.saturating_add(1);
    });
}

pub fn reset_for_new_game() {
    let _ = serialization::with_model_mut::<SaveState, _>(|state| {
        skse_message!("Resetting for new game");
        *state = SaveState::default();
    });
}
