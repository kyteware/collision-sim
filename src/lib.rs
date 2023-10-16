mod intro;
mod sim;

pub use sim::SimPlugin;

use bevy::prelude::*;

#[derive(Debug, Default, Clone, Eq, PartialEq, Hash, States)]
pub enum AppState {
    #[default]
    Intro,
    Controls,
    Sim,
}

#[derive(Resource)]
pub struct Webstimages(pub [Handle<Image>; 4]);
