use maze_core::render::{Color, DrawCommand};
use web_sys::CanvasRenderingContext2d;

fn color_to_css(color: &Color) -> &'static str {
    match color {
        Color::White => "#aaaaaa",
        Color::Black => "#000000",
        Color::Green => "#00cc00",
        Color::Cyan => "#00cccc",
        Color::Yellow => "#cccc00",
        Color::Red => "#cc0000",
    }
}

/// Character cell dimensions matching monospace terminal proportions.
/// Width:height ratio ~0.6:1, like a terminal character cell.
pub const CELL_WIDTH: f64 = 10.0;
pub const CELL_HEIGHT: f64 = 18.0;
pub const FONT_SIZE: f64 = 16.0;

pub fn paint(ctx: &CanvasRenderingContext2d, cmds: &[DrawCommand]) {
    ctx.set_font(&format!("{}px monospace", FONT_SIZE));
    ctx.set_text_baseline("top");

    for cmd in cmds {
        match cmd {
            DrawCommand::Clear => {
                let canvas = ctx.canvas().expect("no canvas");
                ctx.set_fill_style_str("#000000");
                ctx.fill_rect(0.0, 0.0, canvas.width() as f64, canvas.height() as f64);
            }
            DrawCommand::DrawChar { x, y, ch, color } => {
                let px = *x as f64 * CELL_WIDTH;
                let py = *y as f64 * CELL_HEIGHT;
                // Background: black for all cells
                ctx.set_fill_style_str("#000000");
                ctx.fill_rect(px, py, CELL_WIDTH, CELL_HEIGHT);
                // Character
                ctx.set_fill_style_str(color_to_css(color));
                let mut buf = [0u8; 4];
                let s = ch.encode_utf8(&mut buf);
                let _ = ctx.fill_text(s, px, py + 1.0);
            }
            DrawCommand::DrawText { x, y, text, color } => {
                let px = *x as f64 * CELL_WIDTH;
                let py = *y as f64 * CELL_HEIGHT;
                ctx.set_fill_style_str(color_to_css(color));
                let _ = ctx.fill_text(text, px, py + 1.0);
            }
            DrawCommand::FillRect {
                x,
                y,
                width,
                height,
                color,
            } => {
                ctx.set_fill_style_str(color_to_css(color));
                ctx.fill_rect(
                    *x as f64 * CELL_WIDTH,
                    *y as f64 * CELL_HEIGHT,
                    *width as f64 * CELL_WIDTH,
                    *height as f64 * CELL_HEIGHT,
                );
            }
        }
    }
}
