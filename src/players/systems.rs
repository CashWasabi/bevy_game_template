use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::players::components::*;

pub fn update_player(
    mut _commands: Commands,
    mut query: Query<(
        Entity,
        &mut ExternalForce,
        &mut Velocity,
        &mut GravityScale,
        &PlayerDirection,
    )>,
) {
    // TODO(MO): Fix this!
    // let movement = actions.movement.unwrap_or(Vec2::ZERO);
    for (_entity, mut _external_force, mut velocity, mut _gravity_scale, direction) in &mut query {
        // let movement = Vec2::ZERO;
        // let mut force = Vec2::ZERO;
        let speed = Vec2::ZERO;

        // TODO(MO): How should we handle events now?
        // state_machine.handle(&StateEvent::Idle);

        // only use direct vel on x
        velocity.linvel.x = speed.x * direction.0;

        // keep data for next frame
        // let mut context = &mut *state_machine;
        // context.last_frame_speed = speed;
        // context.last_frame_force = force;
    }
}

pub fn update_jump_buffer(time: Res<Time>, mut query: Query<&mut JumpBufferTimer>) {
    for mut timer in &mut query {
        timer.tick(time.delta());

        // TODO(MO): Fix this!
        // if timer.finished() {
        //     (*state_machine).jump_buffer_active = false;
        // } else if !(*state_machine).jump_buffer_active {
        //     (*state_machine).jump_buffer_active = true;
        // }
    }
}

pub fn update_coyote_time(time: Res<Time>, mut query: Query<&mut CoyoteTimer>) {
    for mut timer in &mut query {
        timer.tick(time.delta());

        // if timer.finished() {
        //     (*state_machine).coyote_time_active = false;
        // } else if !(*state_machine).coyote_time_active {
        //     (*state_machine).coyote_time_active = true;
        // }
    }
}

pub fn dash_cooldown(time: Res<Time>, mut query: Query<&mut DashTimer>) {
    for mut timer in &mut query {
        timer.tick(time.delta());
        // (*state_machine).dash_active = if timer.finished() { false } else { true };
    }
}
