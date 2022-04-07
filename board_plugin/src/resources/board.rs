use bevy::{log, prelude::*, utils::HashMap};

use crate::{bounds::Bounds2, components::Coordinates};

use super::tile_map::TileMap;

#[derive(Debug)]
pub struct Board {
    pub tile_map: TileMap,
    pub bounds: Bounds2,
    pub tile_size: f32,
    pub covered_tiles: HashMap<Coordinates, Entity>,
    pub entity: Entity,
    pub marked_tiles: Vec<Coordinates>,
}

impl Board {
    /// 将鼠标位置转为棋盘坐标
    pub fn mouse_position(&self, window: &Window, position: Vec2) -> Option<Coordinates> {
        let window_size = Vec2::new(window.width(), window.height());
        let position = position - window_size / 2.0;

        // 检测鼠标是否在窗口内
        if !self.bounds.in_bounds(position) {
            return None;
        }

        let coordinates = position - self.bounds.position;
        Some(Coordinates {
            x: (coordinates.x / self.tile_size) as u16,
            y: (coordinates.y / self.tile_size) as u16,
        })
    }

    pub fn is_covered_at(&self, coords: Coordinates) -> bool {
        self.covered_tiles.get(&coords).is_some()
    }

    pub fn is_marked_at(&self, coords: Coordinates) -> bool {
        self.marked_tiles.contains(&coords)
    }

    pub fn is_bomb_at(&self, coords: Coordinates) -> bool {
        self.tile_map.is_bomb_at(coords)
    }

    pub fn is_completed(&self) -> bool {
        self.tile_map.bomb_count() as usize == self.covered_tiles.len()
    }

    /// 隐藏棋子
    pub fn tile_to_uncover(&self, coords: &Coordinates) -> Option<&Entity> {
        // 如果这个棋子已经被标记，则忽略
        if self.marked_tiles.contains(coords) {
            None
        } else {
            self.covered_tiles.get(coords)
        }
    }

    /// 揭开棋子
    pub fn try_uncover_tile(&mut self, coords: &Coordinates) -> Option<Entity> {
        // 如果棋子被标记了，则先去除标记
        if self.marked_tiles.contains(coords) {
            self.unmarked_tile(coords)?;
        }
        self.covered_tiles.remove(coords)
    }

    /// 检测当前棋子周围有没有隐藏的棋子
    pub fn adjacent_covered_tiles(&self, coord: Coordinates) -> Vec<Entity> {
        self.tile_map
            .safe_square_at(coord)
            .filter_map(|c| self.covered_tiles.get(&c))
            .copied()
            .collect()
    }

    /// 移除这个棋子身上的标记
    fn unmarked_tile(&mut self, coords: &Coordinates) -> Option<Coordinates> {
        // 查找是否有坐标一样的标记棋子
        let pos = match self.marked_tiles.iter().position(|a| a == coords) {
            None => {
                log::error!("移除标记失败: {}", coords);
                return None;
            }
            Some(p) => p,
        };
        Some(self.marked_tiles.remove(pos))
    }

    /// 尝试标记一个棋子，如果棋子已经被标记，则取消标记并返回false
    /// 如果棋子未被标记，则标记并返回true
    pub fn try_toggle_mark(&mut self, coords: &Coordinates) -> Option<(Entity, bool)> {
        let entity = *self.covered_tiles.get(coords)?;
        let mark = if self.marked_tiles.contains(coords) {
            self.unmarked_tile(coords)?;
            false
        } else {
            self.marked_tiles.push(*coords);
            true
        };
        Some((entity, mark))
    }
}
