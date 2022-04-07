#[cfg(feature = "debug")]
use colored::Colorize;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tile {
    /// 一个炸弹棋子
    Bomb,
    /// 炸弹附近的显示炸弹数量的棋子
    BombNeighbor(u8),
    /// 空棋子
    Empty,
}

impl Tile {
    /// 判断棋子是否是炸弹
    pub const fn is_bomb(&self) -> bool {
        matches!(self, Self::Bomb)
    }

    #[cfg(feature = "debug")]
    pub fn console_output(&self) -> String {
        format!(
            "{}",
            match self {
                Tile::Bomb => "*".bright_red(),
                Tile::BombNeighbor(v) => match v {
                    1 => "1".cyan(),
                    2 => "2".green(),
                    3 => "3".yellow(),
                    _ => v.to_string().red(),
                },
                Tile::Empty => " ".normal(),
            }
        )
    }
}
