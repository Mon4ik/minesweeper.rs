use vctr2::vector2::Vector2;

use crate::game_types::GameSettings;

pub(crate) const CELL_WIDTH: u16 = 5;
pub(crate) const CELL_HEIGHT: u16 = 3;

// --- Bomb count in different difficulties --- //

// Beginner ––––– The player is presented with a 9x9 grid in which 10 mines are concealed.
// Intermediate – The player is presented with a 16x16 grid in which 40 mines are concealed.
// Expert ––––––– The player is presented with a 16x30 grid in which 99 mines are concealed.
// Custom ––––––– Minesweeper displays a dialog box which allows the player to set grid size and number of mines.
//
// (c) https://en.wikibooks.org/wiki/Minesweeper/Playing_Levels

pub(crate) const DIFFICULY_BEGINNER: GameSettings = GameSettings {
    size: Vector2::new(9, 9),
    mines: 10,
};

pub(crate) const DIFFICULY_INTERMEDIATE: GameSettings = GameSettings {
    size: Vector2::new(16, 16),
    mines: 40,
};

pub(crate) const DIFFICULY_EXPERT: GameSettings = GameSettings {
    size: Vector2::new(16, 30),
    mines: 99,
};
