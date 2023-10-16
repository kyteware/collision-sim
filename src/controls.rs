use bevy::prelude::*;

use crate::AppState;

pub struct ControlsPlugin;

impl Plugin for ControlsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Controls), (setup_ui, init_controls));
    }
}

const BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED: Color = Color::rgb(0.5, 0.5, 0.5);

#[derive(Resource)]
pub struct SimControls {
    pub pressure: bool,
    pub temp: bool
}

fn init_controls(mut commands: Commands) {
    commands.insert_resource(SimControls { pressure: false, temp: false });
}

fn setup_ui(mut commands: Commands) {
    let button_style = Style {
        width: Val::Auto,
        height: Val::Px(65.0),
        border: UiRect::all(Val::Px(5.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };
    
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(90.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(ButtonBundle {
                style: button_style.clone(),
                border_color: BorderColor(Color::BLACK),
                background_color: BUTTON.into(),
                ..default()
            })
            .with_children(|parent| {
                parent.spawn(TextBundle::from_section(
                    "Increase Temperature",
                    TextStyle {
                        font_size: 40.0,
                        color: Color::rgb(0.9, 0.9, 0.9),
                        ..default()
                    },
                ));
            });
            parent.spawn(ButtonBundle {
                    style: button_style.clone(),
                    border_color: BorderColor(Color::BLACK),
                    background_color: BUTTON.into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Run",
                        TextStyle {
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                            ..default()
                        },
                    ));
                });
        });
}