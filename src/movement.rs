use crate::ecs_elements::components::{ColliderShape, DeleteWhenOutOfMap, Map};
use bevy::prelude::{Commands, Entity, Query, Transform, With};

pub fn delete_out_of_map_entities(
    entities: Query<(Entity, &ColliderShape, &Transform), With<DeleteWhenOutOfMap>>,
    map: Query<(&ColliderShape, &Transform), With<Map>>, mut commands: Commands,
) {
    'outer: for (e, shape, transform) in entities.iter() {
        let collider_entity = shape.to_collider(transform.translation.truncate());
        for (m_shape, m_transform) in map.iter() {
            let map_collider_entity = m_shape.to_collider(m_transform.translation.truncate());
            if map_collider_entity.intersects(&collider_entity) {
                continue 'outer;
            }
        }
        // no map collision found
        commands.entity(e).try_despawn();
    }
}
