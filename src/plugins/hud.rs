use bevy::{
    prelude::{
        default, App, AssetServer, Color, Commands, Component, EventReader, Plugin, Query, Res,
        ResMut, TextBundle, With,
    },
    text::{Text, TextStyle},
    ui::{PositionType, Style, UiRect, Val},
};

use super::combat::{KillMonsterEvent, MonstersKilled};

pub struct HudPlugin;
impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(init_score_system)
            .add_system(update_score);
    }
}

#[derive(Component)]
struct Score;

fn init_score_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn_bundle(
            TextBundle::from_section(
                "Score: 0",
                TextStyle {
                    font_size: 40.0,
                    color: Color::rgb(0.5, 0.5, 1.0),
                    font: asset_server.load("fonts/FiraCode-Bold.ttf"),
                },
            )
            .with_style(Style {
                position_type: PositionType::Absolute,
                position: UiRect {
                    top: Val::Px(5.0),
                    left: Val::Px(5.0),
                    ..default()
                },
                ..default()
            }),
        )
        .insert(Score);
}

// update the score displayed during the game
fn update_score(
    mut monsters_killed: ResMut<MonstersKilled>,
    mut ev_monster_killed: EventReader<KillMonsterEvent>,
    mut query: Query<&mut Text, With<Score>>,
) {
    let mut text = query.single_mut();
    for _ in ev_monster_killed.iter() {
        monsters_killed.count += 1;
        text.sections[0].value = format!("Score: {}", monsters_killed.count);
    }
}
