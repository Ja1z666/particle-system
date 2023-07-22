use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

mod particles;

use particles::{Particles, Position};

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, WorldInspectorPlugin::new()))
        .register_type::<Particles>()
        .add_systems(Startup, setup)
        .add_systems(Update, Particles::system)
        .add_systems(Update, update)
        .run();
}

fn update(keyboard: Res<Input<KeyCode>>, mut commands: Commands) {
    if keyboard.just_pressed(KeyCode::Space) {
        commands
            .spawn(SpatialBundle::default())
            .insert(Particles {
                looping: false,
                speed: 300.,
                life_time: 0.2,
                rate: 30,
                size: 6.,
                angle: 360.,
                ..default()
            })
            .insert(Name::new("Particles"));
    }
}

fn setup(mut commands: Commands) {
    commands
        .spawn(SpatialBundle::default())
        .insert(Particles {
            position: Position { x: -200., y: 0. },
            direction: 90.,
            ..default()
        })
        .insert(Name::new("Particles"));
    commands.spawn(Camera2dBundle::default());
}
