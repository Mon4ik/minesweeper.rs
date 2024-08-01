use ratatui::{prelude::*, widgets::*};
use vctr2::vector2::Vector2;

use crate::{
    game_types::{Cell, CellState, CellValue, GameSettings},
    game_utils::get_bombs_around,
};

pub struct GameCell {
    position: Vector2<u16>,
    cells: Vec<Vec<Cell>>,
    game_settings: GameSettings,
}

impl GameCell {
    pub fn new(position: Vector2<u16>, cells: Vec<Vec<Cell>>, game_settings: GameSettings) -> Self {
        Self {
            position,
            cells,
            game_settings,
        }
    }

    fn get_merged_borders(
        &self,
        base_border_set: &symbols::border::Set,
        x: u16,
        y: u16,
    ) -> symbols::border::Set {
        let mut border_set = symbols::border::Set {
            top_left: symbols::line::TOP_LEFT,
            top_right: symbols::line::TOP_RIGHT,
            bottom_left: symbols::line::BOTTOM_LEFT,
            bottom_right: symbols::line::BOTTOM_RIGHT,
            ..base_border_set.clone()
        };

        if x >= 1 && y == 0 {
            border_set.top_left = symbols::line::HORIZONTAL_DOWN;
        }

        if x == 0 && y >= 1 {
            border_set.top_left = symbols::line::VERTICAL_RIGHT;
        }

        if x >= 1 && y >= 1 {
            border_set.top_left = symbols::line::CROSS;
        }

        if x >= 1 && y == self.game_settings.size.y - 1 {
            border_set.bottom_left = symbols::line::HORIZONTAL_UP;
        }

        if x == self.game_settings.size.x - 1 && y >= 1 {
            border_set.top_right = symbols::line::VERTICAL_LEFT;
        }

        border_set
    }
}

impl Widget for GameCell {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if (self.position.y as usize) >= self.cells.len() {
            return;
        }
        if (self.position.x as usize) >= self.cells[self.position.y as usize].len() {
            return;
        }
        let cell = self.cells[self.position.y as usize][self.position.x as usize];
        let cell_symbol;
        let cell_color;

        match cell.state {
            CellState::Closed => {
                cell_symbol = '#';
                cell_color = Color::Black;
            }
            CellState::Opened => match cell.value {
                CellValue::Empty => {
                    let count =
                        get_bombs_around(&self.game_settings.size, &self.position, &self.cells);

                    cell_symbol = if count == 0 {
                        ' '
                    } else {
                        count.to_string().chars().nth(0).unwrap()
                    };
                    cell_color = Color::Yellow;
                }
                CellValue::Bomb => {
                    cell_symbol = '∅';
                    cell_color = Color::Red;
                }
            },
            CellState::Flagged => {
                cell_symbol = '⚑';
                cell_color = Color::Red;
            }
        };

        let border_set =
            self.get_merged_borders(&symbols::border::PLAIN, self.position.x, self.position.y);

        let mut block = Block::bordered()
            .border_set(border_set)
            .border_style(Color::Gray);

        if cell.state == CellState::Opened && cell.value == CellValue::Empty {
            block = block.bg(cell_color);
        }

        Paragraph::new(format!(" {} ", cell_symbol))
            .fg(cell_color)
            .block(block)
            .render(area, buf);
    }
}
