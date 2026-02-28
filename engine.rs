// ============================================================
//  engine.rs — Mathematical Engine
//  Full scientific calculator: tokenizer → parser → evaluator
// ============================================================

use std::collections::HashMap;
use std::f64::consts::{PI, E};

// ─────────────────────────── TOKENS ────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Number(f64),
    Plus, Minus, Mul, Div, Pow,
    LParen, RParen,
    Func(String),
    Const(String),
    Comma,
    Factorial,
    Percent,
}

// ─────────────────────────── ANGLE MODE ────────────────────

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AngleMode {
    Degrees,
    Radians,
    Gradians,
}

impl AngleMode {
    pub fn to_rad(self, v: f64) -> f64 {
        match self {
            AngleMode::Degrees  => v * PI / 180.0,
            AngleMode::Radians  => v,
            AngleMode::Gradians => v * PI / 200.0,
        }
    }
    pub fn from_rad(self, v: f64) -> f64 {
        match self {
            AngleMode::Degrees  => v * 180.0 / PI,
            AngleMode::Radians  => v,
            AngleMode::Gradians => v * 200.0 / PI,
        }
    }
    pub fn label(self) -> &'static str {
        match self {
            AngleMode::Degrees  => "D",
            AngleMode::Radians  => "R",
            AngleMode::Gradians => "G",
        }
    }
}

// ─────────────────────────── DISPLAY FORMAT ────────────────

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DisplayFormat {
    Normal,
    Scientific,
    Engineering,
    Fix(u8),
}

// ─────────────────────────── ENGINE ────────────────────────

pub struct CalcEngine {
    pub angle: AngleMode,
    pub format: DisplayFormat,
    pub ans:    f64,
    pub memory: HashMap<char, f64>,
    pub m_plus: f64,
    pub history: Vec<(String, f64)>,
}

impl Default for CalcEngine {
    fn default() -> Self {
        let mut memory = HashMap::new();
        for c in ['A','B','C','D','E','F','X','Y','M'] {
            memory.insert(c, 0.0);
        }
        Self {
            angle:   AngleMode::Degrees,
            format:  DisplayFormat::Normal,
            ans:     0.0,
            memory,
            m_plus:  0.0,
            history: Vec::new(),
        }
    }
}

impl CalcEngine {
    pub fn new() -> Self { Self::default() }

    pub fn cycle_angle(&mut self) {
        self.angle = match self.angle {
            AngleMode::Degrees  => AngleMode::Radians,
            AngleMode::Radians  => AngleMode::Gradians,
            AngleMode::Gradians => AngleMode::Degrees,
        };
    }

    pub fn store(&mut self, var: char, val: f64) {
        self.memory.insert(var, val);
    }

    pub fn recall(&self, var: char) -> f64 {
        *self.memory.get(&var).unwrap_or(&0.0)
    }

    pub fn m_plus_op(&mut self, val: f64) { self.m_plus += val; }
    pub fn m_minus_op(&mut self, val: f64) { self.m_plus -= val; }
    pub fn recall_m(&self) -> f64 { self.m_plus }
    pub fn clear_m(&mut self) { self.m_plus = 0.0; }

    /// Format a number for the CASIO display (10 digits max)
    pub fn format_result(&self, val: f64) -> String {
        if val.is_nan()      { return "Math ERROR".to_string(); }
        if val.is_infinite() { return if val > 0.0 { "∞".to_string() } else { "-∞".to_string() }; }

        match self.format {
            DisplayFormat::Scientific  => format_scientific(val, 9),
            DisplayFormat::Engineering => format_engineering(val),
            DisplayFormat::Fix(n)      => format!("{:.prec$}", val, prec = n as usize),
            DisplayFormat::Normal      => format_normal(val),
        }
    }

    /// Evaluate a string expression
    pub fn evaluate(&mut self, expr: &str) -> Result<f64, String> {
        let tokens = tokenize(expr, self.ans, &self.memory)?;
        let mut parser = Parser::new(tokens, self.angle);
        let result = parser.parse_expr()?;

        if result.is_nan()      { return Err("Math ERROR".to_string()); }
        if result.is_infinite() { return Err("Math ERROR (overflow)".to_string()); }

        self.ans = result;
        self.history.push((expr.to_string(), result));
        if self.history.len() > 50 { self.history.remove(0); }

        Ok(result)
    }
}

// ─────────────────────────── FORMATTER ─────────────────────

fn format_normal(val: f64) -> String {
    if val == 0.0 { return "0".to_string(); }
    let abs = val.abs();

    if abs < 1e-9 || abs >= 1e10 {
        return format_scientific(val, 9);
    }

    // Try integer first
    if val == val.trunc() && abs < 1e15 {
        return format!("{}", val as i64);
    }

    // Up to 10 significant digits, trim trailing zeros
    let s = format!("{:.10}", val);
    let s = s.trim_end_matches('0').trim_end_matches('.');
    s.to_string()
}

fn format_scientific(val: f64, prec: usize) -> String {
    if val == 0.0 { return "0".to_string(); }
    let exp = val.abs().log10().floor() as i32;
    let mantissa = val / 10f64.powi(exp);
    let s = format!("{:.prec$}", mantissa, prec = prec);
    let s = s.trim_end_matches('0').trim_end_matches('.');
    format!("{}×10^{}", s, exp)
}

fn format_engineering(val: f64) -> String {
    if val == 0.0 { return "0".to_string(); }
    let exp = val.abs().log10().floor() as i32;
    let eng_exp = (exp as f64 / 3.0).floor() as i32 * 3;
    let mantissa = val / 10f64.powi(eng_exp);
    format!("{:.3}×10^{}", mantissa, eng_exp)
}

// ─────────────────────────── TOKENIZER ─────────────────────

fn tokenize(
    input: &str,
    ans: f64,
    memory: &HashMap<char, f64>,
) -> Result<Vec<Token>, String> {
    let mut tokens = Vec::new();
    let chars: Vec<char> = input.chars().collect();
    let mut i = 0;

    // Known function names (longest first to avoid prefix clash)
    let funcs = [
        "asinh","acosh","atanh","asin","acos","atan",
        "sinh","cosh","tanh","sin","cos","tan",
        "log₂","log","ln","sqrt","cbrt","abs","exp",
        "nCr","nPr","Rec","Pol",
    ];

    while i < chars.len() {
        let c = chars[i];

        // Skip spaces
        if c == ' ' { i += 1; continue; }

        // Number (including scientific notation: 1.5e3)
        if c.is_ascii_digit() || c == '.' {
            let start = i;
            while i < chars.len() && (chars[i].is_ascii_digit() || chars[i] == '.') {
                i += 1;
            }
            // optional exponent
            if i < chars.len() && (chars[i] == 'e' || chars[i] == 'E') {
                i += 1;
                if i < chars.len() && (chars[i] == '+' || chars[i] == '-') { i += 1; }
                while i < chars.len() && chars[i].is_ascii_digit() { i += 1; }
            }
            let s: String = chars[start..i].iter().collect();
            let v: f64 = s.parse().map_err(|_| format!("Bad number: {}", s))?;
            tokens.push(Token::Number(v));
            continue;
        }

        // Ans
        if chars[i..].iter().collect::<String>().starts_with("Ans") {
            tokens.push(Token::Number(ans));
            i += 3;
            continue;
        }

        // π and e constants
        if c == 'π' { tokens.push(Token::Number(PI)); i += 1; continue; }
        if c == 'e' && (i + 1 >= chars.len() || !chars[i+1].is_alphanumeric()) {
            tokens.push(Token::Number(E));
            i += 1;
            continue;
        }

        // Memory variables A..F X Y M
        if "ABCDEFXYMm".contains(c) && (i + 1 >= chars.len() || !chars[i+1].is_alphanumeric()) {
            let key = c.to_ascii_uppercase();
            tokens.push(Token::Number(*memory.get(&key).unwrap_or(&0.0)));
            i += 1;
            continue;
        }

        // Functions
        let rest: String = chars[i..].iter().collect();
        let mut matched = false;
        for &fn_name in &funcs {
            if rest.starts_with(fn_name) {
                tokens.push(Token::Func(fn_name.to_string()));
                i += fn_name.len();
                matched = true;
                break;
            }
        }
        if matched { continue; }

        // Operators & punctuation
        match c {
            '+' => tokens.push(Token::Plus),
            '-' => tokens.push(Token::Minus),
            '*' | '×' => tokens.push(Token::Mul),
            '/' | '÷' => tokens.push(Token::Div),
            '^' => tokens.push(Token::Pow),
            '(' => tokens.push(Token::LParen),
            ')' => tokens.push(Token::RParen),
            ',' => tokens.push(Token::Comma),
            '!' => tokens.push(Token::Factorial),
            '%' => tokens.push(Token::Percent),
            _ => return Err(format!("Unknown character: '{}'", c)),
        }
        i += 1;
    }

    Ok(tokens)
}

// ─────────────────────────── PARSER ────────────────────────
// Recursive descent: expr → term → power → unary → primary

struct Parser {
    tokens: Vec<Token>,
    pos:    usize,
    angle:  AngleMode,
}

impl Parser {
    fn new(tokens: Vec<Token>, angle: AngleMode) -> Self {
        Self { tokens, pos: 0, angle }
    }

    fn peek(&self) -> Option<&Token> { self.tokens.get(self.pos) }
    fn next(&mut self) -> Option<Token> {
        if self.pos < self.tokens.len() {
            let t = self.tokens[self.pos].clone();
            self.pos += 1;
            Some(t)
        } else {
            None
        }
    }

    pub fn parse_expr(&mut self) -> Result<f64, String> {
        self.parse_add_sub()
    }

    fn parse_add_sub(&mut self) -> Result<f64, String> {
        let mut left = self.parse_mul_div()?;
        loop {
            match self.peek() {
                Some(Token::Plus)  => { self.next(); left += self.parse_mul_div()?; }
                Some(Token::Minus) => { self.next(); left -= self.parse_mul_div()?; }
                _ => break,
            }
        }
        Ok(left)
    }

    fn parse_mul_div(&mut self) -> Result<f64, String> {
        let mut left = self.parse_power()?;
        loop {
            match self.peek() {
                Some(Token::Mul) => { self.next(); left *= self.parse_power()?; }
                Some(Token::Div) => {
                    self.next();
                    let r = self.parse_power()?;
                    if r == 0.0 { return Err("Math ERROR (div/0)".to_string()); }
                    left /= r;
                }
                _ => break,
            }
        }
        Ok(left)
    }

    fn parse_power(&mut self) -> Result<f64, String> {
        let base = self.parse_postfix()?;
        if self.peek() == Some(&Token::Pow) {
            self.next();
            let exp = self.parse_unary()?; // right-assoc
            return Ok(base.powf(exp));
        }
        Ok(base)
    }

    fn parse_postfix(&mut self) -> Result<f64, String> {
        let mut val = self.parse_unary()?;
        loop {
            match self.peek() {
                Some(Token::Factorial) => {
                    self.next();
                    val = factorial(val)?;
                }
                Some(Token::Percent) => {
                    self.next();
                    val /= 100.0;
                }
                _ => break,
            }
        }
        Ok(val)
    }

    fn parse_unary(&mut self) -> Result<f64, String> {
        match self.peek() {
            Some(Token::Minus) => { self.next(); Ok(-self.parse_primary()?) }
            Some(Token::Plus)  => { self.next(); self.parse_primary() }
            _ => self.parse_primary(),
        }
    }

    fn parse_primary(&mut self) -> Result<f64, String> {
        match self.next() {
            Some(Token::Number(v)) => Ok(v),

            Some(Token::LParen) => {
                let v = self.parse_expr()?;
                if self.peek() == Some(&Token::RParen) { self.next(); }
                Ok(v)
            }

            Some(Token::Func(name)) => {
                // Expect '(' argument ')'
                if self.peek() == Some(&Token::LParen) { self.next(); }
                let arg = self.parse_expr()?;

                // Two-arg functions: nCr, nPr, Rec, Pol
                let result = if ["nCr","nPr","Rec","Pol"].contains(&name.as_str()) {
                    if self.peek() == Some(&Token::Comma) { self.next(); }
                    let arg2 = self.parse_expr()?;
                    if self.peek() == Some(&Token::RParen) { self.next(); }
                    apply_two_arg_func(&name, arg, arg2)?
                } else {
                    if self.peek() == Some(&Token::RParen) { self.next(); }
                    self.apply_func(&name, arg)?
                };

                Ok(result)
            }

            other => Err(format!("Unexpected token: {:?}", other)),
        }
    }

    fn apply_func(&self, name: &str, arg: f64) -> Result<f64, String> {
        let r = self.angle.to_rad(arg);
        let ok = |v: f64| Ok(v);

        match name {
            "sin"   => ok(r.sin()),
            "cos"   => ok(r.cos()),
            "tan"   => {
                if (r.cos()).abs() < 1e-12 { return Err("Math ERROR (tan undef)".to_string()); }
                ok(r.tan())
            }
            "asin"  => {
                if arg.abs() > 1.0 { return Err("Math ERROR".to_string()); }
                ok(self.angle.from_rad(arg.asin()))
            }
            "acos"  => {
                if arg.abs() > 1.0 { return Err("Math ERROR".to_string()); }
                ok(self.angle.from_rad(arg.acos()))
            }
            "atan"  => ok(self.angle.from_rad(arg.atan())),
            "sinh"  => ok(arg.sinh()),
            "cosh"  => ok(arg.cosh()),
            "tanh"  => ok(arg.tanh()),
            "asinh" => ok(arg.asinh()),
            "acosh" => {
                if arg < 1.0 { return Err("Math ERROR".to_string()); }
                ok(arg.acosh())
            }
            "atanh" => {
                if arg.abs() >= 1.0 { return Err("Math ERROR".to_string()); }
                ok(arg.atanh())
            }
            "log"   => {
                if arg <= 0.0 { return Err("Math ERROR".to_string()); }
                ok(arg.log10())
            }
            "log₂"  => {
                if arg <= 0.0 { return Err("Math ERROR".to_string()); }
                ok(arg.log2())
            }
            "ln"    => {
                if arg <= 0.0 { return Err("Math ERROR".to_string()); }
                ok(arg.ln())
            }
            "sqrt"  => {
                if arg < 0.0 { return Err("Math ERROR".to_string()); }
                ok(arg.sqrt())
            }
            "cbrt"  => ok(arg.cbrt()),
            "abs"   => ok(arg.abs()),
            "exp"   => ok(arg.exp()),
            _ => Err(format!("Unknown function: {}", name)),
        }
    }
}

fn apply_two_arg_func(name: &str, a: f64, b: f64) -> Result<f64, String> {
    match name {
        "nCr" => {
            let n = a as u64;
            let r = b as u64;
            if r > n { return Err("Math ERROR".to_string()); }
            Ok(combinations(n, r) as f64)
        }
        "nPr" => {
            let n = a as u64;
            let r = b as u64;
            if r > n { return Err("Math ERROR".to_string()); }
            Ok(permutations(n, r) as f64)
        }
        "Rec" => {
            // Rec(r, θ) → x = r·cos(θ), but we return x here; y shown separately
            Ok(a * b.to_radians().cos())
        }
        "Pol" => {
            // Pol(x, y) → r = √(x²+y²)
            Ok((a * a + b * b).sqrt())
        }
        _ => Err(format!("Unknown 2-arg function: {}", name)),
    }
}

// ─────────────────────────── HELPERS ───────────────────────

fn factorial(n: f64) -> Result<f64, String> {
    if n < 0.0 || n != n.trunc() || n > 69.0 {
        return Err("Math ERROR".to_string());
    }
    let mut result = 1u128;
    for i in 2..=(n as u64) { result *= i as u128; }
    Ok(result as f64)
}

fn combinations(n: u64, r: u64) -> u128 {
    if r == 0 || r == n { return 1; }
    let r = r.min(n - r);
    let mut result = 1u128;
    for i in 0..r {
        result = result * (n - i) as u128 / (i + 1) as u128;
    }
    result
}

fn permutations(n: u64, r: u64) -> u128 {
    if r == 0 { return 1; }
    let mut result = 1u128;
    for i in 0..r { result *= (n - i) as u128; }
    result
}
