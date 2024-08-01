use vctr2::vector2::Vector2;

use crate::game_types::{Cell, CellValue};

pub(crate) fn get_neighbours(
    game_size: &Vector2<u16>,
    position: &Vector2<u16>,
) -> Vec<Vector2<u16>> {
    let position_i32 = Vector2::new(position.x as i32, position.y as i32);

    let neighbours_i32: Vec<Vector2<i32>> = vec![
        Vector2::new(position_i32.x - 1, position_i32.y - 1),
        Vector2::new(position_i32.x, position_i32.y - 1),
        Vector2::new(position_i32.x + 1, position_i32.y - 1),
        Vector2::new(position_i32.x + 1, position_i32.y),
        Vector2::new(position_i32.x + 1, position_i32.y + 1),
        Vector2::new(position_i32.x, position_i32.y + 1),
        Vector2::new(position_i32.x - 1, position_i32.y + 1),
        Vector2::new(position_i32.x - 1, position_i32.y),
    ];

    let mut neighbours = Vec::new();

    for neighbour_i32 in neighbours_i32 {
        if neighbour_i32.x >= 0
            && neighbour_i32.x <= (game_size.x as i32) - 1
            && neighbour_i32.y >= 0
            && neighbour_i32.y <= (game_size.y as i32) - 1
        {
            neighbours.push(Vector2::new(neighbour_i32.x as u16, neighbour_i32.y as u16));
        }
    }

    neighbours
}

pub(crate) fn get_bombs_around(
    game_size: &Vector2<u16>,
    position: &Vector2<u16>,
    cells: &Vec<Vec<Cell>>,
) -> u16 {
    let neighbours_pos = get_neighbours(game_size, position);
    let mut bombs = 0;

    for neighbour_pos in neighbours_pos {
        let cell = cells[neighbour_pos.y as usize][neighbour_pos.x as usize];
        if cell.value == CellValue::Bomb {
            bombs += 1
        }
    }

    bombs
}

#[cfg(test)]
mod tests {
    use crate::DIFFICULY_BEGINNER;

    use super::*;

    #[test]
    fn get_neightbours() {
        assert_eq!(
            get_neighbours(&DIFFICULY_BEGINNER.size, &Vector2::new(1, 1)),
            vec![
                Vector2::new(0, 0),
                Vector2::new(1, 0),
                Vector2::new(2, 0),
                Vector2::new(2, 1),
                Vector2::new(2, 2),
                Vector2::new(1, 2),
                Vector2::new(0, 2),
                Vector2::new(0, 1),
            ]
        );

        assert_eq!(
            get_neighbours(&DIFFICULY_BEGINNER.size, &Vector2::new(0, 0)),
            vec![Vector2::new(1, 0), Vector2::new(1, 1), Vector2::new(0, 1),]
        );
    }
}
