use bevy::prelude::*;

use crate::AppState;

pub struct IntroPlugin;

impl Plugin for IntroPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Intro), setup_ui)
            .add_systems(Update, handle_button.run_if(in_state(AppState::Intro)))
            .add_systems(OnExit(AppState::Intro), destroy_ui);
    }
}

const BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED: Color = Color::rgb(0.5, 0.5, 0.5);

fn setup_ui(mut commands: Commands) {
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
            parent.spawn(TextBundle::from_section(
                "The Collision Simulator",
                TextStyle {
                    font_size: 50.,
                    color: Color::WHITE,
                    ..default()
                },
            ));
            parent
                .spawn(ButtonBundle {
                    style: Style {
                        width: Val::Px(150.0),
                        height: Val::Px(65.0),
                        border: UiRect::all(Val::Px(5.0)),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        margin: UiRect::all(Val::Px(20.)),
                        ..default()
                    },
                    border_color: BorderColor(Color::BLACK),
                    background_color: BUTTON.into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Begin",
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
        (&Interaction, &mut BackgroundColor, &mut BorderColor),
        (Changed<Interaction>, With<Button>),
    >,
    mut next_state: ResMut<NextState<AppState>>,
) {
    for (interaction, mut color, mut border_color) in &mut interaction_query {
        match *interaction {
            Interaction::Hovered => {
                *color = HOVERED.into();
                border_color.0 = Color::WHITE;
            }
            Interaction::None => {
                *color = BUTTON.into();
                border_color.0 = Color::BLACK;
            }
            Interaction::Pressed => {
                next_state.0 = Some(AppState::Controls);
            }
        }
    }
}

fn destroy_ui(mut commands: Commands, query: Query<Entity, With<Node>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}
