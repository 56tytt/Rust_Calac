// ============================================================
//  models.rs — 3 CASIO Models: fx-82MS | fx-991ES | fx-CG50
// ============================================================

use egui::Color32;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ModelType {
    Fx82MS,       // Pink/classic — S-V.P.A.M
    Fx991ES,      // Blue/grey   — Natural VPAM
    FxCG50,       // Black/color — Graphing
}

impl ModelType {
    pub fn label(self) -> &'static str {
        match self {
            ModelType::Fx82MS  => "fx-82MS",
            ModelType::Fx991ES => "fx-991ES PLUS",
            ModelType::FxCG50  => "fx-CG50",
        }
    }

    pub fn subtitle(self) -> &'static str {
        match self {
            ModelType::Fx82MS  => "S-V.P.A.M.  2nd edition",
            ModelType::Fx991ES => "NATURAL-VPAM  2nd edition",
            ModelType::FxCG50  => "GRAPH  COLOR",
        }
    }
}

// ─── Color palette per model ───────────────────────────────

pub struct Palette {
    pub body:         Color32,
    pub body_dark:    Color32,
    pub display_bg:   Color32,
    pub display_text: Color32,
    pub btn_num:      Color32,
    pub btn_op:       Color32,
    pub btn_fn:       Color32,
    pub btn_ctrl:     Color32,
    pub btn_eq:       Color32,
    pub btn_del:      Color32,
    pub btn_ac:       Color32,
    pub btn_shift:    Color32,
    pub btn_alpha:    Color32,
    pub btn_text:     Color32,
    pub shadow_text:  Color32,
    pub border:       Color32,
    pub casio_text:   Color32,
}

impl Palette {
    pub fn for_model(model: ModelType) -> Self {
        match model {
            ModelType::Fx82MS => Self {
                body:         Color32::from_rgb(220, 185, 185),
                body_dark:    Color32::from_rgb(170, 130, 130),
                display_bg:   Color32::from_rgb(200, 215, 185),
                display_text: Color32::from_rgb(10, 30, 10),
                btn_num:      Color32::from_rgb(195, 160, 160),
                btn_op:       Color32::from_rgb(175, 135, 135),
                btn_fn:       Color32::from_rgb(155, 115, 115),
                btn_ctrl:     Color32::from_rgb(140, 100, 110),
                btn_eq:       Color32::from_rgb(190, 150, 120),
                btn_del:      Color32::from_rgb(170, 145, 130),
                btn_ac:       Color32::from_rgb(190, 150, 130),
                btn_shift:    Color32::from_rgb(130, 90, 90),
                btn_alpha:    Color32::from_rgb(130, 90, 90),
                btn_text:     Color32::WHITE,
                shadow_text:  Color32::from_rgb(255, 200, 100),
                border:       Color32::from_rgb(130, 90, 90),
                casio_text:   Color32::WHITE,
            },
            ModelType::Fx991ES => Self {
                body:         Color32::from_rgb(138, 150, 185),
                body_dark:    Color32::from_rgb(90, 105, 140),
                display_bg:   Color32::from_rgb(195, 215, 175),
                display_text: Color32::from_rgb(5, 20, 5),
                btn_num:      Color32::from_rgb(60, 70, 100),
                btn_op:       Color32::from_rgb(75, 45, 55),
                btn_fn:       Color32::from_rgb(50, 60, 90),
                btn_ctrl:     Color32::from_rgb(40, 50, 75),
                btn_eq:       Color32::from_rgb(140, 80, 30),
                btn_del:      Color32::from_rgb(35, 80, 35),
                btn_ac:       Color32::from_rgb(100, 25, 25),
                btn_shift:    Color32::from_rgb(90, 70, 15),
                btn_alpha:    Color32::from_rgb(100, 25, 25),
                btn_text:     Color32::WHITE,
                shadow_text:  Color32::from_rgb(255, 200, 80),
                border:       Color32::from_rgb(50, 60, 90),
                casio_text:   Color32::WHITE,
            },
            ModelType::FxCG50 => Self {
                body:         Color32::from_rgb(30, 30, 35),
                body_dark:    Color32::from_rgb(15, 15, 20),
                display_bg:   Color32::from_rgb(10, 15, 30),
                display_text: Color32::from_rgb(80, 255, 120),
                btn_num:      Color32::from_rgb(40, 40, 50),
                btn_op:       Color32::from_rgb(60, 35, 20),
                btn_fn:       Color32::from_rgb(30, 50, 70),
                btn_ctrl:     Color32::from_rgb(25, 25, 35),
                btn_eq:       Color32::from_rgb(180, 80, 10),
                btn_del:      Color32::from_rgb(20, 70, 20),
                btn_ac:       Color32::from_rgb(90, 15, 15),
                btn_shift:    Color32::from_rgb(80, 60, 5),
                btn_alpha:    Color32::from_rgb(100, 20, 20),
                btn_text:     Color32::WHITE,
                shadow_text:  Color32::from_rgb(100, 200, 255),
                border:       Color32::from_rgb(20, 20, 28),
                casio_text:   Color32::WHITE,
            },
        }
    }
}

// ─── Button definitions per model ──────────────────────────

#[derive(Debug, Clone)]
pub struct BtnDef {
    pub label:       &'static str,
    pub shift_label: Option<&'static str>,
    pub alpha_label: Option<&'static str>,
    pub color:       BtnColor,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BtnColor { Num, Op, Fn, Ctrl, Eq, Del, Ac, Shift, Alpha }

impl BtnDef {
    fn new(label: &'static str, color: BtnColor) -> Self {
        Self { label, shift_label: None, alpha_label: None, color }
    }
    fn with_shift(mut self, s: &'static str) -> Self { self.shift_label = Some(s); self }
    fn with_alpha(mut self, a: &'static str) -> Self { self.alpha_label = Some(a); self }
}

/// Returns the full button grid for a given model
/// Each inner Vec is one row.
pub fn button_grid(model: ModelType) -> Vec<Vec<BtnDef>> {
    use BtnColor::*;

    match model {
        ModelType::Fx82MS | ModelType::Fx991ES => {
            vec![
                // Row 0: SHIFT ALPHA ▲▼◀▶ MODE CLR ON
                vec![
                    BtnDef::new("SHIFT", Shift),
                    BtnDef::new("ALPHA", Alpha),
                    BtnDef::new("MODE", Ctrl),
                    BtnDef::new("ON", Ctrl),
                ],
                // Row 1: x⁻¹ nCr Pol x³
                vec![
                    BtnDef::new("x⁻¹", Fn).with_shift("x!"),
                    BtnDef::new("nCr", Fn).with_shift("nPr"),
                    BtnDef::new("Pol(", Fn).with_shift("Rec("),
                    BtnDef::new("∛x", Fn),
                ],
                // Row 2: a b/c  √  x²  ^  log  ln
                vec![
                    BtnDef::new("a b/c", Fn).with_shift("d/c"),
                    BtnDef::new("√", Fn).with_shift("x√"),
                    BtnDef::new("x²", Fn).with_shift("10^x"),
                    BtnDef::new("^", Op),
                    BtnDef::new("log", Fn).with_shift("e^x"),
                    BtnDef::new("ln", Fn),
                ],
                // Row 3: (-) °'" hyp sin cos tan
                vec![
                    BtnDef::new("(-)", Fn),
                    BtnDef::new("°'\"", Fn),
                    BtnDef::new("hyp", Fn),
                    BtnDef::new("sin", Fn).with_shift("sin⁻¹"),
                    BtnDef::new("cos", Fn).with_shift("cos⁻¹"),
                    BtnDef::new("tan", Fn).with_shift("tan⁻¹"),
                ],
                // Row 4: RCL ENG ( ) , M+
                vec![
                    BtnDef::new("RCL", Ctrl).with_shift("STO"),
                    BtnDef::new("ENG", Ctrl),
                    BtnDef::new("(", Op),
                    BtnDef::new(")", Op),
                    BtnDef::new(",", Op),
                    BtnDef::new("M+", Ctrl).with_shift("M-"),
                ],
                // Row 5: 7 8 9 DEL AC
                vec![
                    BtnDef::new("7", Num),
                    BtnDef::new("8", Num),
                    BtnDef::new("9", Num),
                    BtnDef::new("DEL", Del),
                    BtnDef::new("AC", Ac),
                ],
                // Row 6: 4 5 6 × ÷
                vec![
                    BtnDef::new("4", Num),
                    BtnDef::new("5", Num),
                    BtnDef::new("6", Num),
                    BtnDef::new("×", Op),
                    BtnDef::new("÷", Op),
                ],
                // Row 7: 1 2 3 + −
                vec![
                    BtnDef::new("1", Num),
                    BtnDef::new("2", Num).with_shift("Rnd").with_alpha("Ran#"),
                    BtnDef::new("3", Num),
                    BtnDef::new("+", Op),
                    BtnDef::new("−", Op),
                ],
                // Row 8: 0 . ×10^x Ans =
                vec![
                    BtnDef::new("0", Num),
                    BtnDef::new(".", Num),
                    BtnDef::new("×10^x", Fn),
                    BtnDef::new("Ans", Fn),
                    BtnDef::new("=", Eq),
                ],
            ]
        }

        ModelType::FxCG50 => {
            vec![
                // CG50 has more function keys
                vec![
                    BtnDef::new("SHIFT", Shift),
                    BtnDef::new("ALPHA", Alpha),
                    BtnDef::new("x,θ,T", Fn),
                    BtnDef::new("MENU", Ctrl),
                    BtnDef::new("ON", Ctrl),
                ],
                vec![
                    BtnDef::new("F1", Ctrl),
                    BtnDef::new("F2", Ctrl),
                    BtnDef::new("F3", Ctrl),
                    BtnDef::new("F4", Ctrl),
                    BtnDef::new("F5", Ctrl),
                    BtnDef::new("F6", Ctrl),
                ],
                vec![
                    BtnDef::new("x²", Fn).with_shift("√"),
                    BtnDef::new("^", Op).with_shift("x√"),
                    BtnDef::new("log", Fn).with_shift("10^x"),
                    BtnDef::new("ln", Fn).with_shift("e^x"),
                    BtnDef::new("sin", Fn).with_shift("sin⁻¹"),
                    BtnDef::new("cos", Fn).with_shift("cos⁻¹"),
                ],
                vec![
                    BtnDef::new("tan", Fn).with_shift("tan⁻¹"),
                    BtnDef::new("(-)", Fn),
                    BtnDef::new("EXP", Fn),
                    BtnDef::new("x⁻¹", Fn).with_shift("x!"),
                    BtnDef::new("DEL", Del).with_shift("INS"),
                    BtnDef::new("AC", Ac),
                ],
                vec![
                    BtnDef::new("7", Num),
                    BtnDef::new("8", Num),
                    BtnDef::new("9", Num),
                    BtnDef::new("(", Op),
                    BtnDef::new(")", Op),
                ],
                vec![
                    BtnDef::new("4", Num),
                    BtnDef::new("5", Num),
                    BtnDef::new("6", Num),
                    BtnDef::new("×", Op),
                    BtnDef::new("÷", Op),
                ],
                vec![
                    BtnDef::new("1", Num),
                    BtnDef::new("2", Num),
                    BtnDef::new("3", Num),
                    BtnDef::new("+", Op),
                    BtnDef::new("−", Op),
                ],
                vec![
                    BtnDef::new("0", Num),
                    BtnDef::new(".", Num),
                    BtnDef::new("×10^x", Fn),
                    BtnDef::new("Ans", Fn),
                    BtnDef::new("EXE", Eq),
                ],
            ]
        }
    }
}
