use bevy::prelude::*;
use rand::{thread_rng, Rng};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (add_websters, setup))
        .add_systems(Update, move_websters)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn add_websters(mut commands: Commands, assets: Res<AssetServer>) {
    for _ in 0..100 {
        let calm = assets.load("webster_0.png");
        let rand_angle = Quat::from_axis_angle(Vec3::Z, thread_rng().gen::<f32>() * 3.1415 * 2.);
        commands.spawn((
            SpriteBundle {
                texture: calm,
                transform: Transform {
                    rotation: rand_angle,
                    scale: Vec3::splat(0.5),
                    ..default()
                },
                ..default()
            },
            Webstenergy(thread_rng().gen::<f32>() * 4.)
        ));
    }
}

fn move_websters(mut query: Query<(&mut Transform, &Webstenergy)>, time: Res<Time>) {
    for (mut transform, webstenergy) in query.iter_mut() {
        let movement = transform.rotation.mul_vec3(Vec3::Y);
        transform.translation += movement * webstenergy.0 * time.elapsed_seconds();

        if transform.translation.x < -20. {
            transform.rotation = Quat::from_rotation_arc(movement, movement * -Vec3::Y);
        }
    }
}

#[derive(Component)]
struct Webstenergy(f32);
