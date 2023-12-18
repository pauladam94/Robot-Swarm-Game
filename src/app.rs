use egui::Vec2;

use crate::compute::Grid;

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
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            grid: Grid::new(),
            last_move: None,
            spacing: 5.0,
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
        let Self {
            grid,
            last_move,
            spacing,
        } = self;

        egui::SidePanel::right("Window").show(ctx, |ui| {
            egui::widgets::global_dark_light_mode_buttons(ui);

            ui.add(egui::Slider::new(spacing, 0.0..=20.0).text("Spacing"))
                .on_hover_text("Spacing between cells");

            if let Some(last_move) = last_move {
                ui.heading(format!("Last Move : {}", last_move));
            }

            if ui
                .add(egui::Button::new("Up").min_size(Vec2::new(100.0, 100.0)))
                .clicked()
                || ctx.input(|i| i.keys_down.contains(&egui::Key::ArrowUp))
            {
                grid.up_step();
                *last_move = Some(Move::Up);
            }

            ui.horizontal(|ui| {
                if ui
                    .add(egui::Button::new("Left").min_size(Vec2::new(100.0, 100.0)))
                    .clicked()
                    || ctx.input(|i| i.keys_down.contains(&egui::Key::ArrowLeft))
                {
                    grid.left_step();
                    *last_move = Some(Move::Left);
                }

                if ui
                    .add(egui::Button::new("Right").min_size(Vec2::new(100.0, 100.0)))
                    .clicked()
                    || ctx.input(|i| i.keys_down.contains(&egui::Key::ArrowRight))
                {
                    grid.right_step();
                    *last_move = Some(Move::Right);
                }
            });

            if ui
                .add(egui::Button::new("Down").min_size(Vec2::new(100.0, 100.0)))
                .clicked()
                || ctx.input(|i| i.keys_down.contains(&egui::Key::ArrowDown))
            {
                grid.down_step();
                *last_move = Some(Move::Down);
            }

            if ui
                .add(egui::Button::new("Reverse").min_size(Vec2::new(100.0, 100.0)))
                .clicked()
            {
                grid.reverse();
            }
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            self.grid.draw(ui, *spacing);
        });
    }
}
