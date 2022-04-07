use crate::components::Coordinates;

#[derive(Debug, Clone, Copy)]
pub struct TileTriggerEvent(pub Coordinates);

/// 完成
#[derive(Debug, Clone, Copy)]
pub struct BoardCompletedEvent;

/// 爆炸
#[derive(Debug, Clone, Copy)]
pub struct BombExplosionEvent;

/// 标记
#[derive(Debug, Clone, Copy)]
pub struct TileMarkEvent(pub Coordinates);
