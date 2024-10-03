use bevy::prelude::*;
use bevy::text::BreakLineOn;
use crate::main_menu::components::{MainMenu, PlayButton, QuitButton};
use crate::main_menu::styles::{get_button_text_style, get_title_text_style, get_button_style, get_main_menu_style, NORMAL_BUTTON_COLOR, get_title_style, get_image_style};

pub fn spawn_main_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    build_main_menu(&mut commands, &asset_server);
}

pub fn despawn_main_menu(
    mut commands: Commands,
    main_menu_query: Query<Entity, With<MainMenu>>,
) {
    if let Ok(main_menu_entity) = main_menu_query.get_single() {
        commands.entity(main_menu_entity).despawn_recursive();
    }
}

pub fn build_main_menu(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
) -> Entity {
    let main_menu_entity = commands.spawn(
        (
            NodeBundle {
                style: get_main_menu_style(),
                ..default()
            },
            MainMenu {}
        )
    )
        .with_children(|parent| {
            //     Title
            parent.spawn(
                NodeBundle {
                    style: get_title_style(),
                    ..default()
                }
            )
                .with_children(|parent| {
                    //     Image 1
                    parent.spawn(
                        ImageBundle {
                            style: get_image_style(),
                            image: asset_server.load("sprites/ball_blue_large.png").into(),
                            ..default()
                        }
                    );
                    //     Text
                    parent.spawn(
                        TextBundle {
                            text: Text {
                                sections: vec![
                                    TextSection::new(
                                        "Bevy Ball Game",
                                        get_title_text_style(&asset_server)
                                    )
                                ],
                                justify: JustifyText::Center,
                                linebreak_behavior: BreakLineOn::NoWrap,
                                ..default()
                            },
                            ..default()
                        }
                    );
                    //     Image 2
                    parent.spawn(
                        ImageBundle {
                            style: get_image_style(),
                            image: asset_server.load("sprites/ball_red_large.png").into(),
                            ..default()
                        }
                    );
                });
            //     Play Button
            parent.spawn(
                (
                    ButtonBundle {
                        style: get_button_style(),
                        background_color: NORMAL_BUTTON_COLOR.into(),
                        ..default()
                    },
                    PlayButton {},
                )
            )
                .with_children(|parent| {
                    parent.spawn(
                        TextBundle {
                            text: Text {
                                sections: vec![
                                    TextSection::new(
                                        "Play",
                                        get_button_text_style(&asset_server)
                                    )
                                ],
                                justify: JustifyText::Center,
                                ..default()
                            },
                            ..default()
                        }
                    );
                })
            ;
            //     Quit Button
            parent.spawn(
                (
                    ButtonBundle {
                        style: get_button_style(),
                        background_color: NORMAL_BUTTON_COLOR.into(),
                        ..default()
                    },
                    QuitButton {},
                )
            )
                .with_children(|parent| {
                    parent.spawn(
                        TextBundle {
                            text: Text {
                                sections: vec![
                                    TextSection::new(
                                        "Quit",
                                        get_button_text_style(&asset_server)
                                    )
                                ],
                                justify: JustifyText::Center,
                                ..default()
                            },
                            ..default()
                        }
                    );
                })
            ;
        })
        .id();

    main_menu_entity
}
