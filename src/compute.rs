use egui::{Color32, Pos2, Rect, Stroke, Vec2};

const GRID_SIZE: usize = 9;

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

    pub fn rounding(&self) -> f32 {
        match self {
            Cell::Empty => 0f32,
            Cell::Wall => 0f32,
            Cell::Particule => 10f32,
        }
    }
}

pub struct Grid {
    cells: [[Cell; GRID_SIZE]; GRID_SIZE],
}

use Cell::*;
impl Grid {
    pub fn new() -> Grid {
        Grid {
            cells: [
                [Wall, Wall, Wall, Wall, Wall, Wall, Wall, Wall, Wall],
                [Wall, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Wall],
                [
                    Wall, Empty, Particule, Empty, Empty, Empty, Empty, Empty, Wall,
                ],
                [Wall, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Wall],
                [Wall, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Wall],
                [
                    Wall, Empty, Empty, Empty, Particule, Empty, Empty, Empty, Wall,
                ],
                [Wall, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Wall],
                [
                    Wall, Empty, Empty, Particule, Wall, Empty, Empty, Empty, Wall,
                ],
                [Wall, Wall, Wall, Wall, Wall, Wall, Wall, Wall, Wall],
            ],
        }
    }

    pub fn left_step(&mut self) {
        let previous_cells = self.cells.clone();
        for i in 0..GRID_SIZE {
            for j in (0..GRID_SIZE).rev() {
                if previous_cells[i][j] == Cell::Particule {
                    if j != 0 && previous_cells[i][j - 1] == Cell::Empty {
                        self.cells[i][j] = Cell::Empty;
                        self.cells[i][j - 1] = Cell::Particule;
                    }
                }
            }
        }
    }

    pub fn right_step(&mut self) {
        let previous_cells = self.cells.clone();
        for i in 0..GRID_SIZE {
            for j in 0..GRID_SIZE {
                if previous_cells[i][j] == Cell::Particule {
                    if j != GRID_SIZE - 1 && previous_cells[i][j + 1] == Cell::Empty {
                        self.cells[i][j] = Cell::Empty;
                        self.cells[i][j + 1] = Cell::Particule;
                    }
                }
            }
        }
    }

    pub fn up_step(&mut self) {
        let previous_cells = self.cells.clone();
        for i in 0..GRID_SIZE {
            for j in 0..GRID_SIZE {
                if previous_cells[i][j] == Cell::Particule {
                    if i != 0 && previous_cells[i - 1][j] == Cell::Empty {
                        self.cells[i][j] = Cell::Empty;
                        self.cells[i - 1][j] = Cell::Particule;
                    }
                }
            }
        }
    }

    pub fn down_step(&mut self) {
        let previous_cells = self.cells.clone();
        for i in (0..GRID_SIZE).rev() {
            for j in 0..GRID_SIZE {
                if previous_cells[i][j] == Cell::Particule {
                    if i != GRID_SIZE - 1 && previous_cells[i + 1][j] == Cell::Empty {
                        self.cells[i][j] = Cell::Empty;
                        self.cells[i + 1][j] = Cell::Particule;
                    }
                }
            }
        }
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

    pub fn draw(&self, ui: &mut egui::Ui, spacing: f32) {
        let available_height = ui.available_height() + spacing * GRID_SIZE as f32;
        let available_width = ui.available_width() + spacing * GRID_SIZE as f32;
        let min = available_height.min(available_width);
        let cell_height = min / GRID_SIZE as f32;
        for i in 0..GRID_SIZE {
            for j in 0..GRID_SIZE {
                let cell = self.cells[i][j];
                let fill_color = cell.fill_color();
                let stroke = cell.stroke(0.1f32);
                let rounding = cell.rounding();
                let pos = Pos2::new(
                    j as f32 * cell_height + j as f32 * spacing,
                    i as f32 * cell_height + i as f32 * spacing,
                );
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
