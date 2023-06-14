use bevy::prelude::*;


#[derive(Component)]
pub struct CoyoteTime {
    duration: f32,
}

#[derive(Component)]
pub struct RunMovement {
    pub speed: f32,
    pub is_running: bool,
    pub can_run: bool,
}

#[derive(Component)]
pub struct JumpMovement {
    pub force: f32,
    pub is_jumping: bool,
    pub can_jump: bool,
    pub jump_count: i32,
}

#[derive(Component)]
pub struct WallJumpMovement {
    pub speed: f32,
    pub can_jump: bool,
    pub is_jumping: bool,
}

#[derive(Component)]
pub struct CrouchMovement {
    pub speed_reduction: f32,
    pub is_crouching: bool,
    pub can_crouch: bool,
}

#[derive(Component)]
pub struct DashMovement {
    pub force: f32,
    pub cooldown: f32,
    pub can_dash: bool,
    pub is_dashing: bool,
}
