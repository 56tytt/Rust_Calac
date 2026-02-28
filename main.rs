// ============================================================
//  CASIO Scientific Calculator Suite — Rust + egui
//  3 Models: fx-82MS | fx-991ES PLUS | fx-CG50
//  Author: 56tytt — שי קדוש הנדסת תוכנה אשקלון
// ============================================================

mod engine;
mod models;
mod ui;

use eframe::egui;
use models::ModelType;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
        .with_title("CASIO Scientific Calculator")
        .with_inner_size([400.0, 750.0])
        .with_resizable(true)
        .with_min_inner_size([340.0, 600.0]),
        ..Default::default()
    };

    eframe::run_native(
        "CASIO Calculator",
        options,
        Box::new(|cc| Box::new(ui::CasioApp::new(cc, ModelType::Fx82MS))),
    )
}
