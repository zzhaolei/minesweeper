use bevy::prelude::Component;

/// 炸弹相邻组件
#[cfg_attr(feature = "debug", derive(bevy_inspector_egui::Inspectable))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Component)]
pub struct BombNeighbor {
    /// 记录周围有几个炸弹
    pub count: u8,
}
