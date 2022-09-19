// New Way
use bevy::{prelude::*, render::texture::ImageSettings};
use benimator::*;

use crate::{loading::TextureAssets, GameState};

pub struct InternalAnimationPlugin;

// Create the animation component
// Note: you may make the animation an asset instead of a component
#[derive(Component, Deref)]
pub struct Animation(benimator::Animation);

// Create the player component
#[derive(Default, Component, Deref, DerefMut)]
pub struct AnimationState(benimator::State);

// This plugin manages every animation in the game
impl Plugin for InternalAnimationPlugin {
    fn build(&self, app: &mut App) {
        app
        // TODO(MO): Do we want it to be a resource?
        // .init_resource::<PlayerAnimations>()
        .insert_resource(ImageSettings::default_nearest())
        .init_resource::<PlayerAnimations>()
        // .add_system_set(
        //     SystemSet::on_enter(GameState::Playing)
        //         .with_system(create_player_animations)
        // )
        .add_system_set(
            SystemSet::on_update(GameState::Playing)
                .with_system(animate)
        )
        ;
    }
}


// Create a resource to store handles of the animations
#[derive(Component)]
pub struct PlayerAnimations {
    pub idle: Animation,
    pub run: Animation,
    pub jump: Animation,
    pub dash: Animation,
    pub fall: Animation,
    pub crouch: Animation,
    pub attack: Animation,
}

impl FromWorld for PlayerAnimations {
    fn from_world(world: &mut World) -> Self {
        // You have full access to anything in the ECS from here.
        // For instance, you can mutate other resources:
        let textures = world.get_resource::<TextureAssets>().unwrap();
        let mut texture_atlas = world.get_resource_mut::<Assets<TextureAtlas>>().unwrap();

        PlayerAnimations {
            idle: Animation(benimator::Animation::from_indices(
                0..=3,
                FrameRate::from_fps(12.0),
            )),
            run: Animation(benimator::Animation::from_indices(
                8..=13,
                FrameRate::from_fps(12.0),
            )),
            jump: Animation(benimator::Animation::from_indices(
                14..=23,
                FrameRate::from_fps(12.0),
            )),
            dash: Animation(benimator::Animation::from_indices(
                24..=28,
                FrameRate::from_fps(12.0),
            )),
            fall: Animation(benimator::Animation::from_indices(
                22..=23,
                FrameRate::from_fps(12.0),
            )),
            crouch: Animation(benimator::Animation::from_indices(
                4..=7,
                FrameRate::from_fps(12.0),
            )),
            attack: Animation(benimator::Animation::from_indices(
                55..=57,
                FrameRate::from_fps(12.0),
            )),
        }
    }
}

// fn create_player_animations(
//     textures: Res<TextureAssets>,
//     mut texture_atlas: ResMut<Assets<TextureAtlas>>,
// ) {
//     let player_position = Vec3::ZERO;
//     let texture_atlas = texture_atlas.add(
//         TextureAtlas::from_grid(
//             textures.texture_player.clone().into(),
//             Vec2::new(1.0, 1.0),
//             7, 11
//         )

//     );
//     let player_sprite = SpriteSheetBundle {
//         texture_atlas: texture_atlas,
//         transform: Transform {
//             translation: player_position,
//             ..Transform::default()
//         },
//         ..Default::default()
//     };

//     let player_animations = PlayerAnimations {
//         idle: Animation(benimator::Animation::from_indices(
//             0..=3,
//             FrameRate::from_fps(12.0),
//         )),
//         run: Animation(benimator::Animation::from_indices(
//             8..=13,
//             FrameRate::from_fps(12.0),
//         )),
//         jump: Animation(benimator::Animation::from_indices(
//             14..=23,
//             FrameRate::from_fps(12.0),
//         )),
//         dash: Animation(benimator::Animation::from_indices(
//             24..=28,
//             FrameRate::from_fps(12.0),
//         )),
//         fall: Animation(benimator::Animation::from_indices(
//             22..=23,
//             FrameRate::from_fps(12.0),
//         )),
//         crouch: Animation(benimator::Animation::from_indices(
//             4..=7,
//             FrameRate::from_fps(12.0),
//         )),
//         attack: Animation(benimator::Animation::from_indices(
//             55..=57,
//             FrameRate::from_fps(12.0),
//         )),
//     };
// }

fn animate(
    time: Res<Time>,
    mut query: Query<(&mut AnimationState, &mut TextureAtlasSprite, &Animation)>,
) {
    for (mut player, mut texture, animation) in query.iter_mut() {
        // Update the state
        player.update(animation, time.delta());

        // Update the texture atlas
        texture.index = player.frame_index();
    };
}

// TODO(MO): Remove this since it is only meant as a reference
// fn spawn(
//     mut commands: Commands,
//     asset_server: Res<AssetServer>,
//     mut textures: ResMut<Assets<TextureAtlas>>,
// ) {
//     // Don't forget the camera ;-)
//     commands.spawn_bundle(Camera2dBundle::default());
//
//     // Create an animation
//     let animation = Animation(benimator::Animation::from_indices(
//         0..=4,
//         FrameRate::from_fps(12.0),
//     ));
//
//     commands
//         // Spawn a bevy sprite-sheet
//         .spawn_bundle(SpriteSheetBundle {
//             texture_atlas: textures.add(TextureAtlas::from_grid(
//                 asset_server.load("coin.png"),
//                 Vec2::new(16.0, 16.0),
//                 5,
//                 1,
//             )),
//             transform: Transform::from_scale(Vec3::splat(10.0)),
//             ..Default::default()
//         })
//         // Insert the animation
//         .insert(animation)
//         // Insert the state
//         .insert(AnimationState::default());
// }
