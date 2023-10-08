use azalea::{
    app::{App, Plugin, PreUpdate},
    ecs::prelude::*,
    entity::indexing::EntityIdIndex,
    packet_handling::game::PacketEvent,
    prelude::*,
    protocol::packets::game::ClientboundGamePacket,
    world::MinecraftEntityId,
};

pub struct HaxPlugin;
impl Plugin for HaxPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PreUpdate,
            anti_knockback
                .before(azalea::packet_handling::game::process_packet_events)
                .after(azalea::packet_handling::game::send_packet_events),
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
    events: ResMut<Events<PacketEvent>>,
    player_query: Query<&EntityIdIndex>,
    entity_query: Query<(), With<AntiKnockback>>,
) {
    // bevy please merge this https://github.com/bevyengine/bevy/pull/8051
    // :pleading:
    #[allow(invalid_reference_casting)]
    for event in events
        .iter_current_update_events()
        // you didn't see anything
        .map(|e| unsafe { &mut *(e as *const PacketEvent as *mut PacketEvent) })
    {
        if let ClientboundGamePacket::SetEntityMotion(p) = &mut event.packet {
            let Ok(entity_id_index) = player_query.get(event.entity) else {
                continue;
            };
            let Some(ecs_entity) = entity_id_index.get(&MinecraftEntityId(p.id)) else {
                continue;
            };
            // only apply if the entity has the AntiKnockback component
            if entity_query.get(ecs_entity).is_ok() {
                (p.xa, p.ya, p.za) = (0, 0, 0);
            }
        } else if let ClientboundGamePacket::Explode(p) = &mut event.packet {
            // only apply if the entity has the AntiKnockback component
            if entity_query.get(event.entity).is_ok() {
                (p.knockback_x, p.knockback_y, p.knockback_z) = (0., 0., 0.);
            }
        }
    }
}
