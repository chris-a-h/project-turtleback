use bevy::prelude::*;
use bevy_flycam::prelude::*;
use bevy_rapier3d::prelude::*;


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)

        // PLAYER CONTROLLER plugins
        .add_plugins(PlayerPlugin)

        // RAPIER PHYSICS plugins
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(RapierDebugRenderPlugin::default())




        // SYSTEMS
        .add_systems(Startup, setup_physics)
        .add_systems(Update, console_debugger)


        // CONSOLE DEBUGGING systems
        .add_systems(FixedUpdate, fixed_time_update)
        .insert_resource(Time::<Fixed>::from_seconds(1.0))

        .run();
}

// fn setup_graphics(mut commands: Commands) {
//
// }

fn setup_physics(mut commands: Commands) {
    /* Create the ground. */
    commands
        .spawn(Collider::cuboid(100.0, 0.1, 100.0))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, -2.0, 0.0)));

    /* Create the bouncing ball. */
    commands
        .spawn(RigidBody::Dynamic)
        .insert(Collider::ball(0.5))
        .insert(Restitution::coefficient(0.7))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, 4.0, 0.0)));


    commands
        .spawn(RigidBody::Dynamic)
        .insert(Collider::ball(0.5))
        .insert(TransformBundle::from(Transform::from_xyz(3.0, 3.0, -3.0)))
        .insert(Velocity {
            linvel: Vec3::new(1.0, 2.0, 3.0),
            angvel: Vec3::new(0.2, 0.0, 0.0),
        })
        .insert(GravityScale(0.5))
        .insert(Sleeping::disabled())
        .insert(Ccd::enabled());
}










fn fixed_time_update(mut last_time: Local<f32>, time: Res<Time>) {
    *last_time = time.elapsed_seconds();
}
fn console_debugger(mut last_time: Local<f32>,
                time: Res<Time>,
                positions: Query<&Transform, With<RigidBody>>) {
    info!("### BEGIN DEBUG ###");


    for transform in positions.iter() {
        info!("Ball altitude: {}", transform.translation.y);
    }
    *last_time = time.elapsed_seconds();

    info!("### END DEBUG ###\n");
}