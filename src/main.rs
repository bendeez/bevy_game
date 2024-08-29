use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_systems(Startup, setup)
        .add_systems(Update, update_position)
        .run();
}

#[derive(Component)]
pub struct Velocity {
    value: f32,
}

#[derive(Component)]
pub struct IsDestination(bool);

#[derive(Bundle)]
struct SpaceshipBundle {
    velocity: Velocity,
    rocket: SpriteBundle,
    is_destination: IsDestination,
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(SpaceshipBundle {
        rocket: SpriteBundle {
            transform: Transform::from_scale(Vec3::splat(0.5))
                .with_translation(Vec3::new(0.0, 0.0, 0.0)),
            texture: asset_server.load("1086091.png"),
            ..default()
        },
        velocity: Velocity { value: 1.0 },
        is_destination: IsDestination(false),
    });
    commands.spawn(SpriteBundle {
        transform: Transform::from_scale(Vec3::splat(0.5))
            .with_translation(Vec3::new(50.0, 50.0, 0.0)),
        texture: asset_server.load("png-clipart-asteroids-blue-asteroid-blue-animation.png"),
        ..default()
    });
}

fn update_position(
    mut query: Query<(&mut Velocity, &mut Transform, &mut IsDestination)>,
    time: Res<Time>,
) {
    for (mut velocity, mut transform, mut is_destination) in query.iter_mut() {
        println!("{}", transform.translation.y);
        transform.translation += velocity.value * time.delta_seconds();
        if is_destination.0 {
            if velocity.value > 0.0 {
                velocity.value = -1.0;
                transform.rotation = Quat::from_rotation_z(std::f32::consts::PI);
            } else {
                velocity.value -= 1.0;
            }
        } else {
            if velocity.value < 0.0 {
                velocity.value = 1.0;
                transform.rotation = Quat::IDENTITY;
            } else {
                velocity.value += 1.0;
            }
        }
        if transform.translation.y > 300.0 {
            is_destination.0 = true
        } else if transform.translation.y < -300.0 {
            is_destination.0 = false
        }
    }
}
