use egui::{Color32, Pos2, Rect, Stroke, Vec2};

pub const GRID_SIZE: usize = 10;
const J0: usize = 2;
const I0: usize = 2;

#[derive(Copy, Clone, PartialEq)]
enum Cell {
    Empty,
    Wall,
    Particule,
}

impl Cell {
    pub fn fill_color(&self) -> Color32 {
        match self {
            Cell::Empty => Color32::WHITE,
            Cell::Wall => Color32::BLACK,
            Cell::Particule => Color32::RED,
        }
    }

    pub fn stroke(&self, width: f32) -> Stroke {
        match self {
            Cell::Empty => Stroke::new(0f32, Color32::WHITE),
            Cell::Wall => Stroke::new(width, Color32::BLACK),
            Cell::Particule => Stroke::new(width, Color32::RED),
        }
    }

    pub fn rounding(&self, size: f32) -> f32 {
        let rounding = size / (8 * GRID_SIZE) as f32;
        rounding
    }
}

pub struct Grid {
    cells: [[Cell; GRID_SIZE]; GRID_SIZE],
}

use Cell::*;

use crate::app::Move;
impl Grid {
    pub fn simple_three_particule() -> Grid {
        Grid {
            cells: [
                [Wall, Wall, Wall, Wall, Wall, Wall, Wall, Wall, Wall, Wall],
                [
                    Wall, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Wall,
                ],
                [
                    Wall, Empty, Particule, Empty, Empty, Empty, Empty, Empty, Empty, Wall,
                ],
                [
                    Wall, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Wall,
                ],
                [
                    Wall, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Wall,
                ],
                [
                    Wall, Empty, Empty, Empty, Particule, Empty, Empty, Empty, Empty, Wall,
                ],
                [
                    Wall, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Wall,
                ],
                [
                    Wall, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Wall,
                ],
                [
                    Wall, Empty, Empty, Particule, Wall, Empty, Empty, Empty, Empty, Wall,
                ],
                [Wall, Wall, Wall, Wall, Wall, Wall, Wall, Wall, Wall, Wall],
            ],
        }
    }

    pub fn simple_one_particule() -> Grid {
        Grid {
            cells: [
                [Wall, Wall, Wall, Wall, Wall, Wall, Wall, Wall, Wall, Wall],
                [
                    Wall, Empty, Wall, Empty, Empty, Empty, Empty, Empty, Empty, Wall,
                ],
                [
                    Wall, Empty, Empty, Empty, Empty, Wall, Wall, Empty, Empty, Wall,
                ],
                [
                    Wall, Wall, Wall, Wall, Empty, Empty, Empty, Empty, Wall, Wall,
                ],
                [
                    Wall, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Wall,
                ],
                [
                    Wall, Empty, Wall, Empty, Particule, Empty, Empty, Empty, Empty, Wall,
                ],
                [
                    Wall, Empty, Wall, Empty, Empty, Empty, Empty, Wall, Wall, Wall,
                ],
                [
                    Wall, Empty, Empty, Empty, Wall, Empty, Empty, Empty, Empty, Wall,
                ],
                [
                    Wall, Empty, Empty, Empty, Wall, Empty, Empty, Empty, Empty, Wall,
                ],
                [Wall, Wall, Wall, Wall, Wall, Wall, Wall, Wall, Wall, Wall],
            ],
        }
    }

    pub fn complex_four_particule() -> Grid {
        Grid {
            cells: [
                [Wall, Wall, Wall, Wall, Wall, Wall, Wall, Wall, Wall, Wall],
                [
                    Wall, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Wall,
                ],
                [
                    Wall, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Wall,
                ],
                [
                    Wall, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Wall,
                ],
                [
                    Wall, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Wall,
                ],
                [
                    Wall, Empty, Empty, Empty, Particule, Empty, Empty, Empty, Empty, Wall,
                ],
                [
                    Wall, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Wall,
                ],
                [
                    Wall, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Wall,
                ],
                [
                    Wall, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Wall,
                ],
                [Wall, Wall, Wall, Wall, Wall, Wall, Wall, Wall, Wall, Wall],
            ],
        }
    }

    pub fn left_step(&mut self) {
        for i in 0..GRID_SIZE {
            for j in 0..GRID_SIZE {
                if self.cells[i][j] == Cell::Particule {
                    if j != 0 && self.cells[i][j - 1] == Cell::Empty {
                        self.cells[i][j] = Cell::Empty;
                        self.cells[i][j - 1] = Cell::Particule;
                    }
                }
            }
        }
    }

    pub fn right_step(&mut self) {
        for i in 0..GRID_SIZE {
            for j in (0..GRID_SIZE).rev() {
                if self.cells[i][j] == Cell::Particule {
                    if j != GRID_SIZE - 1 && self.cells[i][j + 1] == Cell::Empty {
                        self.cells[i][j] = Cell::Empty;
                        self.cells[i][j + 1] = Cell::Particule;
                    }
                }
            }
        }
    }

    pub fn up_step(&mut self) {
        for j in 0..GRID_SIZE {
            for i in 0..GRID_SIZE {
                if self.cells[i][j] == Cell::Particule {
                    if i != 0 && self.cells[i - 1][j] == Cell::Empty {
                        self.cells[i][j] = Cell::Empty;
                        self.cells[i - 1][j] = Cell::Particule;
                    }
                }
            }
        }
    }

    pub fn down_step(&mut self) {
        for j in 0..GRID_SIZE {
            for i in (0..GRID_SIZE).rev() {
                if self.cells[i][j] == Cell::Particule {
                    if i != GRID_SIZE - 1 && self.cells[i + 1][j] == Cell::Empty {
                        self.cells[i][j] = Cell::Empty;
                        self.cells[i + 1][j] = Cell::Particule;
                    }
                }
            }
        }
    }

    pub fn step(&mut self, m: Move, last_move: &mut Option<Move>) {
        match m {
            Move::Up => self.up_step(),
            Move::Down => self.down_step(),
            Move::Left => self.left_step(),
            Move::Right => self.right_step(),
        }
        *last_move = Some(m);
    }

    pub fn reverse(&mut self) {
        for i in 0..GRID_SIZE {
            for j in 0..GRID_SIZE {
                match self.cells[i][j] {
                    Cell::Particule => self.cells[i][j] = Cell::Empty,
                    Cell::Empty => self.cells[i][j] = Cell::Particule,
                    _ => (),
                }
            }
        }
    }

    pub fn draw(&self, ui: &mut egui::Ui, spacing: f32, relocation: bool, offset: Vec2) {
        let available_width = ui.available_width() - spacing * GRID_SIZE as f32 - offset.x;
        let available_height = ui.available_height() - spacing * GRID_SIZE as f32 - offset.y;
        let min = available_height.min(available_width);
        let cell_height = min / GRID_SIZE as f32;
        for i in 0..GRID_SIZE {
            for j in 0..GRID_SIZE {
                let cell = self.cells[i][j];
                let fill_color = if relocation && i == I0 && j == J0 && cell == Cell::Empty {
                    Color32::from_rgba_premultiplied(255, 0, 0, 80)
                } else {
                    cell.fill_color()
                };
                let stroke = cell.stroke(0f32);
                let rounding = cell.rounding(min);
                let pos = Pos2::new(
                    j as f32 * cell_height + j as f32 * spacing,
                    i as f32 * cell_height + i as f32 * spacing,
                ) + offset;
                ui.painter().rect_filled(
                    Rect::from_min_size(pos, Vec2::new(cell_height, cell_height)),
                    rounding,
                    fill_color,
                );
                ui.painter().rect_stroke(
                    Rect::from_min_size(pos, Vec2::new(cell_height, cell_height)),
                    rounding,
                    stroke,
                );
            }
        }
    }
}
