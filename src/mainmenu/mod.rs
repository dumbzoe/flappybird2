use crate::AppState;
use bevy::prelude::*;
pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::MainMenu), setup)
            .add_systems(OnExit(AppState::MainMenu), exit)
            .add_systems(Update, button_system.run_if(in_state(AppState::MainMenu)));
    }
}

const NORMAL_BUTTON: Color = Color::rgb(120. / 255., 120. / 255., 120. / 255.);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.5, 0.5, 0.5);

#[derive(Component)]
struct MainMenuComp;

#[derive(Component)]
enum MenuButtonAction {
    Play,
    Exit,
}

//manages the button
fn button_system(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &MenuButtonAction,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut state: ResMut<NextState<AppState>>,
    mut app_exit_events: ResMut<Events<bevy::app::AppExit>>,
) {
    for (interaction, mut color, mut border_color, menu_action) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = PRESSED_BUTTON.into();
                border_color.0 = Color::RED;
                match menu_action {
                    MenuButtonAction::Play => {
                        state.set(AppState::Playing);
                    }
                    MenuButtonAction::Exit => {
                        app_exit_events.send(bevy::app::AppExit);
                    }
                }
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
                border_color.0 = Color::WHITE;
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
                border_color.0 = Color::BLACK;
            }
        }
    }
}

//This function took me a solid day to do
//Why does UI suck
//sets up the mainmenu
fn setup(mut commands: Commands, asset_server: Res<AssetServer>, mut backdrop: ResMut<ClearColor>) {
    backdrop.0 = Color::GRAY;

    let font: Handle<Font> = asset_server.load("fonts/blocky.ttf");

    let button_style = Style {
        width: Val::Px(250.0),
        height: Val::Px(65.0),
        margin: UiRect::all(Val::Px(20.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };

    let button_text_style = TextStyle {
        font_size: 40.0,
        color: Color::WHITE,
        ..default()
    };

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    display: Display::Flex,
                    ..default()
                },
                ..default()
            },
            MainMenuComp,
        ))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(
                        TextBundle::from_section(
                            "Flappy Bird",
                            TextStyle {
                                font: font,
                                font_size: 100.,
                                ..default()
                            },
                        )
                        .with_style(Style {
                            margin: UiRect::all(Val::Px(50.0)),
                            ..default()
                        }),
                    );

                    parent
                        .spawn((
                            ButtonBundle {
                                style: button_style.clone(),
                                background_color: NORMAL_BUTTON.into(),
                                ..default()
                            },
                            MenuButtonAction::Play,
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                "Play".to_string(),
                                button_text_style.clone(),
                            ));
                        });

                    parent
                        .spawn((
                            ButtonBundle {
                                style: button_style.clone(),
                                background_color: NORMAL_BUTTON.into(),
                                ..default()
                            },
                            MenuButtonAction::Exit,
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                "Exit".to_string(),
                                button_text_style.clone(),
                            ));
                        });
                });
        });
}

fn exit(items: Query<Entity, With<MainMenuComp>>, mut commands: Commands) {
    for entity in items.iter() {
        commands.entity(entity).despawn_descendants().despawn();
    }
}
