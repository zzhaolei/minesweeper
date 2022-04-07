use bevy::{log, prelude::*};

use crate::{
    components::{Bomb, BombNeighbor, Coordinates, Uncover},
    events::{BoardCompletedEvent, BombExplosionEvent, TileTriggerEvent},
    resources::board::Board,
};

pub fn trigger_event_handler(
    mut commands: Commands,
    board: Res<Board>,
    mut tile_trigger_evr: EventReader<TileTriggerEvent>,
) {
    for trigger_event in tile_trigger_evr.iter() {
        if board.is_bomb_at(trigger_event.0) {
            log::error!("这是一个炸弹，游戏结束！");
            for (_, entity) in board.covered_tiles.iter() {
                commands.entity(*entity).insert(Uncover);
            }
            break;
        }

        if let Some(entity) = board.tile_to_uncover(&trigger_event.0) {
            commands.entity(*entity).insert(Uncover);
        }
    }
}

pub fn uncover_tiles(
    mut commands: Commands,
    mut board: ResMut<Board>,
    children: Query<(Entity, &Parent), With<Uncover>>,
    parents: Query<(&Coordinates, Option<&Bomb>, Option<&BombNeighbor>)>,
    mut board_completed_event_wr: EventWriter<BoardCompletedEvent>,
    mut bomb_explosion_event_wr: EventWriter<BombExplosionEvent>,
) {
    for (entity, parent) in children.iter() {
        // 销毁覆盖在棋子上的组件
        commands.entity(entity).despawn_recursive();

        let (coords, bomb, bomb_counter) = match parents.get(parent.0) {
            Ok(v) => v,
            Err(e) => {
                log::error!("{}", e);
                continue;
            }
        };

        match board.try_uncover_tile(coords) {
            None => log::debug!("试图揭开一个已经被揭开的棋子"),
            Some(e) => log::debug!("揭开的棋子 {} (entity: {:?})", coords, e),
        }

        if board.is_completed() {
            board_completed_event_wr.send(BoardCompletedEvent);
        }

        // 如果揭开的是炸弹棋子，则结束游戏
        if bomb.is_some() {
            log::info!("Boom!");
            bomb_explosion_event_wr.send(BombExplosionEvent);
        } else if bomb_counter.is_none() {
            // 如果相邻的棋子是空的(Empty)，则添加到commands中，等待下一帧刷新的时候，揭开这些棋子
            for entity in board.adjacent_covered_tiles(*coords) {
                commands.entity(entity).insert(Uncover);
            }
        }
    }
}
