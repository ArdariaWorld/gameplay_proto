use bevy::prelude::*;

use crate::GameState;

use super::player::RespawnPlayerEvent;

#[derive(Component)]
pub struct Button;

#[derive(Component)]
pub struct Menu;

pub struct UiPlugin;
impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(init_ui_system)
            .add_system_set(SystemSet::on_enter(GameState::GameOver).with_system(open_menu_system))
            .add_system_set(
                SystemSet::on_enter(GameState::GameOver).with_system(interaction_button_system),
            )
            .add_system_set(SystemSet::on_exit(GameState::GameOver).with_system(close_menu_system))
            .add_system(interaction_button_system);
    }
}

const BLUE_COLOR: Color = Color::rgb(0.024, 0.12, 0.25);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

fn interaction_button_system(
    mut interaction_query: Query<
        (&Interaction, &mut UiColor, &Children),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
    mut ev_respawn_player: EventWriter<RespawnPlayerEvent>,
) {
    for (interaction, mut color, children) in &mut interaction_query {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Clicked => {
                text.sections[0].value = "Press".to_string();
                *color = PRESSED_BUTTON.into();
                ev_respawn_player.send(RespawnPlayerEvent());
            }
            Interaction::Hovered => {
                text.sections[0].value = "Hover".to_string();
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                text.sections[0].value = "Button".to_string();
                *color = BLUE_COLOR.into();
            }
        }
    }
}

fn init_ui_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn_bundle(NodeBundle {
            visibility: Visibility { is_visible: false },
            style: Style {
                size: Size {
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                },
                margin: UiRect::all(Val::Auto),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            color: Color::Rgba {
                red: 0.,
                green: 0.,
                blue: 0.,
                alpha: 0.9,
            }
            .into(),
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn_bundle(ButtonBundle {
                    style: Style {
                        size: Size::new(Val::Px(350.0), Val::Px(100.0)),
                        // center button
                        margin: UiRect::all(Val::Auto),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    color: BLUE_COLOR.into(),
                    ..default()
                })
                .with_children(|button| {
                    button.spawn_bundle(TextBundle::from_section(
                        format!("Respawn"),
                        TextStyle {
                            font: asset_server.load("fonts/FiraCode-Bold.ttf"),
                            font_size: 44.0,
                            color: Color::GOLD,
                        },
                    ));
                })
                .insert(Button);
        })
        .insert(Menu);
}

fn open_menu_system(mut menu_query: Query<&mut Visibility, With<Menu>>) {
    let mut visibility = match menu_query.get_single_mut() {
        Ok(menu) => menu,
        Err(_) => {
            println!("Cannot find menu");
            return;
        }
    };

    visibility.is_visible = true;
}

fn close_menu_system(mut menu_query: Query<&mut Visibility, With<Menu>>) {
    let mut visibility = match menu_query.get_single_mut() {
        Ok(menu) => menu,
        Err(_) => {
            println!("Cannot find menu");
            return;
        }
    };

    visibility.is_visible = false;
}
