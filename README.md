# 🧮 CASIO Scientific Calculator — Rust + egui

> Built by **56tytt — שי קדוש הנדסת תוכנה אשקלון** 🇮🇱

A pixel-perfect recreation of 3 CASIO scientific calculator models, written in pure **Rust** with **egui**.

---

## 📸 Models

| Model | Style | Features |
|-------|-------|----------|
| **fx-82MS** | 🌸 Pink | S-V.P.A.M, 2nd edition |
| **fx-991ES PLUS** | 🔵 Blue/Grey | Natural VPAM, all functions |
| **fx-CG50** | ⬛ Black | Graphing mode, color display |

---

## ✨ Features

- ✅ Full scientific math engine (tokenizer → recursive-descent parser → evaluator)
- ✅ Trig functions: `sin/cos/tan` + inverses + hyperbolic (`sinh/cosh/tanh`)
- ✅ Logarithms: `log`, `ln`, `10^x`, `e^x`
- ✅ Powers & roots: `x²`, `x³`, `√`, `∛`, `xʸ`, `x⁻¹`
- ✅ Factorial `n!`, combinations `nCr`, permutations `nPr`
- ✅ Angle modes: **Degrees / Radians / Gradians**
- ✅ Memory: `M+`, `M-`, `RCL`, `STO` + variables A–F, X, Y
- ✅ Engineering notation (`ENG`)
- ✅ SHIFT / ALPHA modifier keys
- ✅ History (last 50 calculations)
- ✅ Switch between all 3 models in one click

---

## 🚀 Build & Run

```bash
git clone https://github.com/56tytt/casio-calc
cd casio-calc
cargo run --release
```

**Requirements:**
- Rust 1.75+
- On Linux: `sudo apt install libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev libxkbcommon-dev`

---

## 📁 Project Structure

```
src/
├── main.rs     # Entry point
├── engine.rs   # Math engine: tokenizer + parser + evaluator
├── models.rs   # 3 CASIO model definitions + color palettes
└── ui.rs       # egui rendering
```

---

## 🦀 Why Rust?

- **Zero GC pauses** — smooth UI at all times
- **Memory safe** — no crashes, no undefined behavior
- **Blazing fast** — evaluates expressions in microseconds
- **Single binary** — no runtime dependencies

---

## 🙏 Credits

Built with assistance of **Claude (Anthropic AI)**

## 📜 License

MIT © 56tytt
