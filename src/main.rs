use bevy::prelude::*;
use bevy_egui::EguiPlugin;

use collision_sim::{AppState, SimPlugin, Webstimages};

fn main() {
    App::new()
        .add_state::<AppState>()
        .add_systems(Startup, (setup_camera, load_webstimages))
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin)
        .add_plugins(SimPlugin)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn load_webstimages(mut commands: Commands, assets: Res<AssetServer>) {
    let levels = [
        assets.load("webster_0.png"),
        assets.load("webster_1.png"),
        assets.load("webster_2.png"),
        assets.load("webster_3.png"),
    ];
    commands.insert_resource(Webstimages(levels));
}
