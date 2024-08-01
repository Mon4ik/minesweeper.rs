use vctr2::vector2::Vector2;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CellState {
    Closed,
    Opened,
    Flagged,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CellValue {
    Empty,
    Bomb,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cell {
    pub state: CellState,
    pub value: CellValue,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GameSettings {
    pub size: Vector2<u16>,
    pub mines: i32,
}
