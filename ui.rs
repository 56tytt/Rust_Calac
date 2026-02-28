// ============================================================
//  ui.rs — egui rendering for 3 CASIO models
// ============================================================

use egui::{
    Color32, FontId, Pos2, Rect, Response, RichText, Rounding, Sense,
    Stroke, Ui, Vec2, Frame, Align2,
};
use crate::engine::{CalcEngine, AngleMode};
use crate::models::{button_grid, BtnColor, BtnDef, ModelType, Palette};

// ─────────────────────────── APP STATE ─────────────────────

pub struct CasioApp {
    engine:      CalcEngine,
    model:       ModelType,
    input:       String,
    top_line:    String,
    error:       bool,
    shift_mode:  bool,
    alpha_mode:  bool,
    hyp_mode:    bool,
    show_history:bool,
    palette:     Palette,
}

impl CasioApp {
    pub fn new(_cc: &eframe::CreationContext<'_>, model: ModelType) -> Self {
        Self {
            engine:      CalcEngine::new(),
            palette:     Palette::for_model(model),
            model,
            input:       "0".to_string(),
            top_line:    String::new(),
            error:       false,
            shift_mode:  false,
            alpha_mode:  false,
            hyp_mode:    false,
            show_history:false,
        }
    }
}

impl eframe::App for CasioApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.handle_keyboard(ctx);
        // Dark background
        ctx.set_visuals(egui::Visuals::dark());

        egui::CentralPanel::default()
            .frame(Frame::none().fill(Color32::from_rgb(8, 8, 18)))
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    self.draw_calculator(ui);
                });
            });
    }
}

impl CasioApp {
    fn draw_calculator(&mut self, ui: &mut Ui) {
        let p = &self.palette;
        let body_color = p.body;
        let dark_color = p.body_dark;

        // Outer shell
        let (rect, _) = ui.allocate_exact_size(Vec2::new(340.0, 720.0), Sense::hover());
        let painter = ui.painter();

        // Shadow
        painter.rect_filled(
            rect.translate(Vec2::new(5.0, 8.0)),
            Rounding::same(18.0),
            Color32::from_black_alpha(120),
        );

        // Body
        painter.rect_filled(rect, Rounding::same(18.0), body_color);
        painter.rect_stroke(rect, Rounding::same(18.0), Stroke::new(2.0, dark_color));

        // Inner UI
        let inner = rect.shrink(10.0);
        ui.allocate_ui_at_rect(inner, |ui| {
            ui.spacing_mut().item_spacing = Vec2::new(0.0, 0.0);
            self.draw_header(ui);
            self.draw_display(ui);
            self.draw_model_switcher(ui);
            self.draw_buttons(ui);
        });
    }


    fn handle_keyboard(&mut self, ctx: &egui::Context) {
        ctx.input(|i| {
            for event in &i.events {
                match event {
                    // קליטת טקסט רגיל (מספרים ופעולות)
                    egui::Event::Text(text) => {
                        match text.as_str() {
                            "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" | "." => {
                                self.handle_button(text);
                            }
                            "+" | "-" => self.handle_button(text),
                  "*" => self.handle_button("×"), // המרה לסמל של המחשבון
                  "/" => self.handle_button("÷"), // המרה לסמל של המחשבון
                  _ => {}
                        }
                    }
                    // קליטת מקשים מיוחדים (Enter, Backspace, Escape)
                    egui::Event::Key { key, pressed: true, .. } => {
                        match key {
                            egui::Key::Enter => self.handle_button("="),
                  egui::Key::Backspace => self.handle_button("DEL"),
                  egui::Key::Escape => self.handle_button("AC"),
                  _ => {}
                        }
                    }
                    _ => {}
                }
            }
        });
    }















    fn draw_header(&mut self, ui: &mut Ui) {
        let p = &self.palette;
        ui.add_space(6.0);
        ui.horizontal(|ui| {
            ui.add_space(6.0);
            ui.label(
                RichText::new("CASIO")
                    .font(FontId::proportional(26.0))
                    .strong()
                    .color(p.casio_text),
            );
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.add_space(8.0);
                // Solar panel decoration
                let (rect, _) = ui.allocate_exact_size(Vec2::new(50.0, 12.0), Sense::hover());
                let painter = ui.painter();
                painter.rect_filled(rect, Rounding::same(2.0), Color32::from_rgb(30, 30, 40));
                for i in 0..5 {
                    let x = rect.left() + 2.0 + i as f32 * 10.0;
                    painter.rect_filled(
                        Rect::from_min_size(Pos2::new(x, rect.top() + 1.0), Vec2::new(8.0, 10.0)),
                        Rounding::same(1.0),
                        Color32::from_rgb(50, 60, 80),
                    );
                }
                ui.add_space(4.0);
                ui.label(
                    RichText::new(self.model.label())
                        .font(FontId::proportional(11.0))
                        .color(p.casio_text),
                );
            });
        });
        ui.add_space(2.0);
        ui.horizontal(|ui| {
            ui.add_space(8.0);
            ui.label(
                RichText::new(self.model.subtitle())
                    .font(FontId::proportional(9.0))
                    .color(Color32::from_white_alpha(160)),
            );
        });
        ui.add_space(4.0);
    }

    fn draw_display(&mut self, ui: &mut Ui) {
        let p = &self.palette;
        Frame::none()
            .fill(p.display_bg)
            .inner_margin(egui::Margin::symmetric(10.0, 8.0))
            .rounding(Rounding::same(4.0))
            .stroke(Stroke::new(2.0, Color32::from_black_alpha(150)))
            .show(ui, |ui| {
                ui.set_min_width(310.0);

                // Status bar
                ui.horizontal(|ui| {
                    // Shift/Alpha indicators
                    if self.shift_mode {
                        ui.label(RichText::new("S").font(FontId::monospace(10.0)).color(Color32::from_rgb(255, 160, 0)));
                    }
                    if self.alpha_mode {
                        ui.label(RichText::new("A").font(FontId::monospace(10.0)).color(Color32::from_rgb(220, 60, 60)));
                    }
                    if self.hyp_mode {
                        ui.label(RichText::new("HYP").font(FontId::monospace(9.0)).color(Color32::from_rgb(80, 160, 255)));
                    }
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.label(
                            RichText::new("Math")
                                .font(FontId::monospace(9.0))
                                .color(p.display_text.linear_multiply(0.6)),
                        );
                        ui.add_space(4.0);
                        ui.label(
                            RichText::new(self.engine.angle.label())
                                .font(FontId::monospace(9.0))
                                .color(p.display_text.linear_multiply(0.6)),
                        );
                    });
                });

                // Top line (expression)
                if !self.top_line.is_empty() {
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Min), |ui| {
                        ui.label(
                            RichText::new(&self.top_line)
                                .font(FontId::monospace(11.0))
                                .color(p.display_text.linear_multiply(0.7)),
                        );
                    });
                }

                // Main display line
                ui.add_space(2.0);
                let font_size = if self.input.len() > 14 { 18.0 } else { 30.0 };
                let color = if self.error { Color32::from_rgb(200, 30, 30) } else { p.display_text };
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Min), |ui| {
                    ui.label(
                        RichText::new(&self.input)
                            .font(FontId::monospace(font_size))
                            .color(color)
                            .strong(),
                    );
                });

                ui.add_space(2.0);
            });

        ui.add_space(6.0);
    }

    fn draw_model_switcher(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.add_space(4.0);
            for m in [ModelType::Fx82MS, ModelType::Fx991ES, ModelType::FxCG50] {
                let active = self.model == m;
                let color = if active {
                    Color32::WHITE
                } else {
                    Color32::from_white_alpha(120)
                };
                let bg = if active {
                    Color32::from_rgb(60, 80, 120)
                } else {
                    Color32::from_black_alpha(60)
                };
                let (rect, resp) = ui.allocate_exact_size(Vec2::new(90.0, 18.0), Sense::click());
                ui.painter().rect_filled(rect, Rounding::same(4.0), bg);
                ui.painter().text(
                    rect.center(),
                    Align2::CENTER_CENTER,
                    m.label(),
                    FontId::proportional(9.5),
                    color,
                );
                if resp.clicked() {
                    self.model = m;
                    self.palette = Palette::for_model(m);
                }
                ui.add_space(2.0);
            }
        });
        ui.add_space(6.0);
    }

    fn draw_buttons(&mut self, ui: &mut Ui) {
        let rows = button_grid(self.model);
        for row in &rows {
            ui.horizontal(|ui| {
                ui.add_space(2.0);
                let btn_w = (310.0 - (row.len() as f32 - 1.0) * 4.0) / row.len() as f32;
                for btn in row {
                    if self.draw_button(ui, btn, btn_w) {
                        self.handle_button(btn.label);
                    }
                    ui.add_space(4.0);
                }
            });
            ui.add_space(4.0);
        }
    }

    fn draw_button(&self, ui: &mut Ui, btn: &BtnDef, width: f32) -> bool {
        let p = &self.palette;
        let height = 36.0;

        let base_color = match btn.color {
            BtnColor::Num   => p.btn_num,
            BtnColor::Op    => p.btn_op,
            BtnColor::Fn    => p.btn_fn,
            BtnColor::Ctrl  => p.btn_ctrl,
            BtnColor::Eq    => p.btn_eq,
            BtnColor::Del   => p.btn_del,
            BtnColor::Ac    => p.btn_ac,
            BtnColor::Shift => if self.shift_mode { Color32::from_rgb(220, 160, 20) } else { p.btn_shift },
            BtnColor::Alpha => if self.alpha_mode { Color32::from_rgb(200, 60, 60) } else { p.btn_alpha },
        };

        let (rect, resp) = ui.allocate_exact_size(Vec2::new(width, height), Sense::click());
        let painter = ui.painter();
        let is_hovered = resp.hovered();
        let is_pressed = resp.is_pointer_button_down_on();

        // 3D button shadow
        painter.rect_filled(
            rect.translate(Vec2::new(1.0, 2.0)),
            Rounding::same(5.0),
            Color32::from_black_alpha(120),
        );

        // Button face
        let face_color = if is_pressed {
            darken(base_color, 0.7)
        } else if is_hovered {
            lighten(base_color, 1.15)
        } else {
            base_color
        };
        painter.rect_filled(rect, Rounding::same(5.0), face_color);

        // Shine (top highlight)
        let shine_rect = Rect::from_min_size(
            rect.min + Vec2::new(1.0, 1.0),
            Vec2::new(rect.width() - 2.0, rect.height() * 0.4),
        );
        painter.rect_filled(
            shine_rect,
            Rounding { nw: 5.0, ne: 5.0, sw: 0.0, se: 0.0 },
            Color32::from_white_alpha(25),
        );
        painter.rect_stroke(rect, Rounding::same(5.0), Stroke::new(1.0, darken(base_color, 0.6)));

        // Shift label (small, top)
        if let Some(sl) = btn.shift_label {
            painter.text(
                rect.left_top() + Vec2::new(3.0, 1.0),
                Align2::LEFT_TOP,
                sl,
                FontId::proportional(7.0),
                p.shadow_text,
            );
        }

        // Alpha label (small, top-right)
        if let Some(al) = btn.alpha_label {
            painter.text(
                rect.right_top() + Vec2::new(-2.0, 1.0),
                Align2::RIGHT_TOP,
                al,
                FontId::proportional(7.0),
                Color32::from_rgb(120, 210, 255),
            );
        }

        // Main label
        let fs = if btn.label.len() > 4 { 10.0 } else if btn.label.len() > 2 { 12.0 } else { 16.0 };
        painter.text(
            rect.center() + if btn.shift_label.is_some() { Vec2::new(0.0, 3.0) } else { Vec2::ZERO },
            Align2::CENTER_CENTER,
            btn.label,
            FontId::monospace(fs),
            p.btn_text,
        );

        resp.clicked()
    }

    fn handle_button(&mut self, label: &str) {
        self.error = false;

        match label {
            "AC" => {
                self.input = "0".to_string();
                self.top_line.clear();
                self.shift_mode = false;
                self.alpha_mode = false;
                self.hyp_mode = false;
                self.error = false;
            }

            "DEL" => {
                if self.input.len() > 1 {
                    self.input.pop();
                } else {
                    self.input = "0".to_string();
                }
            }

            "=" | "EXE" => {
                let expr = self.input
                    .replace("×", "*")
                    .replace("÷", "/")
                    .replace("−", "-")
                    .replace("×10^x", "*10^");

                match self.engine.evaluate(&expr) {
                    Ok(val) => {
                        self.top_line = format!("{}=", self.input);
                        self.input = self.engine.format_result(val);
                    }
                    Err(e) => {
                        self.top_line = self.input.clone();
                        self.input = e;
                        self.error = true;
                    }
                }
                self.shift_mode = false;
                self.alpha_mode = false;
                self.hyp_mode = false;
            }

            "SHIFT" => {
                self.shift_mode = !self.shift_mode;
                self.alpha_mode = false;
            }

            "ALPHA" => {
                self.alpha_mode = !self.alpha_mode;
                self.shift_mode = false;
            }

            "MODE" => {
                self.engine.cycle_angle();
            }

            "ON" => {
                self.input = "0".to_string();
                self.top_line.clear();
                self.shift_mode = false;
                self.alpha_mode = false;
                self.hyp_mode = false;
                self.error = false;
                self.engine = CalcEngine::new();
            }

            "hyp" => {
                self.hyp_mode = !self.hyp_mode;
            }

            "Ans" => self.append("Ans"),

            "×10^x" => self.append("×10^"),

            "sin" | "cos" | "tan" => {
                let fn_name = if self.shift_mode {
                    match label {
                        "sin" => "asin",
                        "cos" => "acos",
                        "tan" => "atan",
                        _     => label,
                    }
                } else if self.hyp_mode {
                    match label {
                        "sin" => "sinh",
                        "cos" => "cosh",
                        "tan" => "tanh",
                        _     => label,
                    }
                } else {
                    label
                };
                self.append(&format!("{}(", fn_name));
                self.shift_mode = false;
                self.hyp_mode = false;
            }

            "log" => {
                if self.shift_mode {
                    self.append("10^(");
                } else {
                    self.append("log(");
                }
                self.shift_mode = false;
            }

            "ln" => {
                if self.shift_mode {
                    self.append("exp(");
                } else {
                    self.append("ln(");
                }
                self.shift_mode = false;
            }

            "√" => {
                self.append(if self.shift_mode { "x√(" } else { "sqrt(" });
                self.shift_mode = false;
            }

            "∛x" => { self.append("cbrt("); }

            "x²" => {
                self.append(if self.shift_mode { "^(0.5)" } else { "^2" });
                self.shift_mode = false;
            }

            "x⁻¹" => {
                if self.shift_mode {
                    self.append("!");
                    self.shift_mode = false;
                } else {
                    self.append("^(-1)");
                }
            }

            "nCr" => {
                if self.shift_mode {
                    self.append("nPr(");
                    self.shift_mode = false;
                } else {
                    self.append("nCr(");
                }
            }

            "(-)" => {
                if self.input == "0" {
                    self.input = "-".to_string();
                } else {
                    self.append("×(-1)");
                }
            }

            "M+" => {
                if self.shift_mode {
                    if let Ok(val) = self.engine.evaluate(&self.input.replace("×","*").replace("÷","/").replace("−","-")) {
                        self.engine.m_minus_op(val);
                        self.top_line = format!("M = {}", self.engine.format_result(self.engine.recall_m()));
                    }
                    self.shift_mode = false;
                } else {
                    if let Ok(val) = self.engine.evaluate(&self.input.replace("×","*").replace("÷","/").replace("−","-")) {
                        self.engine.m_plus_op(val);
                        self.top_line = format!("M = {}", self.engine.format_result(self.engine.recall_m()));
                    }
                }
            }

            "RCL" => {
                let m = self.engine.recall_m();
                self.top_line = format!("M = {}", self.engine.format_result(m));
                self.append(&self.engine.format_result(m).clone());
            }

            "ENG" => {
                if let Ok(val) = self.engine.evaluate(&self.input.replace("×","*").replace("÷","/").replace("−","-")) {
                    use crate::engine::DisplayFormat;
                    self.engine.format = DisplayFormat::Engineering;
                    self.input = self.engine.format_result(val);
                    self.engine.format = DisplayFormat::Normal;
                }
            }

            "°'\"" => { self.append("°"); }

            _ => {
                // Regular character append
                self.append(label);
            }
        }
    }

    fn append(&mut self, s: &str) {
        if self.input == "0" && s.chars().next().map(|c| c.is_ascii_digit()).unwrap_or(false) {
            self.input = s.to_string();
        } else if self.error {
            self.input = s.to_string();
            self.error = false;
        } else {
            self.input.push_str(s);
        }
    }
}

// ─── Color helpers ──────────────────────────────────────────

fn darken(c: Color32, factor: f32) -> Color32 {
    Color32::from_rgb(
        (c.r() as f32 * factor) as u8,
        (c.g() as f32 * factor) as u8,
        (c.b() as f32 * factor) as u8,
    )
}

fn lighten(c: Color32, factor: f32) -> Color32 {
    Color32::from_rgb(
        ((c.r() as f32 * factor).min(255.0)) as u8,
        ((c.g() as f32 * factor).min(255.0)) as u8,
        ((c.b() as f32 * factor).min(255.0)) as u8,
    )
}
