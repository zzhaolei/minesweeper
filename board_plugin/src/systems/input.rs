use bevy::{
    input::{mouse::MouseButtonInput, ElementState},
    log,
    prelude::*,
};

use crate::{
    events::{TileMarkEvent, TileTriggerEvent},
    resources::board::Board,
};

pub fn input_handling(
    windows: Res<Windows>,
    board: Res<Board>,
    mut button_evr: EventReader<MouseButtonInput>,
    mut tile_trigger_ewr: EventWriter<TileTriggerEvent>,
    mut tile_mark_ewr: EventWriter<TileMarkEvent>,
) {
    let window = windows.get_primary().unwrap();

    for event in button_evr.iter() {
        if let ElementState::Pressed = event.state {
            let position = window.cursor_position();
            if position.is_none() {
                continue;
            }
            let pos = position.unwrap();
            log::trace!("Mouse button pressed: {:?} at {}", event.button, pos);

            let tile_coordinates = board.mouse_position(window, pos);
            if tile_coordinates.is_none() {
                continue;
            }
            let coordinates = tile_coordinates.unwrap();
            // 棋子已经被翻开，则不能进行任何处理
            if !board.is_covered_at(coordinates) {
                continue;
            }
            match event.button {
                MouseButton::Left => {
                    // 棋子未标记，触发事件
                    if !board.is_marked_at(coordinates) {
                        log::info!("翻开坐标{}的棋子", coordinates);
                        // 发送事件
                        tile_trigger_ewr.send(TileTriggerEvent(coordinates));
                    }
                }
                MouseButton::Right => {
                    if board.is_marked_at(coordinates) {
                        log::info!("解除标记坐标{}的棋子", coordinates);
                    } else {
                        log::info!("标记坐标{}的棋子", coordinates);
                    }
                    tile_mark_ewr.send(TileMarkEvent(coordinates));
                }
                _ => (),
            }
        }
    }
}
