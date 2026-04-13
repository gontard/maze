use maze_core::render::{Color, DrawCommand};
use web_sys::CanvasRenderingContext2d;

fn color_to_css(color: &Color) -> &'static str {
    match color {
        Color::White => "#ffffff",
        Color::Black => "#000000",
        Color::Green => "#00cc00",
        Color::Cyan => "#00cccc",
        Color::Yellow => "#cccc00",
        Color::Red => "#cc0000",
    }
}

pub fn paint(ctx: &CanvasRenderingContext2d, cmds: &[DrawCommand], tile_size: u32) {
    let ts = tile_size as f64;

    for cmd in cmds {
        match cmd {
            DrawCommand::Clear => {
                let canvas = ctx.canvas().expect("no canvas");
                ctx.clear_rect(0.0, 0.0, canvas.width() as f64, canvas.height() as f64);
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
                    *x as f64 * ts,
                    *y as f64 * ts,
                    *width as f64 * ts,
                    *height as f64 * ts,
                );
            }
            DrawCommand::DrawText {
                x, y, text, color, ..
            } => {
                ctx.set_fill_style_str(color_to_css(color));
                ctx.set_font(&format!("{}px monospace", ts));
                let _ = ctx.fill_text(text, *x as f64 * ts, (*y as f64 + 1.0) * ts - 2.0);
            }
        }
    }
}
