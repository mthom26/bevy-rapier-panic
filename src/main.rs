use bevy::prelude::*;
use bevy_rapier2d::{
    physics::{RapierPhysicsPlugin},
    rapier::{
        dynamics::{RigidBodyBuilder},
        geometry::{ColliderBuilder},
    },
    render::RapierRenderPlugin,
};
use rand::{self, Rng};

fn main() {
    App::build()
        .add_default_plugins()
        .add_plugin(RapierPhysicsPlugin)
        .add_plugin(RapierRenderPlugin)
        .add_startup_system(setup.system())
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dComponents::default());

    let ground = RigidBodyBuilder::new_static().translation(0.0, -300.0);
    let ground_col = ColliderBuilder::cuboid(100.0, 10.0);

    commands.spawn((ground, ground_col));

    let mut rng = rand::thread_rng();

    for i in 0..10 {
        for j in 0..8 {
            let x = (i as f32 * 15.0) - 75.0 + rng.gen_range(-1.5, 1.5);
            let y = (j as f32 * 15.0) - 30.0 + rng.gen_range(-1.0, 1.0);
            let body = RigidBodyBuilder::new_dynamic().translation(x, y);

            // Mixing colliders when `simd` and `parallel` features are enabled causes a panic...
            let coll = match rng.gen_bool(0.5) {
                true => ColliderBuilder::ball(rng.gen_range(3.0, 6.0)),
                false => ColliderBuilder::cuboid(rng.gen_range(3.0, 6.0), rng.gen_range(3.0, 6.0)),
            };

            // Using either cuboid or ball (without mixing) is fine...
            // let coll = ColliderBuilder::ball(rng.gen_range(3.0, 6.0));
            // let coll = ColliderBuilder::cuboid(rng.gen_range(3.0, 6.0), rng.gen_range(3.0, 6.0));

            commands.spawn((body, coll));
        }
    }
}
