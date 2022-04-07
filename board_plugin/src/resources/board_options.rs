use bevy::prelude::Vec3;
use serde::{Deserialize, Serialize};

/// TileSize
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TileSize {
    /// 固定
    Fixed(f32),
    /// 窗口自适应
    Adaptive { min: f32, max: f32 },
}

/// BoardPosition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BoardPosition {
    /// 居中
    Centered { offset: Vec3 },
    /// 自定义位置
    Custom(Vec3),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoardOptions {
    /// 棋盘大小
    pub map_size: (u16, u16),
    /// 炸弹数量
    pub bomb_count: u16,
    /// 窗口位置
    pub position: BoardPosition,
    /// 棋子尺寸
    pub tile_size: TileSize,
    /// 每个棋子之间的间隔
    pub tile_padding: f32,
    /// 控制初始是否先揭开一个空白的棋子
    pub safe_start: bool,
}

impl Default for TileSize {
    fn default() -> Self {
        Self::Adaptive {
            min: 10.0,
            max: 50.0,
        }
    }
}

impl Default for BoardPosition {
    fn default() -> Self {
        Self::Centered {
            offset: Default::default(),
        }
    }
}

impl Default for BoardOptions {
    fn default() -> Self {
        Self {
            map_size: (15, 15),
            bomb_count: 30,
            position: Default::default(),
            tile_size: Default::default(),
            tile_padding: 0.,
            safe_start: false,
        }
    }
}
