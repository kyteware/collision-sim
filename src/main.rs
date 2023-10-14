use bevy::{prelude::*, ecs::query::Has, sprite::collide_aabb::collide};
use rand::{thread_rng, Rng};

const SPEED: f32 = 50.;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (add_websters, setup))
        .add_systems(Update, (move_websters, contain_websters.after(move_websters)))
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
                    scale: Vec3::splat(0.1),
                    ..default()
                },
                ..default()
            },
            Webstenergy(thread_rng().gen::<f32>() * 4. + 1.),
            Direction(rand_angle * Vec3::Y)
        ));
    }
}

fn move_websters(mut query: Query<(&mut Transform, &Webstenergy, &Direction)>, time: Res<Time>) {
    for (mut transform, webstenergy, direction) in query.iter_mut() {
        transform.translation += direction.0 * webstenergy.0 * time.delta_seconds() * SPEED;
    }
}

fn contain_websters(mut query: Query<(&mut Direction, &Transform)>) {
    for (mut direction, transform) in query.iter_mut() {
        if !(-500.0..500.0).contains(&transform.translation.x) {
            direction.x *= -1.;
        }
        if !(-500.0..500.0).contains(&transform.translation.y) {
            direction.y *= -1.;
        }
    }
}

fn collide_websters(mut commands: Commands, mut query: Query<(&Transform, Entity), Has<Direction>>) {
    for (transform, entity) in &query {
        for (other_transform, other_entity) in &query {
            if collide(transform.translation, Vec2::new(25., 25.), transform.translation, Vec2::new(2., 25.)).is_some() {

            }
        }
    }
}

#[derive(Component, Deref, DerefMut)]
struct Webstenergy(f32);

#[derive(Component, Deref, DerefMut)]
struct Direction(Vec3);
