// TODO(MO): Move this into it's own project
use crate::GameState;
use bevy::prelude::*;
use bevy_polyline::prelude::*;
use std::time::Duration;
// use rand::prelude::*;

pub struct PainterPlugin;

#[derive(Component)]
pub struct Painter;

#[derive(Component)]
pub struct Swappable;

struct LinesSwapConfig {
    timer: Timer,
}

impl Plugin for PainterPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_system_set(
            SystemSet::on_enter(GameState::Playing)
                .with_system(spawn_camera)
                .with_system(setup_line_swapping)
                .with_system(add_lines)
        )
        .add_system_set(
            SystemSet::on_update(GameState::Playing)
                .with_system(swap_lines)
        );
    }
}

/// Configure our line swapping algorithm
fn setup_line_swapping(
    mut commands: Commands,
) {
    commands.insert_resource(LinesSwapConfig {
        // create the repeating timer
        timer: Timer::new(Duration::from_secs(1), true),
    })
}


fn spawn_camera(mut commands: Commands) {
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_xyz(0.0, 0.0, 1000.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..PerspectiveCameraBundle::new_3d()
    });
}

fn add_lines(
    mut commands: Commands,
    mut polyline_materials: ResMut<Assets<PolylineMaterial>>,
    mut polylines: ResMut<Assets<Polyline>>,
) {
    let mult = 25.;
    // 16 is border
    for pos_x in 1..30 {
        for pos_y in 1..30 {
            let step: f32= 25.;
            let transpose = Vec3::new(-400., -400., 0.);
            let origin = Vec3::new(pos_x as f32 * step, pos_y as f32 * step, 0.) + transpose;
            commands.spawn_bundle(PolylineBundle {
                polyline: polylines.add(Polyline {
                    vertices: vec![
                        origin + Vec3::new(-1., -1., 0.) * mult,
                        origin + Vec3::new(1., 1., 0.)* mult
                    ],
                    ..Default::default()
                }),
                material: polyline_materials.add(PolylineMaterial {
                    width: 5.0,
                    // color: Color::hsl(192.0, 0.2, 0.94),
                    color: Color::hsl(184.0, 0.06, 0.53),
                    perspective: false,
                    ..Default::default()
                }),
                ..Default::default()
            }).insert(Swappable);
        }
    }
}

/// Swap lines in set intervals of time
fn swap_lines(
    time: Res<Time>,
    mut config: ResMut<LinesSwapConfig>,
    mut query: Query<&mut Transform, With<Swappable>>,
) {
    config.timer.tick(time.delta());
    if config.timer.finished() {
        for mut transform in query.iter_mut() {

            if rand::random() {
                *transform = Transform {
                    translation: transform.translation,
                    rotation: transform.rotation * Quat::from_rotation_z(std::f32::consts::PI/2.),
                    ..Default::default()
                };
            }
        }
    }
}
