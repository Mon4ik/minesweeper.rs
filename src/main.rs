use std::io::{stdout, Result};

use constants::*;
use game_cell::GameCell;
use game_types::{Cell, CellState, CellValue, GameSettings};
use game_utils::{get_bombs_around, get_neighbours};
use rand::Rng;
use ratatui::{
    backend::CrosstermBackend,
    crossterm::{
        event::{self, Event, KeyCode},
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
        ExecutableCommand,
    },
    prelude::*,
    widgets::*,
};
use vctr2::vector2::Vector2;

mod constants;
mod game_cell;
mod game_types;
mod game_utils;

fn main() -> Result<()> {
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;
    let mut app = App::new();

    loop {
        terminal.draw(|frame| app.render(frame))?;

        if app.process_event().unwrap_or(true) {
            break;
        }
    }

    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}

#[allow(dead_code)]
enum AppState {
    Menu,
    Playing,
    Dead,
}

#[allow(dead_code)]
struct App {
    cells: Vec<Vec<Cell>>,
    cursor: Vector2<u16>,

    state: AppState,
    game_settings: GameSettings,
}

impl App {
    pub fn new() -> Self {
        Self {
            cells: Self::generate_cells(&DIFFICULY_BEGINNER),
            cursor: Vector2::new(0, 0),
            state: AppState::Menu,
            game_settings: DIFFICULY_BEGINNER,
        }
    }

    pub fn render(&mut self, frame: &mut Frame) {
        self.render_minesweeper(frame);

        match self.state {
            AppState::Menu => {
                // let rect = Rect::new(rect_x, rect_y, CELL_WIDTH, CELL_HEIGHT);
                //
                // frame.render_widget(
                //     Block::bordered()
                //         .border_set(symbols::border::DOUBLE)
                //         .border_style(Color::Green),
                //     rect,
                // )
            }
            AppState::Playing => {
                // Cursor

                let area_x = self.cursor.x * (CELL_WIDTH - 1);
                let area_y = self.cursor.y * (CELL_HEIGHT - 1);

                let area = Rect::new(area_x, area_y, CELL_WIDTH, CELL_HEIGHT);

                frame.render_widget(
                    Block::bordered()
                        .border_set(symbols::border::DOUBLE)
                        .border_style(Color::Green),
                    area,
                )
            }
            _ => {}
        }
    }

    fn render_minesweeper(&mut self, frame: &mut Frame) {
        for y in 0..self.game_settings.size.y {
            for x in 0..self.game_settings.size.x {
                let position = Vector2::new(x, y);

                let area_x = position.x * (CELL_WIDTH - 1);
                let area_y = position.y * (CELL_HEIGHT - 1);
                let area = Rect::new(area_x, area_y, CELL_WIDTH, CELL_HEIGHT);

                frame.render_widget(
                    GameCell::new(position, self.cells.to_vec(), self.game_settings.clone()),
                    area,
                );
            }
        }
    }

    pub fn process_event(&mut self) -> Result<bool> {
        if event::poll(std::time::Duration::from_millis(16))? {
            let read_event = event::read()?;

            match self.state {
                AppState::Menu => match read_event {
                    Event::Key(key) => match key.code {
                        KeyCode::Esc => {
                            return Ok(true);
                        }
                        KeyCode::Up => {}
                        KeyCode::Down => {}
                        KeyCode::Enter => {
                            self.game_settings = DIFFICULY_BEGINNER;
                            self.cells = Self::generate_cells(&DIFFICULY_BEGINNER);
                            self.cursor = Vector2::new(0, 0);
                            self.state = AppState::Playing;
                        }
                        _ => {}
                    },
                    _ => {}
                },
                AppState::Playing => {
                    match read_event {
                        Event::Key(key) => {
                            // k
                            match key.code {
                                KeyCode::Esc => {
                                    self.open_dead_mines();
                                    self.state = AppState::Dead;
                                }
                                KeyCode::Enter => self.open_cell(&self.cursor.clone()),
                                KeyCode::Char('f') => self.flag_cell(&self.cursor.clone()),
                                // Cursor movement
                                KeyCode::Left => {
                                    if self.cursor.x > 0 {
                                        self.cursor.x -= 1;
                                    }
                                }
                                KeyCode::Right => {
                                    if self.cursor.x < self.game_settings.size.x - 1 {
                                        self.cursor.x += 1;
                                    }
                                }
                                KeyCode::Up => {
                                    if self.cursor.y > 0 {
                                        self.cursor.y -= 1;
                                    }
                                }
                                KeyCode::Down => {
                                    if self.cursor.y < self.game_settings.size.y - 1 {
                                        self.cursor.y += 1;
                                    }
                                }
                                _ => {}
                            }
                        }

                        _ => {}
                    }
                }
                AppState::Dead => {
                    self.state = AppState::Menu;
                    self.cells = Self::generate_cells(&DIFFICULY_BEGINNER);
                }
            }
        }

        Ok(false)
    }

    fn generate_cells(game_settings: &GameSettings) -> Vec<Vec<Cell>> {
        let mut cells = Vec::new();

        for _y in 0..game_settings.size.x {
            let mut row = Vec::new();

            for _x in 0..game_settings.size.y {
                row.push(Cell {
                    state: CellState::Closed,
                    value: CellValue::Empty,
                })
            }

            cells.push(row)
        }

        let mut rng = rand::thread_rng();

        let mut count = game_settings.mines;

        while count > 0 {
            let x = rng.gen_range(0..game_settings.size.x) as usize;
            let y = rng.gen_range(0..game_settings.size.y) as usize;

            if cells[y][x].value != CellValue::Bomb {
                cells[y][x].value = CellValue::Bomb;
                count -= 1;
            }
        }
        for _i in 0..10 {}

        return cells;
    }

    //     fn get_cell_symbol(&self, x: u16, y: u16) -> char {
    //         // bomb neightbours: 12345678
    //         // got bomb: Ⓑ//🅱
    //         // flagged: ⚑

    //     }
    //
    fn open_cell(&mut self, position: &Vector2<u16>) {
        {
            let cell = &mut self.cells[position.y as usize][position.x as usize];
            cell.state = CellState::Opened;
        };

        let cell = &self.cells[position.y as usize][position.x as usize];

        if cell.value == CellValue::Bomb {
            self.open_dead_mines();
            return;
        }

        let no_bombs_around =
            get_bombs_around(&self.game_settings.size, &position, &self.cells) == 0;

        if cell.value == CellValue::Empty && no_bombs_around {
            // spread opening
            for neighbour_pos in get_neighbours(&self.game_settings.size, position) {
                let cell = &mut self.cells[neighbour_pos.y as usize][neighbour_pos.x as usize];

                if cell.state != CellState::Closed {
                    continue;
                }

                match cell.value {
                    CellValue::Empty => {
                        cell.state = CellState::Opened;

                        if get_bombs_around(&self.game_settings.size, &neighbour_pos, &self.cells)
                            == 0
                        {
                            self.open_cell(&neighbour_pos);
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    fn flag_cell(&mut self, position: &Vector2<u16>) {
        let cell = &mut self.cells[position.y as usize][position.x as usize];
        if cell.state == CellState::Closed {
            cell.state = CellState::Flagged;
        } else if cell.state == CellState::Flagged {
            cell.state = CellState::Closed;
        }
    }

    fn open_dead_mines(&mut self) {
        self.state = AppState::Dead;
        for y in 0..self.cells.len() {
            for x in 0..self.cells[y].len() {
                if self.cells[y][x].value == CellValue::Bomb {
                    self.cells[y][x].state = CellState::Opened;
                }
            }
        }
    }
}
