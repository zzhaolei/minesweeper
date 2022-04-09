use bevy::{log, prelude::*};
#[cfg(feature = "debug")]
use bevy_inspector_egui::WorldInspectorPlugin;
use board_plugin::{
    resources::{BoardAssets, BoardOptions, SpriteMaterial},
    BoardPlugin,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AppState {
    InGame,
    Out,
    Paused,
    Refresh,
}

fn main() {
    let mut app = App::new();
    app.insert_resource(WindowDescriptor {
        title: "Mine Sweeper!".to_string(),
        width: 700.,
        height: 800.,
        ..Default::default()
    })
    .add_plugins(DefaultPlugins);

    #[cfg(feature = "debug")]
    app.add_plugin(WorldInspectorPlugin::new());

    app.add_startup_system(setup_board)
        .add_state(AppState::Out)
        .add_system(state_handler)
        .add_plugin(BoardPlugin {
            running_state: AppState::InGame,
        });
    app.add_startup_system(camera_setup);

    app.run();
}

fn camera_setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn state_handler(mut state: ResMut<State<AppState>>, keys: Res<Input<KeyCode>>) {
    if state.current() == &AppState::Refresh {
        state.set(AppState::InGame).unwrap();
    }

    keys.get_just_pressed().for_each(|code| match code {
        KeyCode::R => {
            log::info!("重载游戏");
            state.set(AppState::Refresh).unwrap();
        }
        KeyCode::Escape | KeyCode::Space => {
            let current = state.current();
            if current == &AppState::Paused {
                log::info!("继续游戏");
            } else {
                log::info!("暂停游戏");
            }
            if state.current() == &AppState::Paused {
                state.pop().unwrap();
            } else {
                state.push(AppState::Paused).unwrap();
            }
        }
        _ => {}
    });
}

fn setup_board(
    mut commands: Commands,
    mut state: ResMut<State<AppState>>,
    asset_server: Res<AssetServer>,
) {
    commands.insert_resource(BoardOptions {
        map_size: (20, 20),
        bomb_count: 40,
        tile_padding: 1.,
        safe_start: false,
        ..Default::default()
    });
    commands.insert_resource(BoardAssets {
        label: "Default".to_string(),
        board_material: SpriteMaterial {
            color: Color::WHITE,
            ..Default::default()
        },
        tile_material: SpriteMaterial {
            color: Color::DARK_GRAY,
            ..Default::default()
        },
        covered_tile_material: SpriteMaterial {
            color: Color::GRAY,
            ..Default::default()
        },
        bomb_counter_font: asset_server.load("fonts/pixeled.ttf"),
        bomb_counter_colors: BoardAssets::default_colors(),
        flag_material: SpriteMaterial {
            texture: asset_server.load("sprites/flag.png"),
            color: Color::WHITE,
        },
        bomb_material: SpriteMaterial {
            texture: asset_server.load("sprites/bomb.png"),
            color: Color::GRAY,
        },
    });
    state.set(AppState::InGame).unwrap();
}
