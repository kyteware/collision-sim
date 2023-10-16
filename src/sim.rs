use bevy::{prelude::*, sprite::collide_aabb::collide};
use bevy_egui::{egui, EguiContexts};
use egui_plot::{Line, Plot};
use rand::{thread_rng, Rng};

use crate::{AppState, Webstimages};

pub struct SimPlugin;

impl Plugin for SimPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(AppState::Sim),
            (setup_plot_data, add_websters, init_start_time),
        )
        .add_systems(
            Update,
            (
                (move_websters, contain_websters).chain(),
                (collide_websters, resolve_collided, plot_collisions).chain(),
                display_levels,
            )
                .run_if(in_state(AppState::Sim)),
        );
    }
}

const SPEED: f32 = 50.;

#[derive(Resource)]
struct LivePlot(Vec<[f64; 2]>);

#[derive(Resource)]
struct StartTime(f64);

#[derive(Component, Deref, DerefMut)]
struct Webstenergy(f32);

#[derive(Component, Deref, DerefMut)]
struct Direction(Vec3);

#[derive(Component)]
struct JustCollided;

fn add_websters(mut commands: Commands, assets: Res<AssetServer>) {
    for _ in 0..100 {
        let calm = assets.load("webster_0.png");
        let rand_angle = Quat::from_axis_angle(Vec3::Z, thread_rng().gen::<f32>() * 3.1415 * 2.);
        commands.spawn((
            SpriteBundle {
                texture: calm,
                transform: Transform {
                    scale: Vec3::splat(20.),
                    translation: Vec3::new(
                        thread_rng().gen_range(-450.0..450.0),
                        thread_rng().gen_range(-450.0..450.0),
                        0.,
                    ),
                    ..default()
                },
                sprite: Sprite {
                    custom_size: Some(Vec2::new(1., 1.)),
                    ..default()
                },
                ..default()
            },
            Webstenergy(thread_rng().gen::<f32>() * 4. + 1.),
            Direction(rand_angle * Vec3::Y),
        ));
    }
}

fn init_start_time(mut commands: Commands, time: Res<Time>) {
    commands.insert_resource(StartTime(time.elapsed_seconds_f64()));
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

// slow af :D
fn collide_websters(mut commands: Commands, query: Query<(&Transform, Entity), With<Webstenergy>>) {
    for (transform, entity) in &query {
        for (other_transform, other_entity) in &query {
            if entity != other_entity
                && collide(
                    transform.translation,
                    Vec2::new(10., 10.),
                    other_transform.translation,
                    Vec2::new(10., 10.),
                )
                .is_some()
            {
                commands.entity(entity).insert(JustCollided);
                commands.entity(other_entity).insert(JustCollided);
            }
        }
    }
}

fn resolve_collided(
    mut commands: Commands,
    query: Query<Entity, With<JustCollided>>,
    mut colplot: ResMut<LivePlot>,
    time: Res<Time>,
    start_time: Res<StartTime>,
) {
    let mut point_added = false;
    for entity in &query {
        if !point_added {
            let old_cnt = colplot.0.last().unwrap()[1];
            colplot
                .0
                .push([time.elapsed_seconds_f64() - start_time.0, old_cnt + 1.]);
            point_added = true;
        } else {
            colplot.0.iter_mut().last().unwrap()[1] += 1.;
        }
        commands.entity(entity).despawn();
    }
}

fn display_levels(mut query: Query<(&mut Handle<Image>, &Webstenergy)>, levels: Res<Webstimages>) {
    for (mut texture, webstenergy) in &mut query {
        let level = webstenergy.0 as usize - 1;
        *texture = levels.0.get(level).unwrap().clone();
    }
}

fn setup_plot_data(mut commands: Commands) {
    commands.insert_resource(LivePlot(vec![[0., 0.]]));
}

fn plot_collisions(mut contexts: EguiContexts, colplot: Res<LivePlot>) {
    egui::Window::new("Number of collisions").show(contexts.ctx_mut(), |ui| {
        let plot = Plot::new("nos");
        // println!("rebuilding graph: {:?}", colplot.0);
        plot.show(ui, |plot_ui| plot_ui.line(Line::new(colplot.0.clone())))
    });
}
