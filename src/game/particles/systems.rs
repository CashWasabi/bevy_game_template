use bevy::prelude::*;
use bevy_hanabi::prelude::*;


fn setup(mut effects: ResMut<Assets<EffectAsset>>) {
    // Define a color gradient from red to transparent black
    let mut gradient = Gradient::new();
    gradient.add_key(0.0, Vec4::new(1., 0., 0., 1.)); // Red
    gradient.add_key(1.0, Vec4::ZERO); // Transparent black

    // Create the effect asset
    let _effect = effects.add(EffectAsset {
            name: "DemoEffect".to_string(),
            // Maximum number of particles alive at a time
            capacity: 32768,
            // Spawn at a rate of 5 particles per second
            spawner: Spawner::rate(5.0.into()),
            ..Default::default()
        }
        // On spawn, randomly initialize the position of the particle
        // to be over the surface of a sphere of radius 2 units.
        .init(InitPositionSphereModifier {
            center: Vec3::ZERO,
            radius: 2.,
            dimension: ShapeDimension::Surface,
        })
        // Also initialize a radial initial velocity to 6 units/sec
        // away from the (same) sphere center.
        .init(InitVelocitySphereModifier {
            center: Vec3::ZERO,
            speed: 6.0.into(),
        })
        // Also initialize the total lifetime of the particle, that is
        // the time for which it's simulated and rendered. This modifier
        // is mandatory, otherwise the particles won't show up.
        .init(InitLifetimeModifier { lifetime: 10_f32.into() })
        // Every frame, add a gravity-like acceleration downward
        .update(AccelModifier::constant(Vec3::new(0., -3., 0.)))
        // Render the particles with a color gradient over their
        // lifetime. This maps the gradient key 0 to the particle spawn
        // time, and the gradient key 1 to the particle death (here, 10s).
        .render(ColorOverLifetimeModifier { gradient })
    );
}
