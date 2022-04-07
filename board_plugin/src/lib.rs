use bevy::ecs::schedule::StateData;
use bevy::prelude::Plugin;
use bevy::utils::AHashExt;
use bevy::utils::HashMap;
use resources::tile::Tile;
use resources::BoardAssets;
use resources::BoardOptions;

pub mod components;
pub mod resources;

mod bounds;
mod events;
mod systems;

use crate::bounds::Bounds2;
use crate::components::Bomb;
use crate::components::BombNeighbor;
use crate::components::Coordinates;
use crate::components::Uncover;
use crate::events::BoardCompletedEvent;
use crate::events::BombExplosionEvent;
use crate::events::TileMarkEvent;
use crate::events::TileTriggerEvent;
use crate::resources::tile_map::TileMap;
use crate::resources::BoardPosition;
use crate::resources::TileSize;
use bevy::log;
use bevy::prelude::*;

use resources::board::Board;

use bevy::math::Vec3Swizzles;

pub struct BoardPlugin<T> {
    pub running_state: T,
}

impl<T: StateData> Plugin for BoardPlugin<T> {
    fn build(&self, app: &mut App) {
        // on_enter 当进入栈的时候，执行
        // on_update 处理输入，当状态处于活动状态的时候，触发对应的处理事件，允许暂停状态
        // on_in_stack_update uncover系统不应该暂停，所以如果状态在栈中，则不管是否处于活动状态，都要运行
        app.add_system_set(
            SystemSet::on_enter(self.running_state.clone()).with_system(Self::create_board),
        )
        .add_system_set(
            SystemSet::on_update(self.running_state.clone())
                .with_system(systems::input::input_handling)
                .with_system(systems::uncover::trigger_event_handler),
        )
        .add_system_set(
            SystemSet::on_in_stack_update(self.running_state.clone())
                .with_system(systems::uncover::uncover_tiles)
                .with_system(systems::mark::mark_tiles),
        )
        .add_system_set(
            SystemSet::on_exit(self.running_state.clone()).with_system(Self::cleanup_board),
        )
        .add_event::<TileTriggerEvent>()
        .add_event::<TileMarkEvent>()
        .add_event::<BombExplosionEvent>()
        .add_event::<BoardCompletedEvent>();

        log::info!("面板已加载");
        #[cfg(feature = "debug")]
        {
            use crate::components::Uncover;
            use bevy_inspector_egui::RegisterInspectable;
            app.register_inspectable::<Coordinates>();
            app.register_inspectable::<BombNeighbor>();
            app.register_inspectable::<Bomb>();
            app.register_inspectable::<Uncover>();
        }
    }
}

impl<T> BoardPlugin<T> {
    /// bevy 会自动调用此函数生成窗口
    pub fn create_board(
        mut commands: Commands,
        board_options: Option<Res<BoardOptions>>,
        board_assets: Res<BoardAssets>,
        window: Res<WindowDescriptor>,
    ) {
        let options = match board_options {
            None => BoardOptions::default(),
            Some(o) => o.clone(),
        };
        // 创建空的棋盘，并且在其上放置棋子
        let mut tile_map = TileMap::empty(options.map_size.0, options.map_size.1);
        tile_map.set_bombs(options.bomb_count);

        // 如果编译时指定 --features debug，则会执行这一句
        #[cfg(feature = "debug")]
        log::info!("棋盘: {}", tile_map.console_output());

        let tile_size = match options.tile_size {
            TileSize::Fixed(v) => v,
            TileSize::Adaptive { min, max } => Self::adaptative_tile_size(
                window,
                (min, max),
                (tile_map.width(), tile_map.height()),
            ),
        };

        // 计算面板的大小
        let board_size = Vec2::new(
            tile_map.width() as f32 * tile_size,
            tile_map.height() as f32 * tile_size,
        );
        log::info!("面板尺寸: {}", board_size);
        let board_position = match options.position {
            BoardPosition::Centered { offset } => {
                Vec3::new(-(board_size.x / 2.0), -(board_size.y / 2.0), 0.0) + offset
            }
            BoardPosition::Custom(p) => p,
        };

        let mut covered_tiles =
            HashMap::with_capacity((tile_map.width() * tile_map.height()).into());
        let mut safe_start = None;

        let board_entity = commands
            .spawn()
            .insert(Name::new("Board"))
            .insert(Transform::from_translation(board_position))
            .insert(GlobalTransform::default())
            .with_children(|parent| {
                // 在窗口上划出棋子，居中放置
                parent
                    .spawn_bundle(SpriteBundle {
                        sprite: Sprite {
                            color: board_assets.board_material.color,
                            custom_size: Some(board_size),
                            ..Default::default()
                        },
                        texture: board_assets.board_material.texture.clone(),
                        transform: Transform::from_xyz(board_size.x / 2.0, board_size.y / 2.0, 0.0),
                        ..Default::default()
                    })
                    .insert(Name::new("Background"));

                Self::spawn_tiles(
                    parent,
                    &tile_map,
                    tile_size,
                    options.tile_padding,
                    &board_assets,
                    &mut covered_tiles,
                    &mut safe_start,
                )
            })
            .id();

        // 如果开启了这个选项，则使用spawn_tiles函数改过的safe_start
        // 添加Uncover组件，用于揭开这个空白棋子
        if options.safe_start {
            if let Some(entity) = safe_start {
                commands.entity(entity).insert(Uncover);
            }
        }

        commands.insert_resource(Board {
            tile_map,
            bounds: Bounds2 {
                position: board_position.xy(),
                size: board_size,
            },
            tile_size,
            covered_tiles,
            entity: board_entity,
            marked_tiles: Vec::new(),
        })
    }

    ///
    fn adaptative_tile_size(
        window: Res<WindowDescriptor>,
        (min, max): (f32, f32),
        (width, height): (u16, u16),
    ) -> f32 {
        let max_width = window.width / width as f32;
        let max_height = window.height / height as f32;
        max_width.min(max_height).clamp(min, max)
    }

    #[allow(clippy::too_many_arguments)]
    fn spawn_tiles(
        parent: &mut ChildBuilder,
        tile_map: &TileMap,
        size: f32,
        padding: f32,
        board_assets: &BoardAssets,
        covered_tiles: &mut HashMap<Coordinates, Entity>,
        safe_start_entity: &mut Option<Entity>,
    ) {
        for (y, line) in tile_map.iter().enumerate() {
            for (x, tile) in line.iter().enumerate() {
                let coordinates = Coordinates {
                    x: x as u16,
                    y: y as u16,
                };
                let mut cmd = parent.spawn();

                cmd.insert_bundle(SpriteBundle {
                    sprite: Sprite {
                        color: board_assets.tile_material.color,
                        custom_size: Some(Vec2::splat(size - padding as f32)),
                        ..Default::default()
                    },
                    texture: board_assets.tile_material.texture.clone(),
                    transform: Transform::from_xyz(
                        (x as f32 * size) + size / 2.0,
                        (y as f32 * size) + size / 2.0,
                        3.0,
                    ),
                    ..Default::default()
                })
                .insert(Name::new(format!("Tile ({}, {})", x, y)))
                .insert(coordinates);

                // 覆盖棋子
                cmd.with_children(|parent| {
                    let mut cmd = parent.spawn();
                    let entity = cmd
                        .insert_bundle(SpriteBundle {
                            sprite: Sprite {
                                custom_size: Some(Vec2::splat(size - padding)),
                                color: board_assets.covered_tile_material.color,
                                ..Default::default()
                            },
                            texture: board_assets.covered_tile_material.texture.clone(),
                            transform: Transform::from_xyz(0.0, 0.0, 2.0),
                            ..Default::default()
                        })
                        .insert(Name::new("Tile Cover"))
                        .id();

                    covered_tiles.insert(coordinates, entity);

                    if safe_start_entity.is_none() && *tile == Tile::Empty {
                        *safe_start_entity = Some(entity);
                    }
                });

                match tile {
                    Tile::Bomb => {
                        cmd.insert(Bomb);
                        cmd.with_children(|parent| {
                            parent.spawn_bundle(SpriteBundle {
                                sprite: Sprite {
                                    custom_size: Some(Vec2::splat(size - padding)),
                                    ..Default::default()
                                },
                                transform: Transform::from_xyz(0.0, 0.0, 1.0),
                                texture: board_assets.bomb_material.texture.clone(),
                                ..Default::default()
                            });
                        });
                    }
                    Tile::BombNeighbor(v) => {
                        cmd.insert(BombNeighbor { count: *v });
                        cmd.with_children(|parent| {
                            parent.spawn_bundle(Self::bomb_count_text_bundle(
                                *v,
                                board_assets,
                                size - padding,
                            ));
                        });
                    }
                    Tile::Empty => (),
                }
            }
        }
    }

    fn bomb_count_text_bundle(count: u8, board_assets: &BoardAssets, size: f32) -> Text2dBundle {
        let color = board_assets.bomb_counter_color(count);
        Text2dBundle {
            text: Text {
                sections: vec![TextSection {
                    value: count.to_string(),
                    style: TextStyle {
                        color,
                        font: board_assets.bomb_counter_font.clone(),
                        font_size: size,
                    },
                }],
                alignment: TextAlignment {
                    vertical: VerticalAlign::Center,
                    horizontal: HorizontalAlign::Center,
                },
            },
            transform: Transform::from_xyz(0.0, 0.0, 1.0),
            ..Default::default()
        }
    }

    fn cleanup_board(board: Res<Board>, mut commands: Commands) {
        // 清理所有的棋子
        commands.entity(board.entity).despawn_recursive();
        commands.remove_resource::<Board>();
    }
}
