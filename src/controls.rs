use bevy::prelude::*;

use crate::AppState;

pub struct ControlsPlugin;

impl Plugin for ControlsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Controls), (setup_ui, init_controls))
            .add_systems(Update, handle_button.run_if(in_state(AppState::Controls)))
            .add_systems(OnExit(AppState::Controls), destroy_ui);
    }
}

const BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const BUTTON_ON: Color = Color::rgb(0.15, 0.5, 0.15);

#[derive(Resource)]
pub struct SimControls {
    pub pressure: bool,
    pub temp: bool,
}

fn init_controls(mut commands: Commands) {
    commands.insert_resource(SimControls {
        pressure: false,
        temp: false,
    });
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
        // base
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
            // temp
            parent
                .spawn(ButtonBundle {
                    style: button_style.clone(),
                    border_color: BorderColor(Color::BLACK),
                    background_color: BUTTON.into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "^Temperature^",
                        TextStyle {
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                            ..default()
                        },
                    ));
                });
                // pressure
                parent
                    .spawn(ButtonBundle {
                        style: button_style.clone(),
                        border_color: BorderColor(Color::BLACK),
                        background_color: BUTTON.into(),
                        ..default()
                    })
                    .with_children(|parent| {
                        parent.spawn(TextBundle::from_section(
                            "^Pressure^",
                            TextStyle {
                                font_size: 40.0,
                                color: Color::rgb(0.9, 0.9, 0.9),
                                ..default()
                            },
                        ));
                    });
            // run
            parent
                .spawn(ButtonBundle {
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

fn handle_button(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &mut BorderColor, &Children),
        (Changed<Interaction>, With<Button>),
    >,
    text_query: Query<&Text>,
    mut next_state: ResMut<NextState<AppState>>,
    mut controls: ResMut<SimControls>
) {
    for (interaction, mut color, mut border_color, children) in &mut interaction_query {
        let text = &text_query.get(children[0]).unwrap().sections[0].value;
        match *interaction {
            Interaction::Hovered => {
                border_color.0 = Color::WHITE;
            }
            Interaction::None => {
                border_color.0 = Color::BLACK;
            }
            Interaction::Pressed => {
                if text == "Run" {
                    next_state.0 = Some(AppState::Sim);
                }
                else if text == "^Temperature^" {
                    controls.temp = !controls.temp;
                    if controls.temp {
                        *color = BUTTON_ON.into();
                    } else {
                        *color = BUTTON.into();
                    }
                }
                else if text == "^Pressure^" {
                    controls.pressure = !controls.pressure;
                    if controls.pressure {
                        *color = BUTTON_ON.into();
                    } else {
                        *color = BUTTON.into();
                    }
                }
            }
        }
    }
}

fn destroy_ui(mut commands: Commands, query: Query<Entity, With<Node>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}