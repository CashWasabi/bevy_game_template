use bevy::app::AppExit;
use bevy::prelude::*;

use crate::GameState;
use crate::loading::FontAssets;

use super::styles::{
    CLICKED_BUTTON_COLOR,
    HOVERED_BUTTON_COLOR,
    NORMAL_BUTTON_COLOR,
    MAIN_MENU_STYLE,
    BUTTON_STYLE,
    TITLE_STYLE,
    get_title_text_style,
    get_button_text_style,

};

use super::components::{
    PlayButton,
    QuitButton,
    MainMenu,
};

pub fn click_play_button(
    mut state: ResMut<NextState<GameState>>,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<PlayButton>),
    >,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                *color = CLICKED_BUTTON_COLOR.into();
                state.set(GameState::Playing);
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON_COLOR.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON_COLOR.into();
            }
        }
    }
}

pub fn click_exit_button(
    mut app_exit_event_writer: EventWriter<AppExit>,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<QuitButton>),
    >,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                *color = CLICKED_BUTTON_COLOR.into();
                app_exit_event_writer.send(AppExit);
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON_COLOR.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON_COLOR.into();
            }
        }
    }
}

pub fn spawn_main_menu(mut commands: Commands, font_assets: Res<FontAssets>) {
    commands.spawn(Camera2dBundle::default());
    build_main_menu(&mut commands, &font_assets);
}

pub fn despawn_main_menu(mut commands: Commands, main_menu_query: Query<Entity, With<MainMenu>>) {
    if let Ok(main_menu_entity) = main_menu_query.get_single() {
        commands.entity(main_menu_entity).despawn_recursive();
    }
}

pub fn build_main_menu(commands: &mut Commands, font_assets: &Res<FontAssets>) -> Entity {
    let main_menu_entity = commands
        .spawn((
            NodeBundle {
                style: MAIN_MENU_STYLE,
                ..default()
            },
            MainMenu {},
        ))
        .with_children(|parent| {
            // === Title ===
            parent
                .spawn(NodeBundle {
                    style: TITLE_STYLE,
                    ..default()
                })
                .with_children(|parent| {
                    // Text
                    parent.spawn(TextBundle {
                        text: Text {
                            sections: vec![TextSection::new(
                                "Kaginawa",
                                get_title_text_style(&font_assets),
                            )],
                            alignment: TextAlignment::Center,
                            ..default()
                        },
                        ..default()
                    });
                });
            // === Play Button ===
            parent
                .spawn((
                    ButtonBundle {
                        style: BUTTON_STYLE,
                        background_color: NORMAL_BUTTON_COLOR.into(),
                        ..default()
                    },
                    PlayButton {},
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle {
                        text: Text {
                            sections: vec![TextSection::new(
                                "Play",
                                get_button_text_style(&font_assets),
                            )],
                            alignment: TextAlignment::Center,
                            ..default()
                        },
                        ..default()
                    });
                });
            // === Quit Button ===
            parent
                .spawn((
                    ButtonBundle {
                        style: BUTTON_STYLE,
                        background_color: NORMAL_BUTTON_COLOR.into(),
                        ..default()
                    },
                    QuitButton {},
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle {
                        text: Text {
                            sections: vec![TextSection::new(
                                "Quit",
                                get_button_text_style(&font_assets),
                            )],
                            alignment: TextAlignment::Center,
                            ..default()
                        },
                        ..default()
                    });
                });
        })
        .id();

    main_menu_entity
}
