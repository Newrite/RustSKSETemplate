use core::sync::atomic::{AtomicBool, Ordering};

use libskyrim::re::{Actor, HitData, InventoryEntryData, PlayerCharacter, TESBoundObject, TESForm};
use libskyrim::relocation::{RelocationID, VariantOffset, skyrim_cast_mut};
use libskyrim::sdk::hooks::{self, Original, Resolved};
use libskyrim::sdk::plugin;
use libskyrim::{skse_error, skse_message};

static PLAYER_UPDATE_LOGGED: AtomicBool = AtomicBool::new(false);

#[hooks::call_hook(
    target = RelocationID::new(37673, 38627),
    offset = VariantOffset::new(0x3C0, 0x4A80, 0x3C0),
    size = 5,
    guard = hooks::guards::original()
)]
fn weapon_hit(
    original: Original<fn(&Actor, &mut HitData)>,
    target: &Actor,
    hit_data: &mut HitData,
) {
    skse_message!(
        "Total Damage {} to Actor {:p}",
        hit_data.total_damage,
        target.get_display_full_name()
    );
    original.call(target, hit_data);
}

#[hooks::call_hook(
    target = RelocationID::new(37673, 38627),
    offset = VariantOffset::new(0x185, 0x194, 0x185),
    size = 5,
    guard = hooks::guards::original()
)]
fn apply_attack_spells_call(
    original: Original<fn(Resolved<Actor>, Option<&mut InventoryEntryData>, bool, Resolved<Actor>)>,
    actor: Resolved<Actor>,
    entry_data: Option<&mut InventoryEntryData>,
    is_left: bool,
    target: Resolved<Actor>,
) {
    if let Some(entry_data) = entry_data {
        original.call(actor, Some(entry_data), is_left, target);
        return;
    }

    let Some(form) = TESForm::lookup_by_id(0x1F4) else {
        skse_error!("example hook: failed to find default iron sword form 0x1F4");
        original.call(actor, None, is_left, target);
        return;
    };

    let Some(object) = (unsafe { skyrim_cast_mut::<TESForm, TESBoundObject>(&mut *form) }) else {
        skse_error!("example hook: form 0x1F4 is not a TESBoundObject");
        original.call(actor, None, is_left, target);
        return;
    };

    let mut temp_data = InventoryEntryData::new(object, 1);
    skse_message!("example call hook injected fallback InventoryEntryData");
    original.call(actor, Some(&mut temp_data), is_left, target);
}

#[hooks::vtable_hook(vtable = PlayerCharacter::VTABLE[0], offset = Actor::VFUNC_UPDATE, guard = hooks::guards::original())]
fn player_character_update(
    original: Original<fn(&mut PlayerCharacter, f32)>,
    player: &mut PlayerCharacter,
    delta: f32,
) {
    if !PLAYER_UPDATE_LOGGED.swap(true, Ordering::Relaxed) {
        skse_message!("example PlayerCharacter::Update vtable hook is active");
    }

    original.call(player, delta);
}

pub fn install() {
    plugin::alloc_trampoline(256);

    if let Err(error) = hooks::install_all![
        self::apply_attack_spells_call_hook,
        self::player_character_update_hook,
        self::weapon_hit_hook
    ] {
        skse_error!("failed to install example hooks: {}", error);
    } else {
        skse_message!("example hooks installed");
    }
}
