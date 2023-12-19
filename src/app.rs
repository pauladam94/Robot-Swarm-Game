use crate::compute::{Grid, GRID_SIZE};
use egui::Vec2;

#[derive(Clone, Copy, PartialEq)]
pub enum Move {
    Left,
    Right,
    Up,
    Down,
}

impl std::fmt::Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Move::Left => write!(f, "Left"),
            Move::Right => write!(f, "Right"),
            Move::Up => write!(f, "Up"),
            Move::Down => write!(f, "Down"),
        }
    }
}
pub struct TemplateApp {
    grid: Grid,
    last_move: Option<Move>,
    spacing: f32,
    relocation: bool,
    step: usize,
    next_move: Option<Move>,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            grid: Grid::simple_three_particule(),
            last_move: None,
            spacing: 7.0,
            relocation: false,
            step: 1,
            next_move: None,
        }
    }
}

impl TemplateApp {
    pub fn new(_: &eframe::CreationContext<'_>) -> Self {
        Default::default()
    }
}

impl eframe::App for TemplateApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::right("Window").show(ctx, |ui| {
            egui::widgets::global_dark_light_mode_buttons(ui);

            ui.add(egui::Slider::new(&mut self.spacing, 0.0..=20.0).text("Spacing"))
                .on_hover_text("Spacing between cells");

            ui.horizontal(|ui| {
                if ui.add(egui::Button::new("Up")).clicked()
                    || ctx.input(|i| i.key_pressed(egui::Key::ArrowUp))
                {
                    self.next_move = Some(Move::Up);
                }

                if ui.add(egui::Button::new("Left")).clicked()
                    || ctx.input(|i| i.key_pressed(egui::Key::ArrowLeft))
                {
                    self.next_move = Some(Move::Left);
                }

                if ui.add(egui::Button::new("Right")).clicked()
                    || ctx.input(|i| i.key_pressed(egui::Key::ArrowRight))
                {
                    self.next_move = Some(Move::Right);
                }

                if ui.add(egui::Button::new("Down")).clicked()
                    || ctx.input(|i| i.key_pressed(egui::Key::ArrowDown))
                {
                    self.next_move = Some(Move::Down);
                }
            });

            if ui.add(egui::Button::new("Map 3 Particule")).clicked() {
                self.grid = Grid::simple_three_particule();
            }

            if ui.add(egui::Button::new("Map 1 Particule")).clicked() {
                self.grid = Grid::simple_one_particule();
            }

            if ui.add(egui::Button::new("Map 4 Particule")).clicked() {
                self.grid = Grid::complex_four_particule();
            }

            if ui.add(egui::Button::new("Reverse")).clicked() {
                self.grid.reverse();
            }

            ui.checkbox(&mut self.relocation, "Relocation Problem");
            // slider for the number of steps
            ui.add(egui::Slider::new(&mut self.step, 1..=GRID_SIZE).text("Step"))
                .on_hover_text("Number of steps to do");

            ui.separator();
            ui.heading(format!("Last Move :"));
            if let Some(last_move) = &self.last_move {
                ui.label(
                    egui::RichText::new(format!("{last_move}"))
                        .font(egui::FontId::proportional(70.0)),
                );
            }

            if let Some(next_move) = self.next_move {
                for _ in 0..self.step {
                    self.grid.step(next_move, &mut self.last_move);
                }
                self.next_move = None;
            }
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            self.grid
                .draw(ui, self.spacing, self.relocation, Vec2::new(20f32, 20f32));
        });
    }
}
