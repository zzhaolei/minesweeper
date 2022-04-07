use std::ops::{Deref, DerefMut};

use rand::Rng;

use crate::{components::Coordinates, resources::tile::Tile};

/*
*--------*-------*-------*
| -1, 1  | 0, 1  | 1, 1  |
|--------|-------|-------|
| -1, 0  | tile  | 1, 0  |
|--------|-------|-------|
| -1, -1 | 0, -1 | 1, -1 |
*--------*-------*-------*
*/
const SQUARE_COORDINATES: [(i8, i8); 8] = [
    // 左下
    (-1, -1),
    // 下
    (0, -1),
    // 右下
    (1, -1),
    // 左
    (-1, 0),
    // 右
    (1, 0),
    // 左上
    (-1, 1),
    // 上
    (0, 1),
    // 右上
    (1, 1),
];

/// 定义棋盘
#[derive(Debug, Clone)]
pub struct TileMap {
    /// 炸弹数量
    bomb_count: u16,
    /// 高度
    height: u16,
    /// 宽度
    width: u16,
    /// 定义一个2维的棋盘，[[], []]，一维为高，二维为宽
    // TODO: 尝试使用数据实现这个map
    map: Vec<Vec<Tile>>,
}

impl TileMap {
    /// 生成一个空的map
    pub fn empty(width: u16, height: u16) -> Self {
        let map = (0..height)
            .into_iter()
            .map(|_| (0..width).into_iter().map(|_| Tile::Empty).collect())
            .collect();
        Self {
            bomb_count: 0,
            height,
            width,
            map,
        }
    }

    #[cfg(feature = "debug")]
    pub fn console_output(&self) -> String {
        // TODO: 优化字符串拼接，这种方式并不高效
        let mut buffer = format!(
            "棋盘尺寸: ({}, {})，包含炸弹: {}\n",
            self.width, self.height, self.bomb_count
        );
        let line: String = (0..=(self.width + 1)).into_iter().map(|_| '-').collect();
        buffer = format!("{}{}\n", buffer, line);
        for line in self.iter().rev() {
            buffer = format!("{}|", buffer);
            for tile in line.iter() {
                buffer = format!("{}{}", buffer, tile.console_output());
            }
            buffer = format!("{}|\n", buffer);
        }
        format!("{}{}", buffer, line)
    }

    pub fn width(&self) -> u16 {
        self.width
    }

    pub fn height(&self) -> u16 {
        self.height
    }

    pub fn bomb_count(&self) -> u16 {
        self.bomb_count
    }

    pub fn safe_square_at(&self, coordinates: Coordinates) -> impl Iterator<Item = Coordinates> {
        SQUARE_COORDINATES
            .iter()
            .copied()
            .map(move |tuple| coordinates + tuple)
    }
    /// 判断一个坐标棋子是不是炸弹
    pub fn is_bomb_at(&self, coordinates: Coordinates) -> bool {
        if coordinates.x >= self.width || coordinates.y >= self.height {
            return false;
        }
        self.map[coordinates.y as usize][coordinates.x as usize].is_bomb()
    }
    /// 计算一个棋子周围有几个炸弹
    pub fn bomb_count_at(&self, coordinates: Coordinates) -> u8 {
        if self.is_bomb_at(coordinates) {
            return 0;
        }

        let res = self
            .safe_square_at(coordinates)
            .filter(|coord| self.is_bomb_at(*coord))
            .count();
        res as u8
    }

    // 在棋盘上放置炸弹和邻居
    pub fn set_bombs(&mut self, bomb_count: u16) {
        self.bomb_count = bomb_count;
        let mut remaining_bombs = bomb_count;
        let mut rng = rand::thread_rng();

        // 放置炸弹
        while remaining_bombs > 0 {
            // 在棋盘上随机放置一个炸弹
            let (x, y) = (
                rng.gen_range(0..self.width) as usize,
                rng.gen_range(0..self.height) as usize,
            );
            // self实现了deref，返回一个&self.map
            if let Tile::Empty = self[y][x] {
                // 放置炸弹
                self[y][x] = Tile::Bomb;
                remaining_bombs -= 1;
            }
        }

        // 放置和炸弹相邻的棋子
        for y in 0..self.height {
            for x in 0..self.width {
                let coords = Coordinates { x, y };
                // 如果这个棋子本身就是一个炸弹，则跳过
                let num = self.bomb_count_at(coords);
                if num == 0 {
                    continue;
                }
                // 记录这个棋子周围有几个炸弹
                let tile = &mut self[y as usize][x as usize];
                *tile = Tile::BombNeighbor(num);
            }
        }
    }
}

impl Deref for TileMap {
    type Target = Vec<Vec<Tile>>;

    fn deref(&self) -> &Self::Target {
        &self.map
    }
}

impl DerefMut for TileMap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.map
    }
}
