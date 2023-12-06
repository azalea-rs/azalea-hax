#![doc = include_str!("../README.md")]

use azalea::{
    app::{App, Plugin, PreUpdate},
    ecs::prelude::*,
    movement::{handle_knockback, KnockbackEvent, KnockbackType},
    prelude::*,
    Vec3,
};

pub struct HaxPlugin;
impl Plugin for HaxPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PreUpdate,
            anti_knockback
                .after(azalea::packet_handling::game::process_packet_events)
                .before(handle_knockback),
        );
    }
}

pub trait HaxClientExt {
    fn has_anti_knockback(&self) -> bool;
    /// Enable or disable anti-knockback for this client. If enabled, then the server won't be able
    /// to change this client's velocity.
    fn set_anti_knockback(&self, value: bool);
}

impl HaxClientExt for azalea::Client {
    fn has_anti_knockback(&self) -> bool {
        self.get_component::<AntiKnockback>().is_some()
    }

    fn set_anti_knockback(&self, enabled: bool) {
        let mut ecs = self.ecs.lock();
        let mut entity_mut = ecs.entity_mut(self.entity);
        if enabled {
            entity_mut.insert(AntiKnockback);
        } else {
            entity_mut.remove::<AntiKnockback>();
        }
    }
}

/// The component that controls whether we have anti-knockback or not.
#[derive(Component, Clone)]
pub struct AntiKnockback;

fn anti_knockback(
    mut events: EventReader<KnockbackEvent>,
    entity_query: Query<(), With<AntiKnockback>>,
) {
    // bevy please merge this https://github.com/bevyengine/bevy/pull/8051
    // :pleading:
    #[allow(invalid_reference_casting)]
    for event in events
        .read()
        // shhh you didn't see anything
        .map(|e| unsafe { &mut *(e as *const KnockbackEvent as *mut KnockbackEvent) })
    {
        if entity_query.get(event.entity).is_ok() {
            event.knockback = KnockbackType::Add(Vec3::default());
        }
    }
}
