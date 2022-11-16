use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Clone, Debug, Default, Bundle, LdtkIntCell)]
pub struct ColliderBundle {
    pub collider: Collider,
    pub rigid_body: RigidBody,
    pub velocity: Velocity,
    pub force: ExternalForce,
    pub rotation_constraints: LockedAxes,
    pub restitution: Restitution,
    pub friction: Friction,
    pub density: ColliderMassProperties,
    pub gravity_scale: GravityScale,
}

#[derive(Clone, Debug, Default, Bundle, LdtkIntCell)]
pub struct SensorBundle {
    pub collider: Collider,
    pub sensor: Sensor,
    pub active_events: ActiveEvents,
    pub rotation_constraints: LockedAxes,
}

impl From<EntityInstance> for ColliderBundle {
    fn from(entity_instance: EntityInstance) -> ColliderBundle {
        let rotation_constraints = LockedAxes::ROTATION_LOCKED;

        match entity_instance.identifier.as_ref() {
            "Player" => ColliderBundle {
                // collider: Collider::cuboid(6.0,14.0),
                collider: Collider::cuboid(4.0, 15.0),
                rigid_body: RigidBody::Dynamic,
                rotation_constraints,
                gravity_scale: GravityScale(10.0),
                friction: Friction {
                    coefficient: 0.0,
                    combine_rule: CoefficientCombineRule::Min,
                },
                ..Default::default()
            },
            _ => ColliderBundle::default(),
        }
    }
}

// For ldtk specificy things
impl From<IntGridCell> for ColliderBundle {
    fn from(_int_grid_cell: IntGridCell) -> ColliderBundle {
        ColliderBundle::default()
    }
}

// For ldtk specificy things
impl From<IntGridCell> for SensorBundle {
    fn from(_int_grid_cell: IntGridCell) -> SensorBundle {
        SensorBundle::default()
    }
}
